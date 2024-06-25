pub mod data;
pub mod serde_helpers;

use crate::data::holder::RegistryLoadedValues;
use crate::data::world_preset::WorldPreset;
use util::identifier::IntoIdentifier;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::{fs, io};
use thiserror::Error;
use zip::result::ZipError;
use zip::ZipArchive;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum DataPackError {
    #[error("io: {0}")]
    Io(#[from] io::Error),
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("non-utf8 file path")]
    NonUtf8FilePath,
    #[error("zip: {0}")]
    Zip(#[from] ZipError),
}

impl DataPackError {
    pub fn is_not_found(&self) -> bool {
        match self {
            Self::Io(err) => err.kind() == io::ErrorKind::NotFound,
            Self::Zip(err) => matches!(err, ZipError::FileNotFound),
            _ => false,
        }
    }
}

pub type DataPackResult<T> = Result<T, DataPackError>;

pub struct DataPack {
    file_access: DataPackFileAccess,
    pub(crate) registry_values: RegistryLoadedValues,
}

impl DataPack {
    pub fn new(file: impl AsRef<Path>) -> DataPackResult<DataPack> {
        let file = file.as_ref();
        let metadata = fs::symlink_metadata(file)?;
        let file_access = if metadata.is_dir() {
            DataPackFileAccess::Directory(DirectoryDataPack {
                path: file.to_path_buf(),
            })
        } else {
            DataPackFileAccess::Zip(ZipDataPack {
                zip: Mutex::new(ZipArchive::new(File::open(file)?)?),
            })
        };
        Ok(DataPack {
            file_access,
            registry_values: RegistryLoadedValues::default(),
        })
    }

    fn read_json<T: DeserializeOwned>(&self, path: impl AsRef<str>) -> DataPackResult<T> {
        match &self.file_access {
            DataPackFileAccess::Directory(access) => access.read_json(path),
            DataPackFileAccess::Zip(access) => access.read_json(path),
        }
    }

    fn read_bytes(&self, path: impl AsRef<str>) -> DataPackResult<Vec<u8>> {
        match &self.file_access {
            DataPackFileAccess::Directory(access) => access.read_bytes(path),
            DataPackFileAccess::Zip(access) => access.read_bytes(path),
        }
    }

    fn list_files_under(&self, path: impl AsRef<str>) -> DataPackResult<Vec<String>> {
        match &self.file_access {
            DataPackFileAccess::Directory(access) => access.list_files_under(path),
            DataPackFileAccess::Zip(access) => access.list_files_under(path),
        }
    }

    pub fn get_world_preset<'a>(&self, id: impl IntoIdentifier<'a>) -> DataPackResult<WorldPreset> {
        self.read_json(
            id.into_id()
                .to_datapack_path("worldgen/world_preset", "json"),
        )
    }
}

enum DataPackFileAccess {
    Directory(DirectoryDataPack),
    Zip(ZipDataPack),
}

struct DirectoryDataPack {
    path: PathBuf,
}

impl DirectoryDataPack {
    fn read_json<T: DeserializeOwned>(&self, path: impl AsRef<str>) -> DataPackResult<T> {
        let file = File::open(self.path.join(path.as_ref()))?;
        Ok(serde_json::from_reader(file)?)
    }

    fn read_bytes(&self, path: impl AsRef<str>) -> DataPackResult<Vec<u8>> {
        Ok(fs::read(path.as_ref())?)
    }

    fn list_files_under(&self, path: impl AsRef<str>) -> DataPackResult<Vec<String>> {
        fn walk_dir(base: &Path, dir: &Path, result: &mut Vec<String>) -> DataPackResult<()> {
            for file in fs::read_dir(dir)? {
                let file = file?;
                if file.file_type()?.is_dir() {
                    walk_dir(base, &dir.join(file.file_name()), result)?;
                } else {
                    result.push(
                        file.path()
                            .strip_prefix(base)
                            .unwrap()
                            .to_str()
                            .ok_or(DataPackError::NonUtf8FilePath)?
                            .to_owned(),
                    );
                }
            }
            Ok(())
        }

        let path = path.as_ref();
        assert!(path.ends_with('/'));
        let path = Path::new(path);
        let mut result = Vec::new();
        walk_dir(path, path, &mut result)?;
        Ok(result)
    }
}

struct ZipDataPack {
    zip: Mutex<ZipArchive<File>>,
}

impl ZipDataPack {
    fn read_json<T: DeserializeOwned>(&self, path: impl AsRef<str>) -> DataPackResult<T> {
        let mut zip = self.zip.lock().unwrap();
        let file = zip.by_name(path.as_ref())?;
        Ok(serde_json::from_reader(file)?)
    }

    fn read_bytes(&self, path: impl AsRef<str>) -> DataPackResult<Vec<u8>> {
        let mut zip = self.zip.lock().unwrap();
        let mut file = zip.by_name(path.as_ref())?;
        let mut result = Vec::new();
        file.read_to_end(&mut result)?;
        Ok(result)
    }

    fn list_files_under(&self, path: impl AsRef<str>) -> DataPackResult<Vec<String>> {
        let path = path.as_ref();
        assert!(path.ends_with('/'));
        let zip = self.zip.lock().unwrap();
        Ok(zip
            .file_names()
            .filter(|file| file.starts_with(path))
            .filter(|file| !file.ends_with('/'))
            .map(|file| file.to_owned())
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use crate::data::biome::Biome;
    use crate::data::biome_source::{BiomeSource, MultiNoiseBiomeSource};
    use crate::data::world_preset::ChunkGenerator;
    use crate::DataPack;

    #[test]
    fn test_read_datapack() {
        let datapack = DataPack::new("server.jar").unwrap();
        let default = datapack.get_world_preset("normal").unwrap();
        for (dim_id, dim) in &default.dimensions {
            let ChunkGenerator::Noise(generator) = &dim.generator else {
                panic!("found non-noise generator")
            };
            match &generator.biome_source {
                BiomeSource::MultiNoise(MultiNoiseBiomeSource::Preset(preset)) => {
                    let preset = preset.resolve(&datapack).unwrap();
                }
                BiomeSource::TheEnd(_) => {}
                _ => panic!("found wrong biome source"),
            }
            println!("getting gen settings for {dim_id}");
            match generator.settings.resolve(&datapack) {
                Ok(gen_settings) => {/* println!("{dim_id} generator settings: {gen_settings:#?}") */}
                Err(err) => panic!("{dim_id} error: {err}"),
            }
        }

        for biome_path in datapack.list_files_under("data/minecraft/worldgen/biome/").unwrap() {
            println!("reading biome {biome_path}");
            datapack.read_json::<Biome>(biome_path).unwrap();
        }
    }
}

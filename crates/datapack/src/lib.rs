mod data;
pub mod identifier;
mod serde_helpers;

use crate::data::world_preset::WorldPreset;
use crate::identifier::IntoIdentifier;
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
    inner: DataPackInner,
}

impl DataPack {
    pub fn new(file: impl AsRef<Path>) -> DataPackResult<DataPack> {
        let file = file.as_ref();
        let metadata = fs::symlink_metadata(file)?;
        if metadata.is_dir() {
            Ok(DataPack {
                inner: DataPackInner::Directory(DirectoryDataPack {
                    path: file.to_path_buf(),
                }),
            })
        } else {
            Ok(DataPack {
                inner: DataPackInner::Zip(ZipDataPack {
                    zip: Mutex::new(ZipArchive::new(File::open(file)?)?),
                }),
            })
        }
    }

    fn read_json<T: DeserializeOwned>(&self, path: impl AsRef<str>) -> DataPackResult<T> {
        match &self.inner {
            DataPackInner::Directory(pack) => pack.read_json(path),
            DataPackInner::Zip(pack) => pack.read_json(path),
        }
    }

    fn read_bytes(&self, path: impl AsRef<str>) -> DataPackResult<Vec<u8>> {
        match &self.inner {
            DataPackInner::Directory(pack) => pack.read_bytes(path),
            DataPackInner::Zip(pack) => pack.read_bytes(path),
        }
    }

    fn list_files_under(&self, path: impl AsRef<str>) -> DataPackResult<Vec<String>> {
        match &self.inner {
            DataPackInner::Directory(pack) => pack.list_files_under(path),
            DataPackInner::Zip(pack) => pack.list_files_under(path),
        }
    }

    pub fn get_world_preset<'a>(&self, id: impl IntoIdentifier<'a>) -> DataPackResult<WorldPreset> {
        self.read_json(
            id.into_id()
                .to_datapack_path("worldgen/world_preset", "json"),
        )
    }
}

enum DataPackInner {
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

        let mut result = Vec::new();
        let path = Path::new(path.as_ref());
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
        let zip = self.zip.lock().unwrap();
        let path = path.as_ref();
        Ok(zip
            .file_names()
            .filter(|file| file.starts_with(path))
            .map(|file| file.to_owned())
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use crate::DataPack;

    #[test]
    fn test_read_datapack() {
        let datapack = DataPack::new("server.jar").unwrap();
        let default = datapack.get_world_preset("normal").unwrap();
    }
}

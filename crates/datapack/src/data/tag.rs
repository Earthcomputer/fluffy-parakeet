use crate::built_in_registries::{Block, Fluid};
use crate::data::biome::Biome;
use crate::data::carvers::ConfiguredWorldCarver;
use crate::data::holder::Holder;
use crate::data::structure::set::StructureSet;
use crate::{DataPack, DataPackError, DataPackResult};
use ahash::{AHashMap, AHashSet};
use datapack_macros::UntaggedDeserialize;
use serde::de::Unexpected;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::convert::Infallible;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use util::add_only_map::AddOnlyMultiMap;
use util::identifier::{Identifier, IdentifierBuf};

mod sealed {
    pub trait Sealed {}
}

#[allow(private_interfaces)]
pub trait TaggedRegistry: sealed::Sealed {
    fn get_registry_tags(tags: &RegistryTags) -> &AddOnlyMultiMap<IdentifierBuf, IdentifierBuf>;
    fn load_tag(datapack: &DataPack, id: &Identifier) -> DataPackResult<TagFile>;
}

macro_rules! tagged_registries {
    ($($id:ident: $type:ty[$folder:literal];)*) => {
        $(
            impl sealed::Sealed for $type {}

            #[allow(private_interfaces)]
            impl TaggedRegistry for $type {
                fn get_registry_tags(tags: &RegistryTags) -> &AddOnlyMultiMap<IdentifierBuf, IdentifierBuf> {
                    &tags.$id
                }

                fn load_tag(datapack: &DataPack, id: &Identifier) -> DataPackResult<TagFile> {
                    let (namespace, path) = id.namespace_and_path();
                    datapack.read_json(format!("data/{}/tags/{}/{}.json", namespace, $folder, path))
                }
            }
        )*

        #[derive(Debug, Default)]
        pub(crate) struct RegistryTags {
            $(
                $id: AddOnlyMultiMap<IdentifierBuf, IdentifierBuf>,
            )*
        }
    };
}

tagged_registries! {
    biome: Biome["worldgen/biome"];
    block: Block["block"];
    configured_carver: ConfiguredWorldCarver["worldgen/configured_carver"];
    fluid: Fluid["fluid"];
    structure_set: StructureSet["worldgen/structure_set"];
}

#[derive(Debug)]
pub struct HolderSet<T> {
    pub values: Vec<TagOrId>,
    _phantom: PhantomData<T>,
}

impl<T> Default for HolderSet<T> {
    fn default() -> Self {
        HolderSet {
            values: Vec::default(),
            _phantom: PhantomData,
        }
    }
}

impl<T> HolderSet<T>
where
    T: TaggedRegistry,
{
    pub fn resolve_tag<'a>(
        datapack: &'a DataPack,
        id: &Identifier,
    ) -> DataPackResult<&'a [IdentifierBuf]> {
        let registry_tags = T::get_registry_tags(&datapack.registry_tags);
        if let Some(loaded_tag) = registry_tags.get(id) {
            // fast path: tag is already loaded
            Ok(loaded_tag)
        } else {
            let mut tags_to_add = AHashMap::new();
            let load_error =
                load_tag_recursive::<T>(datapack, id, &mut AHashSet::new(), &mut tags_to_add).err();
            // despite the potential error, there may be some successfully loaded tags to add
            for (tag_id, tag_values) in tags_to_add {
                registry_tags
                    .get_or_try_insert(tag_id, || Ok::<_, Infallible>(tag_values))
                    .unwrap();
            }
            if let Some(load_error) = load_error {
                return Err(load_error);
            }
            Ok(registry_tags.get(id).unwrap())
        }
    }

    pub fn flatten<'a>(&'a self, datapack: &'a DataPack) -> DataPackResult<Vec<&'a Identifier>> {
        let mut added_values = AHashSet::<&Identifier>::new();
        let mut values = Vec::<&Identifier>::new();

        for value in &self.values {
            match value {
                TagOrId::Id(value) => {
                    if added_values.insert(&value) {
                        values.push(&value);
                    }
                }
                TagOrId::Tag(tag) => {
                    for value in Self::resolve_tag(datapack, tag)? {
                        if added_values.insert(&value) {
                            values.push(&value);
                        }
                    }
                }
            }
        }

        Ok(values)
    }
}

impl<'de, T> Deserialize<'de> for HolderSet<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(UntaggedDeserialize)]
        enum Surrogate {
            Inline(TagOrId),
            List(Vec<TagOrId>),
        }
        match Surrogate::deserialize(deserializer)? {
            Surrogate::Inline(value) => Ok(HolderSet {
                values: vec![value],
                _phantom: PhantomData,
            }),
            Surrogate::List(values) => Ok(HolderSet {
                values,
                _phantom: PhantomData,
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TagOrId {
    Id(IdentifierBuf),
    Tag(IdentifierBuf),
}

impl Display for TagOrId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TagOrId::Id(value) => Display::fmt(value, f),
            TagOrId::Tag(tag) => write!(f, "#{tag}"),
        }
    }
}

impl<'de> Deserialize<'de> for TagOrId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let unparsed = String::deserialize(deserializer)?;
        let result = if let Some(unparsed_tag) = unparsed.strip_prefix('#') {
            IdentifierBuf::new(unparsed_tag).map(TagOrId::Tag)
        } else {
            IdentifierBuf::new(&unparsed).map(TagOrId::Id)
        };
        result.map_err(|_| {
            serde::de::Error::invalid_value(Unexpected::Str(&unparsed), &"identifier or tag")
        })
    }
}

impl Serialize for TagOrId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            TagOrId::Id(value) => value.serialize(serializer),
            TagOrId::Tag(tag) => format!("#{tag}").serialize(serializer),
        }
    }
}

#[derive(Debug)]
pub struct HolderValueSet<T> {
    pub values: Vec<TagOrHolder<T>>,
}

impl<T> Default for HolderValueSet<T> {
    fn default() -> Self {
        Self {
            values: Vec::default(),
        }
    }
}

impl<'de, T> Deserialize<'de> for HolderValueSet<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(UntaggedDeserialize)]
        enum Surrogate<T> {
            Inline(TagOrHolder<T>),
            List(Vec<TagOrHolder<T>>),
        }
        match Surrogate::deserialize(deserializer)? {
            Surrogate::Inline(value) => Ok(HolderValueSet {
                values: vec![value],
            }),
            Surrogate::List(values) => Ok(HolderValueSet { values }),
        }
    }
}

#[derive(Debug)]
pub enum TagOrHolder<T> {
    Holder(Holder<T>),
    Tag(IdentifierBuf),
}

impl<'de, T> Deserialize<'de> for TagOrHolder<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(UntaggedDeserialize)]
        enum Surrogate<T> {
            TagOrId(TagOrId),
            Direct(T),
        }
        match Surrogate::deserialize(deserializer)? {
            Surrogate::TagOrId(TagOrId::Id(id)) => Ok(TagOrHolder::Holder(Holder::Reference(id))),
            Surrogate::TagOrId(TagOrId::Tag(tag)) => Ok(TagOrHolder::Tag(tag)),
            Surrogate::Direct(value) => Ok(TagOrHolder::Holder(Holder::Direct(value))),
        }
    }
}

#[derive(Debug, Deserialize)]
struct TagFile {
    values: Vec<TagEntry>,
    #[serde(default)]
    replace: bool,
}

#[derive(Debug)]
struct TagEntry {
    value: TagOrId,
    required: bool,
}

impl<'de> Deserialize<'de> for TagEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct TagEntrySurrogate {
            value: TagOrId,
            #[serde(default = "default_required")]
            required: bool,
        }
        fn default_required() -> bool {
            true
        }

        #[derive(UntaggedDeserialize)]
        enum Surrogate {
            Value(TagOrId),
            Object(TagEntrySurrogate),
        }

        match Surrogate::deserialize(deserializer)? {
            Surrogate::Value(value) => Ok(TagEntry {
                value,
                required: true,
            }),
            Surrogate::Object(TagEntrySurrogate { value, required }) => {
                Ok(TagEntry { value, required })
            }
        }
    }
}

pub fn deserialize_hashed_tag<'de, D>(deserializer: D) -> Result<IdentifierBuf, D::Error>
where
    D: Deserializer<'de>,
{
    let unparsed = String::deserialize(deserializer)?;
    let Some(unparsed_tag) = unparsed.strip_prefix('#') else {
        return Err(serde::de::Error::invalid_value(
            Unexpected::Str(&unparsed),
            &"tag starting with #",
        ));
    };
    IdentifierBuf::new(unparsed_tag).map_err(|_| {
        serde::de::Error::invalid_value(Unexpected::Str(&unparsed), &"tag starting with #")
    })
}

fn load_tag_recursive<T>(
    datapack: &DataPack,
    id: &Identifier,
    currently_loading_tags: &mut AHashSet<IdentifierBuf>,
    tags_to_add: &mut AHashMap<IdentifierBuf, Vec<IdentifierBuf>>,
) -> DataPackResult<Vec<IdentifierBuf>>
where
    T: TaggedRegistry,
{
    let tag_file = T::load_tag(datapack, id)?;
    let mut added_values = AHashSet::new();
    let mut values = Vec::new();

    let mut add_value = |value: IdentifierBuf| {
        if added_values.insert(value.clone()) {
            values.push(value);
        }
    };

    for entry in tag_file.values {
        match entry.value {
            TagOrId::Id(value) => add_value(value),
            TagOrId::Tag(tag) => {
                if let Some(loaded_values) = T::get_registry_tags(&datapack.registry_tags).get(&tag)
                {
                    // fast path: tag has already been loaded in the datapack
                    for value in loaded_values {
                        add_value(value.clone());
                    }
                } else if let Some(loaded_values) = tags_to_add.get(&tag) {
                    // second fast path: tag has already been loaded in this recursive load
                    for value in loaded_values {
                        add_value(value.clone());
                    }
                } else {
                    if !currently_loading_tags.insert(tag.clone()) {
                        return Err(DataPackError::RecursiveTag);
                    }
                    let inner_load_result = load_tag_recursive::<T>(
                        datapack,
                        &tag,
                        currently_loading_tags,
                        tags_to_add,
                    );
                    currently_loading_tags.remove(&tag);
                    match inner_load_result {
                        Ok(loaded_values) => {
                            for value in loaded_values {
                                add_value(value);
                            }
                        }
                        Err(err) => {
                            if entry.required || !err.is_not_found() {
                                return Err(err);
                            }
                        }
                    }
                }
            }
        }
    }

    tags_to_add.insert(id.to_owned(), values.clone());

    Ok(values)
}

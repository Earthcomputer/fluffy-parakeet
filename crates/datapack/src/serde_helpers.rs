use crate::data::biome::Biome;
use crate::data::holder::Holder;
use util::identifier::IdentifierBuf;
use datapack_macros::UntaggedDeserialize;
use num::FromPrimitive;
use serde::de::{Expected, Unexpected};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut, Div};
use glam::IVec3;

/// Converts deserialize errors into the value provided by `Def`
pub struct DefaultOnError<T, Def = DefaultValueProvider<T>>(T, PhantomData<Def>);

impl<T, Def> Debug for DefaultOnError<T, Def>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("DefaultOnError").field(&self.0).finish()
    }
}

impl<T, Def> Default for DefaultOnError<T, Def>
where
    Def: ValueProvider<T>,
{
    fn default() -> Self {
        Self::from(Def::provide())
    }
}

impl<'de, T, Def> Deserialize<'de> for DefaultOnError<T, Def>
where
    T: Deserialize<'de>,
    Def: ValueProvider<T>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self::from(
            Deserialize::deserialize(deserializer).unwrap_or_else(|_| Def::provide()),
        ))
    }
}

impl<T, Def> Serialize for DefaultOnError<T, Def>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<T, Def> From<T> for DefaultOnError<T, Def> {
    fn from(value: T) -> Self {
        Self(value, PhantomData)
    }
}

impl<T, Def> Deref for DefaultOnError<T, Def> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T, Def> DerefMut for DefaultOnError<T, Def> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

/// If a sequence is of length 1, inlines that value
#[derive(Debug)]
pub struct InlineVec<T>(Vec<T>);

impl<T> Default for InlineVec<T> {
    fn default() -> Self {
        InlineVec(Vec::default())
    }
}

impl<'de, T> Deserialize<'de> for InlineVec<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(UntaggedDeserialize)]
        enum PossiblyInlinedSingleton<T> {
            Vec(Vec<T>),
            Inline(T),
        }

        let possibly_inlined = PossiblyInlinedSingleton::deserialize(deserializer)?;
        match possibly_inlined {
            PossiblyInlinedSingleton::Vec(vec) => Ok(Self(vec)),
            PossiblyInlinedSingleton::Inline(val) => Ok(Self(vec![val])),
        }
    }
}

impl<T> Serialize for InlineVec<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.0.len() == 1 {
            self.0[0].serialize(serializer)
        } else {
            self.0.serialize(serializer)
        }
    }
}

impl<T> From<Vec<T>> for InlineVec<T> {
    fn from(value: Vec<T>) -> Self {
        Self(value)
    }
}

impl<T> Deref for InlineVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Vec<T> {
        &self.0
    }
}

impl<T> DerefMut for InlineVec<T> {
    fn deref_mut(&mut self) -> &mut Vec<T> {
        &mut self.0
    }
}

#[derive(Debug)]
pub struct NonEmptyVec<T>(Vec<T>);

impl<'de, T> Deserialize<'de> for NonEmptyVec<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let vec = Vec::<T>::deserialize(deserializer)?;
        if vec.is_empty() {
            return Err(serde::de::Error::invalid_length(0, &"non-empty vec"));
        }
        Ok(Self(vec))
    }
}

impl<T> Serialize for NonEmptyVec<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.0.is_empty() {
            return Err(serde::ser::Error::custom("empty vec"));
        }
        self.0.serialize(serializer)
    }
}

impl<T> From<Vec<T>> for NonEmptyVec<T> {
    fn from(value: Vec<T>) -> Self {
        Self(value)
    }
}

impl<T> Deref for NonEmptyVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Vec<T> {
        &self.0
    }
}

impl<T> DerefMut for NonEmptyVec<T> {
    fn deref_mut(&mut self) -> &mut Vec<T> {
        &mut self.0
    }
}

/// Checks that the value is in range on deserialization, clamps it on serialization
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ranged<T, const MIN: i64, const MAX: i64, const SCALE: u64 = 1>(T);

pub type RangedNonNegativeU32 = Ranged<u32, 0, { i32::MAX as i64 }>;
pub type RangedPositiveU32 = Ranged<u32, 1, { i32::MAX as i64 }>;

impl<'de, T, const MIN: i64, const MAX: i64, const SCALE: u64> Deserialize<'de>
    for Ranged<T, MIN, MAX, SCALE>
where
    T: Deserialize<'de> + Ord + FromPrimitive + Div<Output = T> + Debug,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let result = T::deserialize(deserializer)?;
        let min = T::from_i64(MIN).unwrap() / T::from_u64(SCALE).unwrap();
        let max = T::from_i64(MAX).unwrap() / T::from_u64(SCALE).unwrap();
        if result < min {
            return Err(serde::de::Error::invalid_value(
                Unexpected::Other("value out of range"),
                &ExpectedAtLeast(min),
            ));
        }
        if result > max {
            return Err(serde::de::Error::invalid_value(
                Unexpected::Other("value out of range"),
                &ExpectedAtMost(max),
            ));
        }
        Ok(Ranged(result))
    }
}

impl<T, const MIN: i64, const MAX: i64, const SCALE: u64> Serialize for Ranged<T, MIN, MAX, SCALE>
where
    T: Serialize + Ord + FromPrimitive + Div<Output = T>,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let min = T::from_i64(MIN).unwrap() / T::from_u64(SCALE).unwrap();
        let max = T::from_i64(MAX).unwrap() / T::from_u64(SCALE).unwrap();
        let value = (&self.0).clamp(&min, &max);
        value.serialize(serializer)
    }
}

impl<T, const MIN: i64, const MAX: i64, const SCALE: u64> From<T> for Ranged<T, MIN, MAX, SCALE> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T, const MIN: i64, const MAX: i64, const SCALE: u64> Deref for Ranged<T, MIN, MAX, SCALE> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T, const MIN: i64, const MAX: i64, const SCALE: u64> DerefMut for Ranged<T, MIN, MAX, SCALE> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

/// Checks that an [`IVec3`] is in range on deserialization, clamps it on serialization
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct RangedIVec3<const MIN_XZ: i32, const MAX_XZ: i32, const MIN_Y: i32, const MAX_Y: i32>(IVec3);

impl<'de, const MIN_XZ: i32, const MAX_XZ: i32, const MIN_Y: i32, const MAX_Y: i32> Deserialize<'de> for RangedIVec3<MIN_XZ, MAX_XZ, MIN_Y, MAX_Y> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        let vec = IVec3::deserialize(deserializer)?;
        if vec.x < MIN_XZ {
            return Err(serde::de::Error::invalid_value(
                Unexpected::Other("x out of range"),
                &ExpectedAtLeast(MIN_XZ),
            ));
        }
        if vec.x > MAX_XZ {
            return Err(serde::de::Error::invalid_value(
                Unexpected::Other("x out of range"),
                &ExpectedAtMost(MAX_XZ),
            ));
        }
        if vec.z < MIN_XZ {
            return Err(serde::de::Error::invalid_value(
                Unexpected::Other("z out of range"),
                &ExpectedAtLeast(MIN_XZ),
            ));
        }
        if vec.z > MAX_XZ {
            return Err(serde::de::Error::invalid_value(
                Unexpected::Other("z out of range"),
                &ExpectedAtMost(MAX_XZ),
            ));
        }
        if vec.y < MIN_Y {
            return Err(serde::de::Error::invalid_value(
                Unexpected::Other("y out of range"),
                &ExpectedAtLeast(MIN_Y),
            ));
        }
        if vec.y > MAX_Y {
            return Err(serde::de::Error::invalid_value(
                Unexpected::Other("y out of range"),
                &ExpectedAtMost(MAX_Y),
            ));
        }
        Ok(Self(vec))
    }
}

impl<const MIN_XZ: i32, const MAX_XZ: i32, const MIN_Y: i32, const MAX_Y: i32> Serialize for RangedIVec3<MIN_XZ, MAX_XZ, MIN_Y, MAX_Y> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        IVec3::new(self.0.x.clamp(MIN_XZ, MAX_XZ), self.0.y.clamp(MIN_Y, MAX_Y), self.0.z.clamp(MIN_XZ, MAX_XZ)).serialize(serializer)
    }
}

impl<const MIN_XZ: i32, const MAX_XZ: i32, const MIN_Y: i32, const MAX_Y: i32> From<IVec3> for RangedIVec3<MIN_XZ, MAX_XZ, MIN_Y, MAX_Y> {
    fn from(value: IVec3) -> Self {
        Self(value)
    }
}

impl<const MIN_XZ: i32, const MAX_XZ: i32, const MIN_Y: i32, const MAX_Y: i32> Deref for RangedIVec3<MIN_XZ, MAX_XZ, MIN_Y, MAX_Y> {
    type Target = IVec3;

    fn deref(&self) -> &IVec3 {
        &self.0
    }
}

impl<const MIN_XZ: i32, const MAX_XZ: i32, const MIN_Y: i32, const MAX_Y: i32> DerefMut for RangedIVec3<MIN_XZ, MAX_XZ, MIN_Y, MAX_Y> {
    fn deref_mut(&mut self) -> &mut IVec3 {
        &mut self.0
    }
}

pub trait ValueProvider<T> {
    fn provide() -> T;
}

pub struct DefaultValueProvider<T>(PhantomData<T>);

impl<T> ValueProvider<T> for DefaultValueProvider<T>
where
    T: Default,
{
    fn provide() -> T {
        T::default()
    }
}

pub struct DefaultToAir;

impl ValueProvider<IdentifierBuf> for DefaultToAir {
    fn provide() -> IdentifierBuf {
        IdentifierBuf::new("air").unwrap()
    }
}

pub struct DefaultToPlains;

impl ValueProvider<Holder<Biome>> for DefaultToPlains {
    fn provide() -> Holder<Biome> {
        Holder::Reference(IdentifierBuf::new("plains").unwrap())
    }
}

struct ExpectedAtLeast<T>(T);

impl<T> Expected for ExpectedAtLeast<T>
where
    T: Debug,
{
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "at least {:?}", self.0)
    }
}

struct ExpectedAtMost<T>(T);

impl<T> Expected for ExpectedAtMost<T>
where
    T: Debug,
{
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "at most {:?}", self.0)
    }
}

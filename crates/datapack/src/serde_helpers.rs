use crate::data::biome::Biome;
use crate::data::holder::Holder;
use glam::IVec3;

use num::FromPrimitive;
use serde::de::Unexpected;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut, Div};
use util::identifier::IdentifierBuf;
use util::ranged::{value_too_big_error, value_too_small_error, Ranged, RangedValue};

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

/// Checks that an [`IVec3`] is in range on deserialization, clamps it on serialization
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct RangedIVec3<const MIN_XZ: i32, const MAX_XZ: i32, const MIN_Y: i32, const MAX_Y: i32>(
    IVec3,
);

impl<'de, const MIN_XZ: i32, const MAX_XZ: i32, const MIN_Y: i32, const MAX_Y: i32> Deserialize<'de>
    for RangedIVec3<MIN_XZ, MAX_XZ, MIN_Y, MAX_Y>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let vec = IVec3::deserialize(deserializer)?;
        if vec.x < MIN_XZ {
            return Err(value_too_small_error(
                Unexpected::Other("vector out of range"),
                MIN_XZ,
            ));
        }
        if vec.x > MAX_XZ {
            return Err(value_too_big_error(
                Unexpected::Other("vector out of range"),
                MAX_XZ,
            ));
        }
        if vec.z < MIN_XZ {
            return Err(value_too_small_error(
                Unexpected::Other("vector out of range"),
                MIN_XZ,
            ));
        }
        if vec.z > MAX_XZ {
            return Err(value_too_big_error(
                Unexpected::Other("vector out of range"),
                MAX_XZ,
            ));
        }
        if vec.y < MIN_Y {
            return Err(value_too_small_error(
                Unexpected::Other("vector out of range"),
                MIN_Y,
            ));
        }
        if vec.y > MAX_Y {
            return Err(value_too_big_error(
                Unexpected::Other("vector out of range"),
                MAX_Y,
            ));
        }
        Ok(Self(vec))
    }
}

impl<const MIN_XZ: i32, const MAX_XZ: i32, const MIN_Y: i32, const MAX_Y: i32> Serialize
    for RangedIVec3<MIN_XZ, MAX_XZ, MIN_Y, MAX_Y>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        IVec3::new(
            self.0.x.clamp(MIN_XZ, MAX_XZ),
            self.0.y.clamp(MIN_Y, MAX_Y),
            self.0.z.clamp(MIN_XZ, MAX_XZ),
        )
        .serialize(serializer)
    }
}

impl<const MIN_XZ: i32, const MAX_XZ: i32, const MIN_Y: i32, const MAX_Y: i32> From<IVec3>
    for RangedIVec3<MIN_XZ, MAX_XZ, MIN_Y, MAX_Y>
{
    fn from(value: IVec3) -> Self {
        Self(value)
    }
}

impl<const MIN_XZ: i32, const MAX_XZ: i32, const MIN_Y: i32, const MAX_Y: i32> Deref
    for RangedIVec3<MIN_XZ, MAX_XZ, MIN_Y, MAX_Y>
{
    type Target = IVec3;

    fn deref(&self) -> &IVec3 {
        &self.0
    }
}

impl<const MIN_XZ: i32, const MAX_XZ: i32, const MIN_Y: i32, const MAX_Y: i32> DerefMut
    for RangedIVec3<MIN_XZ, MAX_XZ, MIN_Y, MAX_Y>
{
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

pub struct DefaultToNum<const N: i64, const SCALE: u64 = 1>;

impl<T, const N: i64, const SCALE: u64> ValueProvider<T> for DefaultToNum<N, SCALE>
where
    T: FromPrimitive + Div<Output = T>,
{
    fn provide() -> T {
        T::from_i64(N).unwrap() / T::from_u64(SCALE).unwrap()
    }
}

pub struct DefaultToRanged<const N: i64, const SCALE: u64 = 1>;

impl<
        T,
        const N: i64,
        const VALUE_SCALE: u64,
        const MIN: i64,
        const MAX: i64,
        const RANGED_SCALE: u64,
        const MIN_INCLUSIVE: bool,
        const MAX_INCLUSIVE: bool,
        const HAS_MIN: bool,
        const HAS_MAX: bool,
    >
    ValueProvider<Ranged<T, MIN, MAX, RANGED_SCALE, MIN_INCLUSIVE, MAX_INCLUSIVE, HAS_MIN, HAS_MAX>>
    for DefaultToRanged<N, VALUE_SCALE>
where
    T: RangedValue,
{
    fn provide() -> Ranged<T, MIN, MAX, RANGED_SCALE, MIN_INCLUSIVE, MAX_INCLUSIVE, HAS_MIN, HAS_MAX>
    {
        Ranged::new(T::from_i64(N).unwrap() / T::from_u64(VALUE_SCALE).unwrap()).unwrap()
    }
}

pub struct DefaultToTrue;

impl ValueProvider<bool> for DefaultToTrue {
    fn provide() -> bool {
        true
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

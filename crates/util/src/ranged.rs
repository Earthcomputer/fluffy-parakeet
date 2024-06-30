use num::FromPrimitive;
use serde::de::{Expected, Unexpected};
use serde::{Deserialize, Deserializer, Serialize};
use std::borrow::Borrow;
use std::fmt::{Debug, Display, Formatter};
use std::hint::unreachable_unchecked;
use std::ops::{Deref, Div};
use thiserror::Error;

/// A type which always holds a value between `MIN / SCALE` and `MAX / SCALE`.
// Invariants:
// - if HAS_MIN is true, then the value must be greater than MIN / SCALE, or, if MIN_INCLUSIVE is true, equal to MIN / SCALE.
// - if HAS_MAX is true, then the value must be less than MAX / SCALE, or, if MAX_INCLUSIVE is true, equal to MAX / SCALE.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(transparent)]
pub struct Ranged<
    T: RangedValue,
    const MIN: i64,
    const MAX: i64,
    const SCALE: u64 = 1,
    const MIN_INCLUSIVE: bool = true,
    const MAX_INCLUSIVE: bool = true,
    const HAS_MIN: bool = true,
    const HAS_MAX: bool = true,
>(T);

pub type NonNegativeI8 = Ranged<u8, 0, { i8::MAX as i64 }>;
pub type PositiveI8 = Ranged<u8, 1, { i8::MAX as i64 }>;
pub type NonNegativeI16 = Ranged<u16, 0, { i16::MAX as i64 }>;
pub type PositiveI16 = Ranged<u16, 1, { i16::MAX as i64 }>;
pub type NonNegativeI32 = Ranged<u32, 0, { i32::MAX as i64 }>;
pub type PositiveI32 = Ranged<u32, 1, { i32::MAX as i64 }>;
pub type NonNegativeI64 = Ranged<u64, 0, { i64::MAX }>;
pub type PositiveI64 = Ranged<u64, 1, { i64::MAX }>;
pub type NonNegativeF32 = Ranged<f32, 0, 0, 1, true, true, true, false>;
pub type PositiveF32 = Ranged<f32, 0, 0, 1, false, true, true, false>;
pub type NonNegativeF64 = Ranged<f64, 0, 0, 1, true, true, true, false>;
pub type PositiveF64 = Ranged<f64, 0, 0, 1, false, true, true, false>;

#[derive(Debug, Error)]
pub struct OutOfRangeError;

impl Display for OutOfRangeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("value is out of range")
    }
}

mod sealed {
    // these types are known to implement their comparison operators as expected,
    // so we can uphold the invariant of the Ranged type.
    pub trait TrustedValue {}
}

pub trait RangedValue:
    sealed::TrustedValue + Debug + Default + Copy + FromPrimitive + PartialOrd + Div<Output = Self>
{
    fn into_unexpected(self) -> Unexpected<'static>;
}

macro_rules! ranged_values {
    ($($type:ty, $unexpected:ident;)*) => {
        $(
            impl sealed::TrustedValue for $type {}
            impl RangedValue for $type {
                fn into_unexpected(self) -> Unexpected<'static> {
                    Unexpected::$unexpected(self as _)
                }
            }
        )*
    };
}
ranged_values!(
    i8, Signed;
    u8, Unsigned;
    i16, Signed;
    u16, Unsigned;
    i32, Signed;
    u32, Unsigned;
    i64, Signed;
    u64, Unsigned;
    f32, Float;
    f64, Float;
);

impl<
        T: RangedValue,
        const MIN: i64,
        const MAX: i64,
        const SCALE: u64,
        const MIN_INCLUSIVE: bool,
        const MAX_INCLUSIVE: bool,
        const HAS_MIN: bool,
        const HAS_MAX: bool,
    > Ranged<T, MIN, MAX, SCALE, MIN_INCLUSIVE, MAX_INCLUSIVE, HAS_MIN, HAS_MAX>
{
    pub fn min() -> T {
        T::from_i64(MIN).unwrap() / T::from_u64(SCALE).unwrap()
    }

    pub fn max() -> T {
        T::from_i64(MAX).unwrap() / T::from_u64(SCALE).unwrap()
    }

    pub fn new(value: T) -> Result<Self, OutOfRangeError> {
        // check for NaN
        if value != value {
            return Err(OutOfRangeError);
        }

        if HAS_MIN {
            if MIN_INCLUSIVE {
                if value < Self::min() {
                    return Err(OutOfRangeError);
                }
            } else {
                if value <= Self::min() {
                    return Err(OutOfRangeError);
                }
            }
        }

        if HAS_MAX {
            if MAX_INCLUSIVE {
                if value > Self::max() {
                    return Err(OutOfRangeError);
                }
            } else {
                if value >= Self::max() {
                    return Err(OutOfRangeError);
                }
            }
        }

        Ok(Self(value))
    }

    /// # Safety
    /// The value must be between `MIN / SCALE` and `MAX / SCALE`.
    pub const unsafe fn new_unchecked(value: T) -> Self {
        Self(value)
    }

    pub fn value(&self) -> T {
        **self
    }
}

impl<
        T: RangedValue,
        const MIN: i64,
        const MAX: i64,
        const SCALE: u64,
        const MIN_INCLUSIVE: bool,
        const MAX_INCLUSIVE: bool,
        const HAS_MIN: bool,
        const HAS_MAX: bool,
    > Default for Ranged<T, MIN, MAX, SCALE, MIN_INCLUSIVE, MAX_INCLUSIVE, HAS_MIN, HAS_MAX>
{
    fn default() -> Self {
        Self::new(T::default()).expect("default T is an invalid value")
    }
}

impl<
        T: RangedValue,
        const MIN: i64,
        const MAX: i64,
        const SCALE: u64,
        const MIN_INCLUSIVE: bool,
        const MAX_INCLUSIVE: bool,
        const HAS_MIN: bool,
        const HAS_MAX: bool,
    > Borrow<T> for Ranged<T, MIN, MAX, SCALE, MIN_INCLUSIVE, MAX_INCLUSIVE, HAS_MIN, HAS_MAX>
{
    fn borrow(&self) -> &T {
        &**self
    }
}

impl<
        T: RangedValue,
        const MIN: i64,
        const MAX: i64,
        const SCALE: u64,
        const MIN_INCLUSIVE: bool,
        const MAX_INCLUSIVE: bool,
        const HAS_MIN: bool,
        const HAS_MAX: bool,
    > Deref for Ranged<T, MIN, MAX, SCALE, MIN_INCLUSIVE, MAX_INCLUSIVE, HAS_MIN, HAS_MAX>
{
    type Target = T;

    fn deref(&self) -> &T {
        if Self::new(self.0).is_err() {
            // SAFETY: this condition is impossible due to the struct invariant
            unsafe {
                unreachable_unchecked();
            }
        }

        &self.0
    }
}

impl<
        'de,
        T: RangedValue,
        const MIN: i64,
        const MAX: i64,
        const SCALE: u64,
        const MIN_INCLUSIVE: bool,
        const MAX_INCLUSIVE: bool,
        const HAS_MIN: bool,
        const HAS_MAX: bool,
    > Deserialize<'de>
    for Ranged<T, MIN, MAX, SCALE, MIN_INCLUSIVE, MAX_INCLUSIVE, HAS_MIN, HAS_MAX>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = T::deserialize(deserializer)?;

        if value != value {
            return Err(serde::de::Error::invalid_value(
                value.into_unexpected(),
                &"a non-nan float",
            ));
        }

        if HAS_MIN {
            if MIN_INCLUSIVE {
                if value < Self::min() {
                    return Err(value_too_small_error(value.into_unexpected(), Self::min()));
                }
            } else {
                if value <= Self::min() {
                    struct ExpectedGreaterThan<T>(T);
                    impl<T> Expected for ExpectedGreaterThan<T>
                    where
                        T: Debug,
                    {
                        fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
                            write!(formatter, "greater than {:?}", self.0)
                        }
                    }
                    return Err(serde::de::Error::invalid_value(
                        value.into_unexpected(),
                        &ExpectedGreaterThan(Self::min()),
                    ));
                }
            }
        }

        if HAS_MAX {
            if MAX_INCLUSIVE {
                if value > Self::max() {
                    return Err(value_too_big_error(value.into_unexpected(), Self::max()));
                }
            } else {
                if value >= Self::max() {
                    struct ExpectedLessThan<T>(T);
                    impl<T> Expected for ExpectedLessThan<T>
                    where
                        T: Debug,
                    {
                        fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
                            write!(formatter, "less than {:?}", self.0)
                        }
                    }
                    return Err(serde::de::Error::invalid_value(
                        value.into_unexpected(),
                        &ExpectedLessThan(Self::max()),
                    ));
                }
            }
        }

        Ok(Ranged::new(value).unwrap())
    }
}

pub fn value_too_small_error<T, E>(unexpected: Unexpected, min_value: T) -> E
where
    T: Debug,
    E: serde::de::Error,
{
    struct ExpectedAtLeast<T>(T);

    impl<T> Expected for ExpectedAtLeast<T>
    where
        T: Debug,
    {
        fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
            write!(formatter, "at least {:?}", self.0)
        }
    }

    E::invalid_value(unexpected, &ExpectedAtLeast(min_value))
}

pub fn value_too_big_error<T, E>(unexpected: Unexpected, max_value: T) -> E
where
    T: Debug,
    E: serde::de::Error,
{
    struct ExpectedAtMost<T>(T);

    impl<T> Expected for ExpectedAtMost<T>
    where
        T: Debug,
    {
        fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
            write!(formatter, "at most {:?}", self.0)
        }
    }

    E::invalid_value(unexpected, &ExpectedAtMost(max_value))
}

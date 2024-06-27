use dashmap::{DashMap, Entry};
use std::borrow::Borrow;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::{mem, slice};

/// A map with multiple values which can only be added to. The slice values in the map share the
/// lifetime of the map itself. This map may be concurrently modified.
// Invariants:
// The following invariants hold until after the start of the Drop function:
// 1. Values in the map are pointers to a value allocated by the global allocator, and the number
//    of such objects allocated.
// 2. The preconditions for `slice::from_raw_parts` hold for all values in the map.
// 3. There are no mutable references to values in the map
pub struct AddOnlyMultiMap<K, V>(DashMap<K, (*mut V, usize), ahash::RandomState>)
where
    K: Hash + Eq;

impl<K, V> AddOnlyMultiMap<K, V>
where
    K: Hash + Eq,
{
    pub fn get<Q>(&self, key: &Q) -> Option<&[V]>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        // SAFETY: slice can be created due to struct invariant 2
        self.0
            .get(key)
            .map(|value| unsafe { slice::from_raw_parts(value.0, value.1) })
    }

    pub fn get_or_try_insert<E>(
        &self,
        key: K,
        new_value: impl FnOnce() -> Result<Vec<V>, E>,
    ) -> Result<&[V], E> {
        match self.0.entry(key) {
            Entry::Occupied(entry) => {
                let (ptr, len) = *entry.get();
                // SAFETY: slice can be created due to struct invariant 2
                Ok(unsafe { slice::from_raw_parts(ptr, len) })
            }
            Entry::Vacant(entry) => {
                // create a value according to struct invariant 1
                let value = new_value()?.into_boxed_slice();
                let len = value.len();
                let ptr = Box::leak(value).as_mut_ptr();
                entry.insert((ptr, len));
                // SAFETY: we just created this value from a box, so it's valid to dereference.
                // Creating an immutable slice in accordance with struct invariant 3
                Ok(unsafe { slice::from_raw_parts(ptr, len) })
            }
        }
    }
}

impl<K, V> Drop for AddOnlyMultiMap<K, V>
where
    K: Hash + Eq,
{
    fn drop(&mut self) {
        for (_, (ptr, len)) in mem::take(&mut self.0) {
            // SAFETY: value is still a valid allocation of the given length in the global allocator (struct invariant 1)
            let value = unsafe { Vec::from_raw_parts(ptr, len, len) };
            drop(value);
        }
    }
}

impl<K, V> Debug for AddOnlyMultiMap<K, V>
where
    K: Hash + Eq + Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut debug_map = f.debug_map();
        for entry in self.0.iter() {
            let (ptr, len) = *entry.value();
            // SAFETY: slice can be created due to struct invariant 2
            debug_map.entry(entry.key(), unsafe { &slice::from_raw_parts(ptr, len) });
        }
        debug_map.finish()
    }
}

impl<K, V> Default for AddOnlyMultiMap<K, V>
where
    K: Hash + Eq,
{
    fn default() -> Self {
        Self(DashMap::default())
    }
}

/// A map which can only be added to. The values in the map share the lifetime of the map itself.
/// This map may be concurrently modified.
// Invariant:
// All values in this map are slices of length 1
#[derive(Debug)]
pub struct AddOnlyMap<K, V>(AddOnlyMultiMap<K, V>)
where
    K: Hash + Eq;

impl<K, V> Default for AddOnlyMap<K, V>
where
    K: Hash + Eq,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<K, V> AddOnlyMap<K, V>
where
    K: Hash + Eq,
{
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.0.get(key).map(|value| {
            // SAFETY: value is length 1 due to struct invariant
            unsafe { value.get_unchecked(0) }
        })
    }

    pub fn get_or_try_insert<E>(
        &self,
        key: K,
        new_value: impl FnOnce() -> Result<V, E>,
    ) -> Result<&V, E> {
        self.0
            .get_or_try_insert(key, || new_value().map(|new_value| vec![new_value]))
            .map(|value| {
                // SAFETY: value is length 1 due to struct invariant
                unsafe { value.get_unchecked(0) }
            })
    }
}

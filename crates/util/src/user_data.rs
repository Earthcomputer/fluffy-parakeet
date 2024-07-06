use parking_lot::{MappedRwLockReadGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};
use std::any::TypeId;
use std::mem::{ManuallyDrop, MaybeUninit};

const INLINE_DATA_SIZE: usize = 80;
const INLINE_DATA_ARRAY_SIZE: usize = INLINE_DATA_SIZE.div_ceil(std::mem::size_of::<u64>());

pub struct UserData {
    inner_locked: RwLock<UserDataInner>,
}

// Invariants:
// 1. This type always "contains" the type specified by self.type_id.
// 2. The contained type is always Send, Sync, Sized and 'static.
// 3. self.size contains the size of the contained type.
// 4. self.drop is a pointer to drop in-place a value of the contained type. It is safe to call with
//    a pointer of type *mut T casted to *mut u8, provided the preconditions of ptr::drop_in_place
//    are met.
// 5. If the size of the contained type is not greater than INLINE_DATA_SIZE, then the stored value
//    is in self.data.inline. The align of the contained type must not be greater than the
//    alignment of u64.
// 6. If the size of the contained type is greater than INLINE_DATA_SIZE, then the stored value is
//    allocated by the global allocator and pointed to by self.data.heap.ptr. self.data.heap.align
//    contains the align of the contained type.
struct UserDataInner {
    type_id: TypeId,
    size: usize,
    drop: unsafe fn(*mut u8),
    data: UserDataData,
}

union UserDataData {
    inline: [MaybeUninit<u64>; INLINE_DATA_ARRAY_SIZE],
    heap: ManuallyDrop<HeapAlloc>,
}

struct HeapAlloc {
    ptr: *mut u8,
    align: usize,
}

struct EmptyUserData;

impl Default for UserData {
    #[inline]
    fn default() -> Self {
        UserData {
            inner_locked: RwLock::new(UserDataInner {
                type_id: TypeId::of::<EmptyUserData>(),
                size: 0,
                drop: do_drop::<EmptyUserData>,
                data: UserDataData {
                    inline: [MaybeUninit::uninit(); INLINE_DATA_ARRAY_SIZE],
                },
            }),
        }
    }
}

// SAFETY: non-send types cannot be stored in UserData (UserDataInner invariant 2)
unsafe impl Send for UserData {}
// SAFETY: non-sync types cannot be stored in UserData (UserDataInner invariant 2)
unsafe impl Sync for UserData {}

impl UserData {
    pub fn get_or_init<T: Send + Sync + 'static>(
        &self,
        init: impl FnOnce() -> T,
    ) -> MappedRwLockReadGuard<T> {
        let inner = self.inner_locked.read();
        if inner.type_id == TypeId::of::<T>() {
            return RwLockReadGuard::map(inner, |inner| {
                // SAFETY: we just checked that the contained type is T
                unsafe { inner.get_data() }
            });
        }
        drop(inner);

        let mut inner = self.inner_locked.write();
        if inner.type_id == TypeId::of::<T>() {
            return RwLockReadGuard::map(RwLockWriteGuard::downgrade(inner), |inner| {
                // SAFETY: we just checked that the contained type is T
                unsafe { inner.get_data() }
            });
        }

        inner.drop_value();
        inner.set_value(init());
        RwLockReadGuard::map(RwLockWriteGuard::downgrade(inner), |inner| {
            // SAFETY: set_value, which we just called, sets the contained type to T
            unsafe { inner.get_data() }
        })
    }
}

impl UserDataInner {
    /// # Safety
    /// Assumes that the contained type is T
    unsafe fn get_data<T>(&self) -> &T {
        if std::mem::size_of::<T>() <= INLINE_DATA_SIZE {
            // SAFETY: the value is in self.data.inline (invariant 5)
            &*(self.data.inline.as_ptr() as *const T)
        } else {
            // SAFETY: the value is pointed to by self.data.heap.ptr (invariant 6)
            &*(self.data.heap.ptr as *const T)
        }
    }

    /// Drops the currently contained value and sets the contained type to `EmptyUserData`.
    fn drop_value(&mut self) {
        // start by setting the type to EmptyUserData so that if the drop panics, we won't have a
        // double-free next time this is called.
        let prev_size = self.size;
        let prev_drop = self.drop;
        self.type_id = TypeId::of::<EmptyUserData>();
        self.size = 0;
        self.drop = do_drop::<EmptyUserData>;

        if prev_size <= INLINE_DATA_SIZE {
            // SAFETY: we're dropping the type that was assigned at the start of this function, it's
            // inline because the size is not greater than INLINE_DATA_SIZE. See invariants 3, 4 and
            // 5
            unsafe {
                prev_drop(self.data.inline.as_mut_ptr() as *mut u8);
            }
        } else {
            // SAFETY: we're dropping the type that was assigned at the start of the function, it's
            // on the heap because the size is greater than INLINE_DATA_SIZE. We then deallocate the
            // value with the global allocator with the type's size and align. See invariants 3, 4
            // and 6
            unsafe {
                prev_drop(self.data.heap.ptr);
                dealloc(
                    self.data.heap.ptr,
                    Layout::from_size_align_unchecked(prev_size, self.data.heap.align),
                );
            }
        }
    }

    fn set_value<T: Send + Sync + 'static>(&mut self, value: T) {
        if std::mem::size_of::<T>() <= INLINE_DATA_SIZE {
            // assert precondition for invariant 5 to avoid unaligned reads and writes of this type
            // in the inline array
            assert!(
                std::mem::align_of::<T>() <= std::mem::align_of::<u64>(),
                "T has greater alignment than u64"
            );
            // SAFETY: writing T to inline memory because its size is not greater than
            // INLINE_DATA_SIZE. T is Send, Sync, Sized and 'static based on the signature of this
            // function. See invariants 2 and 5
            unsafe {
                std::ptr::write(self.data.inline.as_mut_ptr() as *mut T, value);
            }
        } else {
            // SAFETY: the size of T is not 0 because we just checked that it's greater than
            // INLINE_DATA_SIZE
            let ptr = unsafe { alloc(Layout::new::<T>()) };
            if ptr.is_null() {
                handle_alloc_error(Layout::new::<T>());
            }

            self.data.heap = ManuallyDrop::new(HeapAlloc {
                ptr,
                align: std::mem::align_of::<T>(),
            });
            // SAFETY: writing T to heap memory because its size is greater than INLINE_DATA_SIZE,
            // and we just assigned the heap memory to a non-null pointer allocated by the global
            // allocator. See invariant 6
            unsafe {
                std::ptr::write(self.data.heap.ptr as *mut T, value);
            }
        }

        self.type_id = TypeId::of::<T>();
        self.size = std::mem::size_of::<T>();
        self.drop = do_drop::<T>;
    }
}

impl Drop for UserDataInner {
    fn drop(&mut self) {
        self.drop_value();
    }
}

/// # Safety
/// All the preconditions of `ptr::drop_in_place` must be met for `ptr`. Additionally, `ptr` must
/// have earlier been casted from `*mut T`.
unsafe fn do_drop<T>(ptr: *mut u8) {
    std::ptr::drop_in_place(ptr as *mut T);
}

#[cfg(test)]
mod test {
    use crate::user_data::UserData;
    use std::array;

    #[test]
    fn test_ub() {
        let user_data = UserData::default();

        let forty_two = user_data.get_or_init(|| 42);
        assert_eq!(*forty_two, 42);
        drop(forty_two);
        let forty_two = user_data.get_or_init(|| 69);
        assert_eq!(*forty_two, 42);
        drop(forty_two);

        let hello = user_data.get_or_init(|| "hello, world!".to_owned());
        assert_eq!(*hello, "hello, world!");
        drop(hello);
        let hello = user_data.get_or_init(|| "foo".to_owned());
        assert_eq!(*hello, "hello, world!");
        drop(hello);

        let sixty_nine = user_data.get_or_init(|| 69);
        assert_eq!(*sixty_nine, 69);
        drop(sixty_nine);

        let numbers = user_data.get_or_init(|| array::from_fn::<usize, 100, _>(|i| i + 1));
        assert_eq!(numbers.iter().sum::<usize>(), 5050);
        drop(numbers);
        let numbers = user_data.get_or_init(|| array::from_fn::<usize, 100, _>(|i| i + 101));
        assert_eq!(numbers.iter().sum::<usize>(), 5050);
        drop(numbers);

        let sixty_nine = user_data.get_or_init(|| 69);
        assert_eq!(*sixty_nine, 69);
        drop(sixty_nine);

        let numbers = user_data.get_or_init(|| array::from_fn::<usize, 100, _>(|i| i + 101));
        assert_eq!(numbers.iter().sum::<usize>(), 15050);
        drop(numbers);
    }
}

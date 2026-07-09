use std::ops::{Deref, DerefMut};

type UnloadFn<T> = fn(*mut T);

#[derive(Debug)]
/// Bytes that are allocated and returned by RayLib
pub struct RlSlice<T> {
    ptr: *mut T,
    len: usize,
    free: UnloadFn<T>,
}

impl<T> Drop for RlSlice<T> {
    fn drop(&mut self) {
        assert!(!std::mem::needs_drop::<T>());
        (self.free)(self.ptr.cast());
    }
}

impl<T> RlSlice<T> {
    /// # SAFETY
    ///
    /// The pointer must have been allocated by RayLib and is intended to be freed with the
    /// `free` function
    pub(crate) unsafe fn from_raw_parts(ptr: *mut T, len: usize, free: UnloadFn<T>) -> Self {
        assert!(!ptr.is_null() || len == 0);
        Self { ptr, len, free }
    }

    /// Reallocate this as a rust vec
    pub fn reallocate(self) -> Vec<T>
    where
        T: Clone,
    {
        self.to_vec()
    }
}

impl<T> AsRef<[T]> for RlSlice<T> {
    fn as_ref(&self) -> &[T] {
        // SAFETY: required by constructor
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl<T> Deref for RlSlice<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> DerefMut for RlSlice<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: required by constructor
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

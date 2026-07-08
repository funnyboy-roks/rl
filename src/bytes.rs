use std::ops::{Deref, DerefMut};

use raylib_sys as sys;

#[derive(Debug)]
/// Bytes that are allocated and returned by RayLib
pub struct RlBytesOwned {
    ptr: *mut u8,
    len: usize,
}

impl Drop for RlBytesOwned {
    fn drop(&mut self) {
        // SAFETY: from_raw_parts requires that the pointer was allocated by RayLib
        unsafe { sys::MemFree(self.ptr.cast()) };
    }
}

impl RlBytesOwned {
    /// # SAFETY
    ///
    /// The pointer must have been allocated by RayLib (probaly MemAlloc)
    pub(crate) unsafe fn from_raw_parts(ptr: *mut u8, len: usize) -> Self {
        Self { ptr, len }
    }

    /// Reallocate this as a rust vec
    pub fn reallocate(self) -> Vec<u8> {
        self.to_vec()
    }
}

impl AsRef<[u8]> for RlBytesOwned {
    fn as_ref(&self) -> &[u8] {
        // SAFETY: required by constructor
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl Deref for RlBytesOwned {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl DerefMut for RlBytesOwned {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: required by constructor
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

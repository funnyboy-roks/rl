use std::{
    cell::RefCell,
    ffi::{CStr, CString, FromBytesWithNulError},
    fs::File,
    io::{self, Read},
    path::Path,
};

/// `std::fs::read_to_string` except it produces a c-string (without reallocating after reading)
pub(crate) fn read_to_c_string(path: impl AsRef<Path>) -> io::Result<CString> {
    let path = path.as_ref();
    let mut file = File::open(path)?;
    let size = file
        .metadata()
        .map(|m| usize::try_from(m.len() + 1).unwrap_or(usize::MAX))
        .ok();
    let mut bytes = Vec::with_capacity(size.unwrap_or(0));
    file.read_to_end(&mut bytes)?;

    bytes.push(0); // push NULL

    CString::from_vec_with_nul(bytes).map_err(io::Error::other)
}

thread_local! {
    static CSTR_BUF: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}

/// Convert a string into a c-string by adding a NULL byte.
///
/// # SAFETY
///
/// The returned c-string will become invalid once this function is called again.
pub unsafe fn allocate_cstring<'a, S: AsRef<str>>(s: S) -> &'a CStr {
    unsafe { allocate_cstring_bytes(s.as_ref()) }.expect("str has no null")
}

/// Convert a string into a c-string by adding a NULL byte.
///
/// # SAFETY
///
/// The returned c-string will become invalid once this function is called again.
pub unsafe fn allocate_cstring_bytes<'a, S: AsRef<[u8]>>(
    s: S,
) -> Result<&'a CStr, FromBytesWithNulError> {
    CSTR_BUF.with_borrow_mut(|buf| {
        buf.clear();
        let bytes = s.as_ref();
        buf.reserve(bytes.len() + 1);
        buf.extend_from_slice(bytes);
        buf.push(0);
        // SAFETY: This is the same as <Vec as AsRef<[u8]>>::as_ref()
        // NOTE: We use from_raw_parts instead of as_ref to erase the lifetime
        let slice = unsafe { std::slice::from_raw_parts(buf.as_ptr(), buf.len()) };
        CStr::from_bytes_with_nul(slice)
    })
}

/// Convert a string into a c-string by adding a NULL byte.
///
/// # SAFETY
///
/// The returned c-string will become invalid once this function is called again.
pub unsafe fn allocate_cstrings<'a, const N: usize, S: AsRef<[u8]>>(s: [S; N]) -> [&'a CStr; N] {
    CSTR_BUF.with_borrow_mut(|buf| {
        buf.clear();

        s.map(|s| {
            let bytes = s.as_ref();
            buf.reserve(bytes.len() + 1);
            buf.extend_from_slice(bytes);
            buf.push(0);
            // SAFETY: This is the same as <Vec as AsRef<[u8]>>::as_ref()
            // NOTE: We use from_raw_parts instead of as_ref to erase the lifetime
            let slice = unsafe { std::slice::from_raw_parts(buf.as_ptr(), buf.len()) };
            CStr::from_bytes_with_nul(slice).expect("str cannot have null")
        })
    })
}

#[cfg(test)]
mod test {
    use crate::util::allocate_cstring;

    #[test]
    fn c_string() {
        let s = "hello world";
        let cs = unsafe { allocate_cstring(s) };
        assert_eq!(cs, c"hello world");

        let s = "foo bar";
        let cs = unsafe { allocate_cstring(s) };
        assert_eq!(cs, c"foo bar");
    }
}

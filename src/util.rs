use std::{
    ffi::CString,
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

use std::ffi::CString;

use raylib_sys as sys;

pub fn measure(text: impl AsRef<str>, font_size: u32) -> u32 {
    let text = CString::new(text.as_ref()).expect("str has no null");
    unsafe { sys::MeasureText(text.as_ptr(), font_size as _) }
        .try_into()
        .unwrap()
}

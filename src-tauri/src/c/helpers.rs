use std::{ffi::OsStr, os::windows::prelude::OsStrExt};

pub fn to_wide_string(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(Some(0)).collect()
}

use std::slice::from_raw_parts;

use kanaria::string::{TextSearch, UCSStr};

#[no_mangle]
pub unsafe extern "C" fn is_contains_for_utf16(
    src_chars_ptr: *const u16,
    src_chars_ptr_size: u32,
    search_chars_ptr: *const u16,
    search_chars_ptr_size: u32,
) -> bool {
    let src = from_raw_parts(src_chars_ptr, src_chars_ptr_size as usize);
    let search = from_raw_parts(search_chars_ptr, search_chars_ptr_size as usize);

    let ucs_str = UCSStr::from_slice(src);
    ucs_str.is_contains(search)
}

#[no_mangle]
pub unsafe extern "C" fn index_of_for_utf16(
    src_chars_ptr: *const u16,
    src_chars_ptr_size: u32,
    search_chars_ptr: *const u16,
    search_chars_ptr_size: u32,
) -> i32 {
    let src = from_raw_parts(src_chars_ptr, src_chars_ptr_size as usize);
    let search = from_raw_parts(search_chars_ptr, search_chars_ptr_size as usize);

    let ucs_str = UCSStr::from_slice(src);
    ucs_str.index_of(search)
}

use kanaria::converter::{Converter, ConverterFactory};

#[no_mangle]
pub unsafe extern "C" fn to_upper_case_for_utf16(
    target_chars_ptr: *const u16, target_chars_ptr_size: u32,
    result_chars_ptr: *mut u16, result_chars_ptr_size: u32,
) -> u32 {
    result_chars_ptr.write_bytes(0, result_chars_ptr_size as usize);
    ConverterFactory::from_raw(target_chars_ptr, target_chars_ptr_size as usize)
        .upper_case()
        .write_to_ptr(result_chars_ptr) as u32
}

#[no_mangle]
pub unsafe extern "C" fn to_lower_case_for_utf16(
    target_chars_ptr: *const u16, target_chars_ptr_size: u32,
    result_chars_ptr: *mut u16, result_chars_ptr_size: u32,
) -> u32 {
    result_chars_ptr.write_bytes(0, result_chars_ptr_size as usize);
    ConverterFactory::from_raw(target_chars_ptr, target_chars_ptr_size as usize)
        .lower_case()
        .write_to_ptr(result_chars_ptr) as u32
}

#[no_mangle]
pub unsafe extern "C" fn to_wide_for_utf16(
    target_chars_ptr: *const u16, target_chars_ptr_size: u32,
    result_chars_ptr: *mut u16, result_chars_ptr_size: u32,
) -> u32 {
    result_chars_ptr.write_bytes(0, result_chars_ptr_size as usize);
    ConverterFactory::from_raw(target_chars_ptr, target_chars_ptr_size as usize)
        .wide()
        .write_to_ptr(result_chars_ptr) as u32
}

#[no_mangle]
pub unsafe extern "C" fn to_narrow_for_utf16(
    target_chars_ptr: *const u16, target_chars_ptr_size: u32,
    result_chars_ptr: *mut u16, result_chars_ptr_size: u32,
) -> u32 {
    result_chars_ptr.write_bytes(0, result_chars_ptr_size as usize);
    ConverterFactory::from_raw(target_chars_ptr, target_chars_ptr_size as usize)
        .narrow()
        .write_to_ptr(result_chars_ptr) as u32
}

#[no_mangle]
pub unsafe extern "C" fn to_hiragana_for_utf16(
    target_chars_ptr: *const u16, target_chars_ptr_size: u32,
    result_chars_ptr: *mut u16, result_chars_ptr_size: u32,
) -> u32 {
    result_chars_ptr.write_bytes(0, result_chars_ptr_size as usize);
    ConverterFactory::from_raw(target_chars_ptr, target_chars_ptr_size as usize)
        .hiragana()
        .write_to_ptr(result_chars_ptr) as u32
}

#[no_mangle]
pub unsafe extern "C" fn to_katakana_for_utf16(
    target_chars_ptr: *const u16, target_chars_ptr_size: u32,
    result_chars_ptr: *mut u16, result_chars_ptr_size: u32,
) -> u32 {
    result_chars_ptr.write_bytes(0, result_chars_ptr_size as usize);
    ConverterFactory::from_raw(target_chars_ptr, target_chars_ptr_size as usize)
        .katakana()
        .write_to_ptr(result_chars_ptr) as u32
}

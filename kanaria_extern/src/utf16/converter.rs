use kanaria::constants::*;
use kanaria::string::ConvertType;
use kanaria::string::ConvertType::{Hiragana, Katakana, LowerCase, Narrow, UpperCase, Wide};
use kanaria::string::UCSStr;
use kanaria::utils::ConvertTarget;

fn int_to_convert_type(convert_type: u32) -> ConvertType {
    match convert_type {
        CONVERT_TYPE_UPPER_CASE => UpperCase,
        CONVERT_TYPE_LOWER_CASE => LowerCase,
        CONVERT_TYPE_HIRAGANA => Hiragana,
        CONVERT_TYPE_KATAKANA => Katakana,
        CONVERT_TYPE_NARROW => Narrow,
        CONVERT_TYPE_WIDE => Wide,
        _ => ConvertType::None,
    }
}

#[no_mangle]
pub unsafe extern "C" fn convert_for_utf16(
    src_chars_ptr: *const u16,
    src_chars_ptr_size: u32,
    dst_chars_ptr: *mut u16,
    dst_chars_ptr_size: u32,
    convert_type: u32,
    convert_target: u32,
) -> u32 {
    dst_chars_ptr.write_bytes(0, dst_chars_ptr_size as usize);
    return UCSStr::convert_raw(
        src_chars_ptr,
        dst_chars_ptr,
        src_chars_ptr_size as usize,
        int_to_convert_type(convert_type),
        ConvertTarget::from_bits(convert_target).unwrap_or(ConvertTarget::ALL),
    ) as u32;
}

#[no_mangle]
pub unsafe extern "C" fn to_upper_case_for_utf16(
    src_chars_ptr: *const u16,
    src_chars_ptr_size: u32,
    dst_chars_ptr: *mut u16,
    dst_chars_ptr_size: u32,
) -> u32 {
    convert_for_utf16(
        src_chars_ptr,
        src_chars_ptr_size,
        dst_chars_ptr,
        dst_chars_ptr_size,
        CONVERT_TYPE_UPPER_CASE,
        CONVERT_TARGET_ALL,
    )
}

#[no_mangle]
pub unsafe extern "C" fn to_lower_case_for_utf16(
    src_chars_ptr: *const u16,
    src_chars_ptr_size: u32,
    dst_chars_ptr: *mut u16,
    dst_chars_ptr_size: u32,
) -> u32 {
    convert_for_utf16(
        src_chars_ptr,
        src_chars_ptr_size,
        dst_chars_ptr,
        dst_chars_ptr_size,
        CONVERT_TYPE_LOWER_CASE,
        CONVERT_TARGET_ALL,
    )
}

#[no_mangle]
pub unsafe extern "C" fn to_hiragana_for_utf16(
    src_chars_ptr: *const u16,
    src_chars_ptr_size: u32,
    dst_chars_ptr: *mut u16,
    dst_chars_ptr_size: u32,
) -> u32 {
    convert_for_utf16(
        src_chars_ptr,
        src_chars_ptr_size,
        dst_chars_ptr,
        dst_chars_ptr_size,
        CONVERT_TYPE_HIRAGANA,
        CONVERT_TARGET_ALL,
    )
}

#[no_mangle]
pub unsafe extern "C" fn to_katakana_for_utf16(
    src_chars_ptr: *const u16,
    src_chars_ptr_size: u32,
    dst_chars_ptr: *mut u16,
    dst_chars_ptr_size: u32,
) -> u32 {
    convert_for_utf16(
        src_chars_ptr,
        src_chars_ptr_size,
        dst_chars_ptr,
        dst_chars_ptr_size,
        CONVERT_TYPE_KATAKANA,
        CONVERT_TARGET_ALL,
    )
}

#[no_mangle]
pub unsafe extern "C" fn to_wide_for_utf16(
    src_chars_ptr: *const u16,
    src_chars_ptr_size: u32,
    dst_chars_ptr: *mut u16,
    dst_chars_ptr_size: u32,
    convert_target: u32,
) -> u32 {
    convert_for_utf16(
        src_chars_ptr,
        src_chars_ptr_size,
        dst_chars_ptr,
        dst_chars_ptr_size,
        CONVERT_TYPE_WIDE,
        convert_target,
    )
}

#[no_mangle]
pub unsafe extern "C" fn to_narrow_for_utf16(
    src_chars_ptr: *const u16,
    src_chars_ptr_size: u32,
    dst_chars_ptr: *mut u16,
    dst_chars_ptr_size: u32,
    convert_target: u32,
) -> u32 {
    convert_for_utf16(
        src_chars_ptr,
        src_chars_ptr_size,
        dst_chars_ptr,
        dst_chars_ptr_size,
        CONVERT_TYPE_NARROW,
        convert_target
    )
}

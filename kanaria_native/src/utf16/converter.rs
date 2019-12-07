use kanaria::str::ConvertType;
use kanaria::UCSStr;
use kanaria::utils::ConvertTarget;

#[no_mangle]
pub unsafe extern "C" fn to_upper_case_for_utf16(
    src_chars_ptr: *const u16, src_chars_ptr_size: u32,
    dst_chars_ptr: *mut u16, dst_chars_ptr_size: u32,
) -> u32 {
    dst_chars_ptr.write_bytes(0, dst_chars_ptr_size as usize);
    return UCSStr::convert(
        src_chars_ptr,
        dst_chars_ptr,
        src_chars_ptr_size as usize,
        ConvertType::UpperCase,
        ConvertTarget::ALL,
    ) as u32;
}

#[no_mangle]
pub unsafe extern "C" fn to_lower_case_for_utf16(
    src_chars_ptr: *const u16, src_chars_ptr_size: u32,
    dst_chars_ptr: *mut u16, dst_chars_ptr_size: u32,
) -> u32 {
    dst_chars_ptr.write_bytes(0, dst_chars_ptr_size as usize);
    return UCSStr::convert(
        src_chars_ptr,
        dst_chars_ptr,
        src_chars_ptr_size as usize,
        ConvertType::LowerCase,
        ConvertTarget::ALL,
    ) as u32;
}

#[no_mangle]
pub unsafe extern "C" fn to_wide_for_utf16(
    src_chars_ptr: *const u16, src_chars_ptr_size: u32,
    dst_chars_ptr: *mut u16, dst_chars_ptr_size: u32,
    convert_target: u32
) -> u32 {
    dst_chars_ptr.write_bytes(0, dst_chars_ptr_size as usize);
    return UCSStr::convert(
        src_chars_ptr,
        dst_chars_ptr,
        src_chars_ptr_size as usize,
        ConvertType::Wide,
        ConvertTarget::from_bits(convert_target).unwrap_or(ConvertTarget::ALL),
    ) as u32;
}

#[no_mangle]
pub unsafe extern "C" fn to_narrow_for_utf16(
    src_chars_ptr: *const u16, src_chars_ptr_size: u32,
    dst_chars_ptr: *mut u16, dst_chars_ptr_size: u32,
    convert_target: u32
) -> u32 {
    dst_chars_ptr.write_bytes(0, dst_chars_ptr_size as usize);
    return UCSStr::convert(
        src_chars_ptr,
        dst_chars_ptr,
        src_chars_ptr_size as usize,
        ConvertType::Narrow,
        ConvertTarget::from_bits(convert_target).unwrap_or(ConvertTarget::ALL),
    ) as u32;
}

#[no_mangle]
pub unsafe extern "C" fn to_hiragana_for_utf16(
    src_chars_ptr: *const u16, src_chars_ptr_size: u32,
    dst_chars_ptr: *mut u16, dst_chars_ptr_size: u32,
) -> u32 {
    dst_chars_ptr.write_bytes(0, dst_chars_ptr_size as usize);
    return UCSStr::convert(
        src_chars_ptr,
        dst_chars_ptr,
        src_chars_ptr_size as usize,
        ConvertType::Hiragana,
        ConvertTarget::ALL,
    ) as u32;
}

#[no_mangle]
pub unsafe extern "C" fn to_katakana_for_utf16(
    src_chars_ptr: *const u16, src_chars_ptr_size: u32,
    dst_chars_ptr: *mut u16, dst_chars_ptr_size: u32,
) -> u32 {
    dst_chars_ptr.write_bytes(0, dst_chars_ptr_size as usize);
    return UCSStr::convert(
        src_chars_ptr,
        dst_chars_ptr,
        src_chars_ptr_size as usize,
        ConvertType::Katakana,
        ConvertTarget::ALL,
    ) as u32;
}

use kanaria::utils::{AsciiUtils, KanaUtils, WidthUtils, CharsUtils};

#[no_mangle]
pub unsafe extern "C" fn is_narrow_for_utf16(target: u16) -> bool {
    CharsUtils::is_narrow(target)
}

#[no_mangle]
pub unsafe extern "C" fn is_wide_for_utf16(target: u16) -> bool {
    CharsUtils::is_wide(target)
}


#[no_mangle]
pub unsafe extern "C" fn is_ascii_for_utf16(target: u16) -> bool {
    AsciiUtils::is_ascii(target)
}

#[no_mangle]
pub unsafe extern "C" fn is_narrow_ascii_for_utf16(target: u16) -> bool {
    AsciiUtils::is_narrow_ascii(target)
}

#[no_mangle]
pub unsafe extern "C" fn is_wide_ascii_for_utf16(target: u16) -> bool {
    AsciiUtils::is_wide_ascii(target)
}

#[no_mangle]
pub unsafe extern "C" fn is_lower_case_for_utf16(target: u16) -> bool {
    AsciiUtils::is_lower_case(target)
}

#[no_mangle]
pub unsafe extern "C" fn is_narrow_lower_case_for_utf16(target: u16) -> bool {
    AsciiUtils::is_narrow_lower_case(target)
}

#[no_mangle]
pub unsafe extern "C" fn is_wide_lower_case_for_utf16(target: u16) -> bool {
    AsciiUtils::is_wide_lower_case(target)
}

#[no_mangle]
pub unsafe extern "C" fn is_upper_case_for_utf16(target: u16) -> bool {
    AsciiUtils::is_upper_case(target)
}

#[no_mangle]
pub unsafe extern "C" fn is_narrow_upper_case_for_utf16(target: u16) -> bool {
    AsciiUtils::is_narrow_upper_case(target)
}

#[no_mangle]
pub unsafe extern "C" fn is_wide_upper_case_for_utf16(target: u16) -> bool {
    AsciiUtils::is_wide_upper_case(target)
}

#[no_mangle]
pub unsafe extern "C" fn is_ascii_symbol_for_utf16(target: u16) -> bool {
    AsciiUtils::is_ascii_symbol(target)
}

#[no_mangle]
pub unsafe extern "C" fn is_narrow_ascii_symbol_for_utf16(target: u16) -> bool {
    AsciiUtils::is_narrow_ascii_symbol(target)
}

#[no_mangle]
pub unsafe extern "C" fn is_wide_ascii_symbol_for_utf16(target: u16) -> bool {
    AsciiUtils::is_wide_ascii_symbol(target)
}

#[no_mangle]
pub unsafe extern "C" fn is_number_for_utf16(target: u16) -> bool {
    AsciiUtils::is_number(target)
}

#[no_mangle]
pub unsafe extern "C" fn is_narrow_number_for_utf16(target: u16) -> bool {
    AsciiUtils::is_narrow_number(target)
}

#[no_mangle]
pub unsafe extern "C" fn is_wide_number_for_utf16(target: u16) -> bool {
    AsciiUtils::is_wide_number(target)
}

#[no_mangle]
pub unsafe extern "C" fn convert_to_upper_case_for_utf16(target: u16) -> u16 {
    AsciiUtils::convert_to_upper_case(target)
}

#[no_mangle]
pub unsafe extern "C" fn convert_to_lower_case_for_utf16(target: u16) -> u16 {
    AsciiUtils::convert_to_lower_case(target)
}



#[no_mangle]
pub unsafe extern "C" fn is_hiragana_for_utf16(target: u16) -> bool {
    KanaUtils::is_hiragana(target)
}

#[no_mangle]
pub unsafe extern "C" fn is_katakana_for_utf16(target: u16) -> bool {
    KanaUtils::is_katakana(target)
}

#[no_mangle]
pub unsafe extern "C" fn is_narrow_katakana_for_utf16(target: u16) -> bool {
    KanaUtils::is_narrow_katakana(target)
}

#[no_mangle]
pub unsafe extern "C" fn is_wide_katakana_for_utf16(target: u16) -> bool {
    KanaUtils::is_wide_katakana(target)
}

#[no_mangle]
pub unsafe extern "C" fn is_jis_symbol(target: u16) -> bool {
    KanaUtils::is_jis_symbol(target)
}

#[no_mangle]
pub unsafe extern "C" fn is_narrow_jis_symbol_for_utf16(target: u16) -> bool {
    KanaUtils::is_narrow_jis_symbol(target)
}

#[no_mangle]
pub unsafe extern "C" fn is_wide_jis_symbol_for_utf16(target: u16) -> bool {
    KanaUtils::is_wide_jis_symbol(target)
}

#[no_mangle]
pub unsafe extern "C" fn is_can_convert_hiragana_for_utf16(target: u16) -> bool {
    KanaUtils::is_can_convert_hiragana(target)
}

#[no_mangle]
pub unsafe extern "C" fn convert_to_hiragana_for_utf16(target: u16) -> u16 {
    KanaUtils::convert_to_hiragana(target)
}

#[no_mangle]
pub unsafe extern "C" fn convert_to_katakana_for_utf16(target: u16) -> u16 {
    KanaUtils::convert_to_katakana(target)
}



#[no_mangle]
pub unsafe extern "C" fn convert_to_wide_for_utf16(target: u16, next: u16, is_pad: *mut bool) -> u16 {
    let (result_ret, is_pad_ret) = WidthUtils::convert_to_wide(target, next);
    is_pad.write(is_pad_ret);
    return result_ret;
}

#[no_mangle]
pub unsafe extern "C" fn convert_to_narrow_for_utf16(target: u16, out_char: *mut u16) -> u16 {
    let (first, second) = WidthUtils::convert_to_narrow(target);
    out_char.write(second);
    return first;
}
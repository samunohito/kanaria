extern crate jni;
extern crate kanaria;

use std::slice::from_raw_parts;

use jni::JNIEnv;
use jni::sys::*;

use kanaria::utils::{AsciiUtils, KanaUtils, WidthUtils};

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub extern fn Java_com_kanaria_utils_AsciiUtils_isAsciiNative(env: JNIEnv, class: jclass, target: jchar) -> jboolean {
    AsciiUtils::is_ascii(target) as jboolean
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub extern fn Java_com_kanaria_utils_AsciiUtils_isNarrowAsciiNative(env: JNIEnv, class: jclass, target: jchar) -> jboolean {
    AsciiUtils::is_narrow_ascii(target) as jboolean
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub extern fn Java_com_kanaria_utils_AsciiUtils_isWideAsciiNative(env: JNIEnv, class: jclass, target: jchar) -> jboolean {
    AsciiUtils::is_wide_ascii(target) as jboolean
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub extern fn Java_com_kanaria_utils_AsciiUtils_isLowerCaseNative(env: JNIEnv, class: jclass, target: jchar) -> jboolean {
    AsciiUtils::is_lower_case(target) as jboolean
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub extern fn Java_com_kanaria_utils_AsciiUtils_isNarrowLowerCaseNative(env: JNIEnv, class: jclass, target: jchar) -> jboolean {
    AsciiUtils::is_narrow_lower_case(target) as jboolean
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub extern fn Java_com_kanaria_utils_AsciiUtils_isWideLowerCaseNative(env: JNIEnv, class: jclass, target: jchar) -> jboolean {
    AsciiUtils::is_wide_lower_case(target) as jboolean
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub extern fn Java_com_kanaria_utils_AsciiUtils_isUpperCaseNative(env: JNIEnv, class: jclass, target: jchar) -> jboolean {
    AsciiUtils::is_upper_case(target) as jboolean
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub extern fn Java_com_kanaria_utils_AsciiUtils_isNarrowUpperCaseNative(env: JNIEnv, class: jclass, target: jchar) -> jboolean {
    AsciiUtils::is_narrow_upper_case(target) as jboolean
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub extern fn Java_com_kanaria_utils_AsciiUtils_isWideUpperCaseNative(env: JNIEnv, class: jclass, target: jchar) -> jboolean {
    AsciiUtils::is_wide_upper_case(target) as jboolean
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub extern fn Java_com_kanaria_utils_AsciiUtils_isAsciiSymbolNative(env: JNIEnv, class: jclass, target: jchar) -> jboolean {
    AsciiUtils::is_ascii_symbol(target) as jboolean
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub extern fn Java_com_kanaria_utils_AsciiUtils_isNarrowAsciiSymbolNative(env: JNIEnv, class: jclass, target: jchar) -> jboolean {
    AsciiUtils::is_narrow_ascii_symbol(target) as jboolean
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub extern fn Java_com_kanaria_utils_AsciiUtils_isWideAsciiSymbolNative(env: JNIEnv, class: jclass, target: jchar) -> jboolean {
    AsciiUtils::is_wide_ascii_symbol(target) as jboolean
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub extern fn Java_com_kanaria_utils_AsciiUtils_isNumberNative(env: JNIEnv, class: jclass, target: jchar) -> jboolean {
    AsciiUtils::is_number(target) as jboolean
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub extern fn Java_com_kanaria_utils_AsciiUtils_isNarrowNumberNative(env: JNIEnv, class: jclass, target: jchar) -> jboolean {
    AsciiUtils::is_narrow_number(target) as jboolean
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub extern fn Java_com_kanaria_utils_AsciiUtils_isWideNumberNative(env: JNIEnv, class: jclass, target: jchar) -> jboolean {
    AsciiUtils::is_wide_number(target) as jboolean
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub extern fn Java_com_kanaria_utils_AsciiUtils_convertToUpperCaseNative(env: JNIEnv, class: jclass, target: jchar) -> jchar {
    AsciiUtils::convert_to_upper_case(target) as jchar
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub extern fn Java_com_kanaria_utils_AsciiUtils_convertToLowerCaseNative(env: JNIEnv, class: jclass, target: jchar) -> jchar {
    AsciiUtils::convert_to_lower_case(target) as jchar
}


#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub extern fn Java_com_kanaria_utils_KanaUtils_isHiraganaNative(env: JNIEnv, class: jclass, target: jchar) -> jboolean {
    KanaUtils::is_hiragana(target) as jboolean
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub extern fn Java_com_kanaria_utils_KanaUtils_isKatakanaNative(env: JNIEnv, class: jclass, target: jchar) -> jboolean {
    KanaUtils::is_katakana(target) as jboolean
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub extern fn Java_com_kanaria_utils_KanaUtils_isNarrowKatakanaNative(env: JNIEnv, class: jclass, target: jchar) -> jboolean {
    KanaUtils::is_narrow_katakana(target) as jboolean
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub extern fn Java_com_kanaria_utils_KanaUtils_isWideKatakanaNative(env: JNIEnv, class: jclass, target: jchar) -> jboolean {
    KanaUtils::is_wide_katakana(target) as jboolean
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub extern fn Java_com_kanaria_utils_KanaUtils_isJisSymbolNative(env: JNIEnv, class: jclass, target: jchar) -> jboolean {
    KanaUtils::is_jis_symbol(target) as jboolean
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub extern fn Java_com_kanaria_utils_KanaUtils_isNarrowJisSymbolNative(env: JNIEnv, class: jclass, target: jchar) -> jboolean {
    KanaUtils::is_narrow_jis_symbol(target) as jboolean
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub extern fn Java_com_kanaria_utils_KanaUtils_isWideJisSymbolNative(env: JNIEnv, class: jclass, target: jchar) -> jboolean {
    KanaUtils::is_wide_jis_symbol(target) as jboolean
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub extern fn Java_com_kanaria_utils_KanaUtils_isCanConvertHiraganaNative(env: JNIEnv, class: jclass, target: jchar) -> jboolean {
    KanaUtils::is_can_convert_hiragana(target) as jboolean
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub extern fn Java_com_kanaria_utils_KanaUtils_convertToHiraganaNative(env: JNIEnv, class: jclass, target: jchar) -> jchar {
    KanaUtils::convert_to_hiragana(target) as jchar
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub extern fn Java_com_kanaria_utils_KanaUtils_convertToKatakanaNative(env: JNIEnv, class: jclass, target: jchar) -> jchar {
    KanaUtils::convert_to_katakana(target) as jchar
}


#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub unsafe extern fn Java_com_kanaria_utils_WidthUtils_convertToWideNative(env: JNIEnv, class: jclass, target: jchar, next: jchar, is_pad: jbooleanArray) -> jchar {
    let (result_ret, is_pad_ret) = WidthUtils::convert_to_wide(target, next);
    let _ = env.set_boolean_array_region(is_pad, 0, from_raw_parts(&(is_pad_ret as u8) as *const jboolean, 1));
    return result_ret as jchar;
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub unsafe extern fn Java_com_kanaria_utils_WidthUtils_convertToNarrowNative(env: JNIEnv, class: jclass, target: jchar, out_char: jcharArray) -> jchar {
    let (first, second) = WidthUtils::convert_to_narrow(target);
    let _ = env.set_char_array_region(out_char, 0, from_raw_parts(&(second as u16) as *const jchar, 1));
    return first as jchar;
}
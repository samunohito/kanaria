extern crate jni;
extern crate kanaria;

use jni::JNIEnv;
use jni::sys::*;

use kanaria::constants::*;
use kanaria::string::ConvertType;
use kanaria::string::ConvertType::{Hiragana, Katakana, LowerCase, Narrow, UpperCase, Wide};
use kanaria::utils::ConvertTarget;
use kanaria::string::UCSStr;

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
#[allow(unused_variables, non_snake_case)]
pub unsafe extern fn Java_com_kanaria_UcsString_convertNative(
    env: JNIEnv,
    class: jclass,
    src_chars: jcharArray,
    src_chars_len: jint,
    dst_chars: jcharArray,
    dst_chars_len: jint,
    convert_type: jint,
    convert_target: jint,
) -> jint {
    // 処理対象文字列の取得
    let mut src_buffer = Vec::<jchar>::with_capacity(src_chars_len as usize);
    src_buffer.set_len(src_chars_len as usize);
    let _ = env.get_char_array_region(src_chars, 0, src_buffer.as_mut());

    // 処理後文字列の領域確保
    let mut dst_buffer = Vec::with_capacity(dst_chars_len as usize);
    dst_buffer.set_len(dst_chars_len as usize);

    // 変換処理
    let size = UCSStr::convert_raw(
        src_buffer.as_ptr(),
        dst_buffer.as_mut_ptr(),
        src_chars_len as usize,
        int_to_convert_type(convert_type as u32),
        ConvertTarget::from_bits(convert_target as u32).unwrap_or(ConvertTarget::ALL),
    ) as u32;
    dst_buffer.set_len(size as usize);

    // 処理後文字列の返却処理
    let _ = env.set_char_array_region(dst_chars, 0, dst_buffer.as_slice());

    return dst_buffer.len() as jint;
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub unsafe extern fn Java_com_kanaria_UcsString_toUpperCaseNative(
    env: JNIEnv,
    class: jclass,
    src_chars: jcharArray,
    src_chars_len: jint,
    dst_chars: jcharArray,
    dst_chars_len: jint,
) -> jint {
    Java_com_kanaria_UcsString_convertNative(
        env,
        class,
        src_chars,
        src_chars_len,
        dst_chars,
        dst_chars_len,
        CONVERT_TYPE_UPPER_CASE as jint,
        CONVERT_TARGET_ALL as jint,
    )
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub unsafe extern fn Java_com_kanaria_UcsString_toLowerCaseNative(
    env: JNIEnv,
    class: jclass,
    src_chars: jcharArray,
    src_chars_len: jint,
    dst_chars: jcharArray,
    dst_chars_len: jint,
) -> jint {
    Java_com_kanaria_UcsString_convertNative(
        env,
        class,
        src_chars,
        src_chars_len,
        dst_chars,
        dst_chars_len,
        CONVERT_TYPE_LOWER_CASE as jint,
        CONVERT_TARGET_ALL as jint,
    )
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub unsafe extern fn Java_com_kanaria_UcsString_toHiraganaNative(
    env: JNIEnv,
    class: jclass,
    src_chars: jcharArray,
    src_chars_len: jint,
    dst_chars: jcharArray,
    dst_chars_len: jint,
) -> jint {
    Java_com_kanaria_UcsString_convertNative(
        env,
        class,
        src_chars,
        src_chars_len,
        dst_chars,
        dst_chars_len,
        CONVERT_TYPE_HIRAGANA as jint,
        CONVERT_TARGET_ALL as jint,
    )
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub unsafe extern fn Java_com_kanaria_UcsString_toKatakanaNative(
    env: JNIEnv,
    class: jclass,
    src_chars: jcharArray,
    src_chars_len: jint,
    dst_chars: jcharArray,
    dst_chars_len: jint,
) -> jint {
    Java_com_kanaria_UcsString_convertNative(
        env,
        class,
        src_chars,
        src_chars_len,
        dst_chars,
        dst_chars_len,
        CONVERT_TYPE_KATAKANA as jint,
        CONVERT_TARGET_ALL as jint,
    )
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub unsafe extern fn Java_com_kanaria_UcsString_toWideNative(
    env: JNIEnv,
    class: jclass,
    src_chars: jcharArray,
    src_chars_len: jint,
    dst_chars: jcharArray,
    dst_chars_len: jint,
    convert_target: jint,
) -> jint {
    Java_com_kanaria_UcsString_convertNative(
        env,
        class,
        src_chars,
        src_chars_len,
        dst_chars,
        dst_chars_len,
        CONVERT_TYPE_WIDE as jint,
        convert_target,
    )
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub unsafe extern fn Java_com_kanaria_UcsString_toNarrowNative(
    env: JNIEnv,
    class: jclass,
    src_chars: jcharArray,
    src_chars_len: jint,
    dst_chars: jcharArray,
    dst_chars_len: jint,
    convert_target: jint,
) -> jint {
    Java_com_kanaria_UcsString_convertNative(
        env,
        class,
        src_chars,
        src_chars_len,
        dst_chars,
        dst_chars_len,
        CONVERT_TYPE_NARROW as jint,
        convert_target,
    )
}

extern crate jni;
extern crate kanaria;

use jni::JNIEnv;
use jni::sys::*;

use kanaria::converter::{Converter, ConverterFactory};

unsafe fn read_target_chars(
    env: &JNIEnv,
    target_chars: jcharArray,
    target_chars_len: jint,
) -> Vec<jchar> {
    let mut buffer = Vec::<jchar>::with_capacity(target_chars_len as usize);
    buffer.set_len(target_chars_len as usize);
    let _ = env.get_char_array_region(target_chars, 0, buffer.as_mut());
    return buffer;
}

fn write_result_chars(
    env: &JNIEnv,
    result_chars: &[jchar],
    result_chars_buffer: jcharArray,
) {
    let _ = env.set_char_array_region(result_chars_buffer, 0, result_chars);
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub unsafe extern fn Java_com_kanaria_UcsString_toUpperCaseNative(
    env: JNIEnv,
    class: jclass,
    target_chars: jcharArray,
    target_chars_len: jint,
    result_chars_buffer: jcharArray,
) -> jint {
    let target = read_target_chars(&env, target_chars, target_chars_len);
    let result = ConverterFactory::from_slice(target.as_slice())
        .upper_case()
        .to_vec();
    write_result_chars(&env, result.as_slice(), result_chars_buffer);
    return result.len() as jint;
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub unsafe extern fn Java_com_kanaria_UcsString_toLowerCaseNative(
    env: JNIEnv,
    class: jclass,
    target_chars: jcharArray,
    target_chars_len: jint,
    result_chars_buffer: jcharArray,
) -> jint {
    let target = read_target_chars(&env, target_chars, target_chars_len);
    let result = ConverterFactory::from_slice(target.as_slice())
        .lower_case()
        .to_vec();
    write_result_chars(&env, result.as_slice(), result_chars_buffer);
    return result.len() as jint;
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub unsafe extern fn Java_com_kanaria_UcsString_toWideNative(
    env: JNIEnv,
    class: jclass,
    target_chars: jcharArray,
    target_chars_len: jint,
    result_chars_buffer: jcharArray,
) -> jint {
    let target = read_target_chars(&env, target_chars, target_chars_len);
    let result = ConverterFactory::from_slice(target.as_slice())
        .wide()
        .to_vec();
    write_result_chars(&env, result.as_slice(), result_chars_buffer);
    return result.len() as jint;
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub unsafe extern fn Java_com_kanaria_UcsString_toNarrowNative(
    env: JNIEnv,
    class: jclass,
    target_chars: jcharArray,
    target_chars_len: jint,
    result_chars_buffer: jcharArray,
) -> jint {
    let target = read_target_chars(&env, target_chars, target_chars_len);
    let result = ConverterFactory::from_slice(target.as_slice())
        .narrow()
        .to_vec();
    write_result_chars(&env, result.as_slice(), result_chars_buffer);
    return result.len() as jint;
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub unsafe extern fn Java_com_kanaria_UcsString_toHiraganaNative(
    env: JNIEnv,
    class: jclass,
    target_chars: jcharArray,
    target_chars_len: jint,
    result_chars_buffer: jcharArray,
) -> jint {
    let target = read_target_chars(&env, target_chars, target_chars_len);
    let result = ConverterFactory::from_slice(target.as_slice())
        .hiragana()
        .to_vec();
    write_result_chars(&env, result.as_slice(), result_chars_buffer);
    return result.len() as jint;
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub unsafe extern fn Java_com_kanaria_UcsString_toKatakanaNative(
    env: JNIEnv,
    class: jclass,
    target_chars: jcharArray,
    target_chars_len: jint,
    result_chars_buffer: jcharArray,
) -> jint {
    let target = read_target_chars(&env, target_chars, target_chars_len);
    let result = ConverterFactory::from_slice(target.as_slice())
        .katakana()
        .to_vec();
    write_result_chars(&env, result.as_slice(), result_chars_buffer);
    return result.len() as jint;
}

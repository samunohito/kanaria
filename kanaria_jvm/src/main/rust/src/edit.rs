use jni::JNIEnv;
use jni::sys::*;

use kanaria::string::{TextSearch, UCSStr};
use kanaria::utils::CharsUtils;

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub unsafe extern fn Java_com_kanaria_UcsString_isContainsNative(
    env: JNIEnv,
    class: jclass,
    src_chars: jcharArray,
    src_chars_len: jint,
    search_chars: jcharArray,
    search_chars_len: jint,
) -> jboolean {
    let mut src_buffer = Vec::<jchar>::with_capacity(src_chars_len as usize);
    src_buffer.set_len(src_chars_len as usize);
    let _ = env.get_char_array_region(src_chars, 0, src_buffer.as_mut());

    let mut search_buffer = Vec::<jchar>::with_capacity(search_chars_len as usize);
    search_buffer.set_len(src_chars_len as usize);
    let _ = env.get_char_array_region(src_chars, 0, search_buffer.as_mut());

    CharsUtils::is_contains(src_buffer.as_slice(), search_buffer.as_slice()) as jboolean
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub unsafe extern fn Java_com_kanaria_UcsString_indexOfNative(
    env: JNIEnv,
    class: jclass,
    src_chars: jcharArray,
    src_chars_len: jint,
    search_chars: jcharArray,
    search_chars_len: jint,
) -> jint {
    let mut src_buffer = Vec::<jchar>::with_capacity(src_chars_len as usize);
    src_buffer.set_len(src_chars_len as usize);
    let _ = env.get_char_array_region(src_chars, 0, src_buffer.as_mut());

    let mut search_buffer = Vec::<jchar>::with_capacity(search_chars_len as usize);
    search_buffer.set_len(search_chars_len as usize);
    let _ = env.get_char_array_region(search_chars, 0, search_buffer.as_mut());

    CharsUtils::index_of(src_buffer.as_slice(), search_buffer.as_slice()) as jint
}

#[no_mangle]
#[allow(unused_variables, non_snake_case)]
pub unsafe extern fn Java_com_kanaria_UcsString_indexOfAllNative(
    env: JNIEnv,
    class: jclass,
    src_chars: jcharArray,
    src_chars_len: jint,
    search_chars: jcharArray,
    search_chars_len: jint,
) -> jintArray {
    let mut res_res = env.new_int_array(result_buffer.len() as jsize);
    if res_res.is_err() {
        panic!("allocate failed for result.")
    }

    let mut src_buffer = Vec::<jchar>::with_capacity(src_chars_len as usize);
    src_buffer.set_len(src_chars_len as usize);
    let _ = env.get_char_array_region(src_chars, 0, src_buffer.as_mut());

    let mut search_buffer = Vec::<jchar>::with_capacity(search_chars_len as usize);
    search_buffer.set_len(search_chars_len as usize);
    let _ = env.get_char_array_region(search_chars, 0, search_buffer.as_mut());

    let result_buffer = CharsUtils::index_of_all(src_buffer.as_slice(), search_buffer.as_slice())
        .iter()
        .map(|i| *i as jint)
        .collect::<Vec<jint>>();
    let mut result = res_res.unwrap();
    env.set_int_array_region(result, 0, result_buffer.as_slice());

    return result;
}

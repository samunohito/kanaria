use std::char;

use crate::UCSChar;
use crate::converter::Converter;

pub(crate) struct ConverterUtils;

impl ConverterUtils {
    /// サロゲートペアとして分割されていた文字を結合します。
    pub fn concat_surrogate<T>(cur_char_ptr: &T, next_char_ptr: &T) -> u32 where T: UCSChar {
        if cur_char_ptr.is_high_surrogate() {
            if next_char_ptr.is_low_surrogate() {
                0x10000 + (cur_char_ptr.as_scalar() - 0xD800) * 0x400 + (next_char_ptr.as_scalar() - 0xDC00)
            } else if next_char_ptr.is_null() {
                cur_char_ptr.as_scalar()
            } else {
                u32::NULL
            }
        } else if cur_char_ptr.is_low_surrogate() {
            if next_char_ptr.is_null() {
                cur_char_ptr.as_scalar()
            } else {
                u32::NULL
            }
        } else {
            u32::NULL
        }
    }

    /// Unicodeスカラ値を表す値からStringを生成します。
    pub fn build_string<T>(target: Vec<T>) -> String where T: UCSChar {
        let mut result = String::with_capacity(target.len());

        let mut iter = target.iter();
        while let Some(cur_char_ptr) = iter.next() {
            let result_scalar = if cur_char_ptr.is_surrogate() {
                if let Some(next_char_ptr) = iter.next() {
                    Self::concat_surrogate(cur_char_ptr, next_char_ptr)
                } else {
                    u32::NULL
                }
            } else {
                cur_char_ptr.as_scalar()
            };

            if let Some(result_char) = char::from_u32(result_scalar) {
                result.push(result_char);
            }
        }

        return result;
    }

    /// Converter<T>からVec<T>へ値を取り出します。
    pub fn build_vec<T, U>(converter: &U, len: usize) -> Vec<T> where T: UCSChar, U: Converter<T> {
        let mut result = Vec::<T>::with_capacity(len);
        unsafe {
            let real_size = converter.write_to_ptr(result.as_mut_ptr());
            result.set_len(real_size);
        }
        return result;
    }
}
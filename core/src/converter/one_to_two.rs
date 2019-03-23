use std::slice::from_raw_parts;

use crate::UCSChar;
use crate::converter::{Converter, ConverterUtils};

pub struct OneToTwoConverter<T> where T: UCSChar {
    /// 変換対象の文字列への参照を保持します。
    ptr: *const T,
    /// 変換対象文字列の長さを保持します。
    len: usize,
    /// 文字列を変換する式を保持します。
    converter: fn(T) -> (T, T),
}

impl<T> OneToTwoConverter<T> where T: UCSChar {
    /// インスタンスを作成します。
    pub(crate) fn create(ptr: *const T, len: usize, converter: fn(T) -> (T, T)) -> Self {
        Self { ptr, len, converter }
    }
}

impl<T> Converter<T> for OneToTwoConverter<T> where T: UCSChar {
    fn to_string(&mut self) -> String {
        ConverterUtils::build_string(self.to_vec())
    }

    fn to_vec(&mut self) -> Vec<T> {
        ConverterUtils::build_vec(self, self.len * 2)
    }

    unsafe fn write_to_ptr(&self, target_ptr: *mut T) -> usize {
        let mut target_ptr_offset: isize = 0;

        let accessor = from_raw_parts(self.ptr, self.len);
        let converter = self.converter;

        accessor.iter().for_each(|target| {
            let (first_target, second_target) = converter(*target);

            target_ptr.offset(target_ptr_offset).write(first_target);
            target_ptr_offset += 1;

            if !second_target.is_null() {
                // 2つ目の戻り値が0以外の場合は濁音・半濁音が入っているのでポインタに追記する
                target_ptr.offset(target_ptr_offset).write(second_target);
                target_ptr_offset += 1;
            }
        });

        target_ptr_offset as usize
    }
}
use std::slice::from_raw_parts;

use crate::converter::{Converter, ConverterUtils};
use crate::UCSChar;

pub struct OneToOneConverter<T> where T: UCSChar {
    /// 変換対象の文字列への参照を保持します。
    ptr: *const T,
    /// 変換対象文字列の長さを保持します。
    len: usize,
    /// 文字列を変換する式を保持します。
    converter: fn(T) -> T,
}

impl<T> OneToOneConverter<T> where T: UCSChar {
    /// インスタンスを作成します。
    pub(crate) fn create(ptr: *const T, len: usize, converter: fn(T) -> T) -> Self {
        Self { ptr, len, converter }
    }
}

impl<T> Converter<T> for OneToOneConverter<T> where T: UCSChar {
    fn to_string(&mut self) -> String {
        ConverterUtils::build_string(self.to_vec())
    }

    fn to_vec(&mut self) -> Vec<T> {
        ConverterUtils::build_vec(self, self.len)
    }

    unsafe fn write_to_ptr(&self, target_ptr: *mut T) -> usize {
        let mut target_ptr_offset: isize = 0;

        let accessor = from_raw_parts(self.ptr, self.len);
        let converter = self.converter;

        accessor.iter()
            .map(|ptr| converter(*ptr))
            .for_each(|item| {
                target_ptr.offset(target_ptr_offset).write(item);
                target_ptr_offset += 1;
            });

        target_ptr_offset as usize
    }
}
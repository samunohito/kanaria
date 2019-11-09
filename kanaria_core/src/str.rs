use std::char;
use std::slice::from_raw_parts;

use utils::{AsciiUtils, KanaUtils, WidthUtils};

use crate::str::ConvertType::{Hiragana, Katakana, LowerCase, Narrow, None, UpperCase, Wide};
use crate::UCSChar;
use crate::utils::ConvertTarget;

#[derive(Debug)]
#[derive(PartialEq)]
enum ConvertType {
    /// アルファベット大文字に変換します。
    UpperCase,
    /// アルファベット小文字に変換します。
    LowerCase,
    /// ひらがなに変換します。
    Hiragana,
    /// カタカナに変換します。
    Katakana,
    /// 半角文字に変換します。
    Narrow,
    /// 全角文字に変換します。
    Wide,
    /// 変換操作を行いません。
    None,
}

#[derive(Debug)]
struct ConvertParameter {
    /// 変換先の種別を保持します。
    convert_type: ConvertType,
    /// 半角・全角変換の対象を設定します。
    /// 大文字・小文字、ひらがな・かたかな変換時は使用されません。
    width_convert_target: ConvertTarget,
}

impl ConvertParameter {
    /// 変換時のパラメータを設定します。
    pub fn from(convert_type: ConvertType, width_convert_target: ConvertTarget) -> Self {
        Self {
            convert_type,
            width_convert_target,
        }
    }
}

pub struct UCSStr<T> where T: UCSChar {
    /// 変換対象文字列を保持します。
    target: Vec<T>,
    /// 変換時のパラメータを保持します。
    convert_params: Vec<ConvertParameter>,
}

impl UCSStr<u16> {
    /// 変換対象の文字列からUCSStr構造体を生成します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::UCSStr;
    ///
    /// let result = UCSStr::from_str("こんにちは");
    /// ```
    pub fn from_str<U>(s: &U) -> Self where U: AsRef<str> + ?Sized {
        Self {
            target: s.as_ref().encode_utf16().map(|scalar| { u16::from_scalar(scalar) }).collect::<Vec<u16>>(),
            convert_params: vec![],
        }
    }
}

impl<'a, T> UCSStr<T> where T: UCSChar {
    /// 変換対象の文字列からUCSStr構造体を生成します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::UCSStr;
    ///
    /// let target = vec!['あ', 'い', 'う', 'え', 'お'];
    /// let result = UCSStr::from_slice(target.as_slice());
    /// ```
    pub fn from_slice(source: &'a [T]) -> Self {
        Self {
            target: source.to_vec(),
            convert_params: vec![],
        }
    }

    /// 変換対象の文字列からUCSStr構造体を生成します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::UCSStr;
    ///
    /// let target = vec!['あ', 'い', 'う', 'え', 'お'];
    /// let result = unsafe { UCSStr::from_raw(target.as_ptr(), target.len()) };
    /// ```
    pub unsafe fn from_raw(source: *const T, len: usize) -> Self where T: UCSChar {
        Self::from_slice(from_raw_parts(source, len))
    }

    /// 文字列を大文字に変換するように設定します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::UCSStr;
    ///
    /// let target = vec!['a', 'b', 'c'];
    /// let result = UCSStr::from_slice(target.as_slice())
    ///     .upper_case()
    ///     .to_vec();
    ///
    /// assert_eq!(result, vec!['A', 'B', 'C']);
    /// ```
    pub fn upper_case(&mut self) -> &mut Self {
        self.convert_params.push(ConvertParameter::from(UpperCase, ConvertTarget::ALL));
        return self;
    }

    /// 文字列を小文字に変換するように設定します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::UCSStr;
    ///
    /// let target = vec!['A', 'B', 'C'];
    /// let result = UCSStr::from_slice(target.as_slice())
    ///     .lower_case()
    ///     .to_vec();
    ///
    /// assert_eq!(result, vec!['a', 'b', 'c']);
    /// ```
    pub fn lower_case(&mut self) -> &mut Self {
        self.convert_params.push(ConvertParameter::from(LowerCase, ConvertTarget::ALL));
        return self;
    }

    /// 文字列をひらがなに変換するように設定します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::UCSStr;
    ///
    /// let target = vec!['ア', 'イ', 'ウ', 'エ', 'オ'];
    /// let result = UCSStr::from_slice(target.as_slice())
    ///     .hiragana()
    ///     .to_vec();
    ///
    /// assert_eq!(result, vec!['あ', 'い', 'う', 'え', 'お']);
    /// ```
    pub fn hiragana(&mut self) -> &mut Self {
        self.convert_params.push(ConvertParameter::from(Hiragana, ConvertTarget::ALL));
        return self;
    }

    /// 文字列を全角カタカナに変換するように設定します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::UCSStr;
    ///
    /// let target = vec!['あ', 'い', 'う', 'え', 'お'];
    /// let result = UCSStr::from_slice(target.as_slice())
    ///     .katakana()
    ///     .to_vec();
    ///
    /// assert_eq!(result, vec!['ア', 'イ', 'ウ', 'エ', 'オ']);
    /// ```
    pub fn katakana(&mut self) -> &mut Self {
        self.convert_params.push(ConvertParameter::from(Katakana, ConvertTarget::ALL));
        return self;
    }

    /// 文字列を全角に変換するように設定します。
    /// ConvertTargetフラグを利用することにより、全角に変換する文字種別を
    /// 数値、アルファベット、記号、カタカナのいずれか（複数指定可）に限定することが出来ます。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::UCSStr;
    ///
    /// let target = vec!['ﾌ', 'ｼ', 'ﾞ', 'ｻ', 'ﾝ'];
    /// let result = UCSStr::from_slice(target.as_slice())
    ///     .wide(ConvertTarget::ALL)
    ///     .to_vec();
    ///
    /// assert_eq!(result.len(), 4);
    /// assert_eq!(result, vec!['フ','ジ','サ','ン']);
    ///
    /// let target2 = vec!['1', '2', '3', 'a', 'b', 'c'];
    /// let result2 = UCSStr::from_slice(target.as_slice())
    ///     .wide(ConvertTarget::NUMBER)
    ///     .to_vec();
    ///
    /// assert_eq!(result2, vec!['１', '２', '３', 'a', 'b', 'c']);
    /// ```
    pub fn wide(&mut self, width_convert_target: ConvertTarget) -> &mut Self {
        self.convert_params.push(ConvertParameter::from(Wide, width_convert_target));
        return self;
    }

    /// 文字列を半角に変換するように設定します。
    ///　ConvertTargetフラグを利用することにより、全角に変換する文字種別を
    /// 数値、アルファベット、記号、カタカナのいずれか（複数指定可）に限定することが出来ます。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::UCSStr;
    /// use kanaria::utils::ConvertTarget;
    ///
    /// let target = vec!['ガ', 'ギ', 'グ', 'ゲ', 'ゴ'];
    /// let result = UCSStr::from_slice(target.as_slice())
    ///     .narrow(ConvertTarget::ALL)
    ///     .to_vec();
    ///
    /// assert_eq!(result.len(), 10);
    /// assert_eq!(result, vec!['ｶ','ﾞ','ｷ','ﾞ','ｸ','ﾞ','ｹ','ﾞ','ｺ','ﾞ']);
    ///
    /// let target2 = vec!['１', '２', '３', 'Ａ', 'Ｂ', 'Ｃ'];
    /// let result2 = UCSStr::from_slice(target.as_slice())
    ///     .wide(ConvertTarget::ALPHABET)
    ///     .to_vec();
    /// assert_eq!(result2, vec!['１', '２', '３', 'A', 'B', 'C']);
    /// ```
    pub fn narrow(&mut self, width_convert_target: ConvertTarget) -> &mut Self {
        self.convert_params.push(ConvertParameter::from(Narrow, width_convert_target));
        return self;
    }

    /// 文字列を変換せず、そのままとするように設定します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::UCSStr;
    ///
    /// let target = vec!['あ', 'い', 'う', 'え', 'お'];
    /// let result1 = UCSStr::from_slice(target.as_slice())
    ///     .none()
    ///     .to_vec();
    ///
    /// assert_eq!(result1, vec!['あ', 'い', 'う', 'え', 'お']);
    ///
    /// // カタカナに変換後、何もしない設定なのでカタカナに変換された文字が出てくる
    /// let result2 = UCSStr::from_slice(target.as_slice())
    ///     .katakana()
    ///     .none()
    ///     .to_vec();
    ///
    /// assert_eq!(result2, vec!['ア', 'イ', 'ウ', 'エ', 'オ']);
    /// ```
    pub fn none(&mut self) -> &mut Self {
        self.convert_params.push(ConvertParameter::from(None, ConvertTarget::ALL));
        return self;
    }

    /// 文字列を変換し、Stringとして返却します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::UCSStr;
    ///
    /// let target = vec!['あ', 'い', 'う', 'え', 'お'];
    /// let result = UCSStr::from_slice(target.as_slice())
    ///     .katakana()
    ///     .to_string();
    ///
    /// assert_eq!(result.as_str(), "アイウエオ")
    /// ```
    pub fn to_string(&self) -> String {
        let result_vec = self.to_vec();
        let mut result = String::with_capacity(result_vec.len());

        let mut iter = result_vec.iter();
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

    /// 文字列を変換し、Vecとして返却します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::UCSStr;
    ///
    /// let target = vec!['あ', 'い', 'う', 'え', 'お'];
    /// let result = UCSStr::from_slice(target.as_slice())
    ///     .katakana()
    ///     .to_vec();
    ///
    /// assert_eq!(result, vec!['ア', 'イ', 'ウ', 'エ', 'オ'])
    /// ```
    pub fn to_vec(&self) -> Vec<T> {
        let mut buffer = self.target.clone();

        unsafe {
            self.convert_params.iter().for_each(|convert_param| {
//                let mut tmp = if convert_param.convert_type == ConvertType::Narrow {
//                    self.convert_for_narrow(&buffer)
//                } else if convert_param.convert_type == ConvertType::Wide {
//                    self.convert_for_wide(&buffer)
//                } else {
//                    match convert_param.convert_type {
//                        UpperCase => self.convert(&buffer, AsciiUtils::convert_to_upper_case),
//                        LowerCase => self.convert(&buffer, AsciiUtils::convert_to_lower_case),
//                        Hiragana => self.convert(&buffer, KanaUtils::convert_to_hiragana),
//                        Katakana => self.convert(&buffer, KanaUtils::convert_to_katakana),
//                        _ => self.convert(&buffer, |dummy| {
//
//                            dummy
//                        }),
//                    }
//                };

                println!("{:?}", self.convert(&buffer, KanaUtils::convert_to_katakana));
            });
        }

        return buffer;
    }

    /// コンバータを使用して文字列を変換します。
    unsafe fn convert(&self, src: &[T], converter: fn(T) -> T) -> Vec<T> where T: UCSChar {
        let mut dst = Vec::<T>::with_capacity(src.len());
        let dst_ptr = dst.as_mut_ptr();
        let mut dst_ptr_offset: isize = 0;

        src.iter()
            .map(|ptr| converter(*ptr))
            .for_each(|item| {
                println!("{:?}", dst);
                dst_ptr.offset(dst_ptr_offset).write(item);
                dst_ptr_offset += 1;
            });

        return dst;
    }

    /// 半角文字列を全角に変換します。
    unsafe fn convert_for_wide(&self, src: &[T]) -> Vec<T> where T: UCSChar {
        let mut dst = Vec::<T>::with_capacity(src.len());
        let dst_ptr = dst.as_mut_ptr();
        let mut dst_ptr_offset: isize = 0;

        let accessor = from_raw_parts(src.as_ptr(), src.len());
        let mut iter = accessor.iter().enumerate();
        while let Some((index, current_ref)) = iter.next() {
            // 濁音・半濁音の結合処理のため、1つ先の文字も同時に取得する
            let current = *current_ref;
            let next = match accessor.get(index + 1) {
                Some(item_ref) => *item_ref,
                _ => T::NULL,
            };

            let (ret, is_pad) = WidthUtils::convert_to_wide(current, next);
            dst_ptr.offset(dst_ptr_offset).write(ret);
            dst_ptr_offset += 1;

            if is_pad {
                // 結合済みの場合は次を読み飛ばす
                iter.next();
            }
        }

        return dst;
    }

    /// 全角文字列を全角に変換します。
    unsafe fn convert_for_narrow(&self, src: &[T]) -> Vec<T> where T: UCSChar {
        let mut dst = Vec::<T>::with_capacity(src.len() * 2);
        let dst_ptr = dst.as_mut_ptr();
        let mut dst_ptr_offset: isize = 0;

        let accessor = from_raw_parts(src.as_ptr(), src.len());
        accessor.iter().for_each(|target| {
            let (first_char, second_char) = WidthUtils::convert_to_narrow(*target);

            dst_ptr.offset(dst_ptr_offset).write(first_char);
            dst_ptr_offset += 1;

            if !second_char.is_null() {
                // 2つ目の戻り値が0以外の場合は濁音・半濁音が入っているのでポインタに追記する
                dst_ptr.offset(dst_ptr_offset).write(second_char);
                dst_ptr_offset += 1;
            }
        });

        return dst;
    }

    /// サロゲートペアとして分割されていた文字を結合します。
    fn concat_surrogate(cur_char_ptr: &T, next_char_ptr: &T) -> u32 where T: UCSChar {
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
}

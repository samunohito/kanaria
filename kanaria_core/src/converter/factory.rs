use crate::converter::{Converter, OneToOneConverter, OneToTwoConverter, TwoToOneConverter};
use crate::UCSChar;
use crate::utils::{AsciiUtils, KanaUtils, WidthUtils};

pub struct ConverterFactory<T> {
    ptr: *const T,
    len: usize,
}

impl<T> ConverterFactory<T> where T: UCSChar {
    /// 変換対象の文字列からインスタンスを生成します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::converter::{Converter, ConverterFactory};
    ///
    /// let target = vec!['あ', 'い', 'う', 'え', 'お'];
    /// let result = ConverterFactory::from_slice(target.as_slice());
    /// ```
    pub fn from_slice(source: &[T]) -> Self {
        Self {
            ptr: source.as_ptr(),
            len: source.len(),
        }
    }

    /// 変換対象の文字列からインスタンスを生成します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::converter::{Converter, ConverterFactory};
    ///
    /// let target = vec!['あ', 'い', 'う', 'え', 'お'];
    /// let result = unsafe { ConverterFactory::from_raw(target.as_ptr(), target.len()) };
    /// ```
    pub unsafe fn from_raw(source: *const T, len: usize) -> Self where T: UCSChar {
        Self {
            ptr: source,
            len: len,
        }
    }

    /// 変換前と変換後の文字列が一対一になるコンバータを生成します。
    #[inline]
    fn one_to_one(&self, converter: fn(T) -> T) -> impl Converter<T> + '_ {
        OneToOneConverter::create(self.ptr, self.len, converter)
    }

    /// 変換前と変換後の文字列が二対一になるコンバータを生成します。
    #[inline]
    fn two_to_one(&self, converter: fn(T, T) -> (T, bool)) -> impl Converter<T> + '_ {
        TwoToOneConverter::create(self.ptr, self.len, converter)
    }

    /// 変換前と変換後の文字列が二対一になるコンバータを生成します。
    #[inline]
    fn one_to_two(&self, converter: fn(T) -> (T, T)) -> impl Converter<T> + '_ {
        OneToTwoConverter::create(self.ptr, self.len, converter)
    }

    /// 文字列を大文字に変換するコンバータを生成します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::converter::{Converter, ConverterFactory};
    ///
    /// let target = vec!['a', 'b', 'c'];
    /// let converter = ConverterFactory::from_slice(target.as_slice());
    /// let result = converter
    ///     .upper_case()
    ///     .to_vec();
    ///
    /// assert_eq!(result, vec!['A', 'B', 'C']);
    /// ```
    pub fn upper_case(&self) -> impl Converter<T> + '_ {
        self.one_to_one(AsciiUtils::convert_to_upper_case)
    }

    /// 文字列を小文字に変換するコンバータを生成します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::converter::{Converter, ConverterFactory};
    ///
    /// let target = vec!['A', 'B', 'C'];
    /// let converter = ConverterFactory::from_slice(target.as_slice());
    /// let result = converter
    ///     .lower_case()
    ///     .to_vec();
    ///
    /// assert_eq!(result, vec!['a', 'b', 'c']);
    /// ```
    pub fn lower_case(&self) -> impl Converter<T> + '_ {
        self.one_to_one(AsciiUtils::convert_to_lower_case)
    }

    /// 文字列をひらがなに変換するコンバータを生成します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::converter::{Converter, ConverterFactory};
    ///
    /// let target = vec!['ア', 'イ', 'ウ', 'エ', 'オ'];
    /// let converter = ConverterFactory::from_slice(target.as_slice());
    /// let result = converter
    ///     .hiragana()
    ///     .to_vec();
    ///
    /// assert_eq!(result, vec!['あ', 'い', 'う', 'え', 'お']);
    /// ```
    pub fn hiragana(&self) -> impl Converter<T> + '_ {
        self.one_to_one(KanaUtils::convert_to_hiragana)
    }

    /// 文字列を全角カタカナに変換するコンバータを生成します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::converter::{Converter, ConverterFactory};
    ///
    /// let target = vec!['あ', 'い', 'う', 'え', 'お'];
    /// let converter = ConverterFactory::from_slice(target.as_slice());
    /// let result = converter
    ///     .katakana()
    ///     .to_vec();
    ///
    /// assert_eq!(result, vec!['ア', 'イ', 'ウ', 'エ', 'オ']);
    /// ```
    pub fn katakana(&self) -> impl Converter<T> + '_ {
        self.one_to_one(KanaUtils::convert_to_katakana)
    }

    /// 文字列を全角に変換するコンバータを生成します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::converter::{Converter, ConverterFactory};
    ///
    /// let target = vec!['ﾌ', 'ｼ', 'ﾞ', 'ｻ', 'ﾝ'];
    /// let converter = ConverterFactory::from_slice(target.as_slice());
    /// let result = converter
    ///     .wide()
    ///     .to_vec();
    ///
    /// assert_eq!(result.len(), 4);
    /// assert_eq!(result, vec!['フ','ジ','サ','ン']);
    /// ```
    pub fn wide(&self) -> impl Converter<T> + '_ {
        self.two_to_one(WidthUtils::convert_to_wide)
    }

    /// 文字列を半角に変換するコンバータを生成します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::converter::{Converter, ConverterFactory};
    ///
    /// let target = vec!['ガ', 'ギ', 'グ', 'ゲ', 'ゴ'];
    /// let converter = ConverterFactory::from_slice(target.as_slice());
    /// let result = converter
    ///     .narrow()
    ///     .to_vec();
    ///
    /// assert_eq!(result.len(), 10);
    /// assert_eq!(result, vec!['ｶ','ﾞ','ｷ','ﾞ','ｸ','ﾞ','ｹ','ﾞ','ｺ','ﾞ'])
    /// ```
    pub fn narrow(&self) -> impl Converter<T> + '_ {
        self.one_to_two(WidthUtils::convert_to_narrow)
    }

    /// 変換処理を行わないコンバータを生成します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::converter::{Converter, ConverterFactory};
    ///
    /// let target = vec!['あ', 'い', 'う', 'え', 'お'];
    /// let converter = ConverterFactory::from_slice(target.as_slice());
    /// let result = converter
    ///     .none()
    ///     .to_vec();
    ///
    /// assert_eq!(result.len(), 5);
    /// assert_eq!(result, vec!['あ', 'い', 'う', 'え', 'お'])
    /// ```
    pub fn none(&self) -> impl Converter<T> + '_ {
        self.one_to_one(|item| { item })
    }
}
use std::slice::from_raw_parts;

use crate::converter::{Converter, ConverterFactory};
use crate::str::ConvertType::{Hiragana, Katakana, LowerCase, Narrow, None, UpperCase, Wide};
use crate::UCSChar;

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

pub struct UCSStr<T> where T: UCSChar {
    /// 変換対象文字列を保持します。
    target: Vec<T>,
    /// 変換先の種別を保持します。
    convet_type: ConvertType,
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
            convet_type: None,
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
            convet_type: None,
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
    pub fn upper_case(&mut self) -> &Self {
        self.convet_type = UpperCase;
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
    pub fn lower_case(&mut self) -> &Self {
        self.convet_type = LowerCase;
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
    pub fn hiragana(&mut self) -> &Self {
        self.convet_type = Hiragana;
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
    pub fn katakana(&mut self) -> &Self {
        self.convet_type = Katakana;
        return self;
    }

    /// 文字列を全角に変換するように設定します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::UCSStr;
    ///
    /// let target = vec!['ﾌ', 'ｼ', 'ﾞ', 'ｻ', 'ﾝ'];
    /// let result = UCSStr::from_slice(target.as_slice())
    ///     .wide()
    ///     .to_vec();
    ///
    /// assert_eq!(result.len(), 4);
    /// assert_eq!(result, vec!['フ','ジ','サ','ン']);
    /// ```
    pub fn wide(&mut self) -> &Self {
        self.convet_type = Wide;
        return self;
    }

    /// 文字列を半角に変換するように設定します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::UCSStr;
    ///
    /// let target = vec!['ガ', 'ギ', 'グ', 'ゲ', 'ゴ'];
    /// let result = UCSStr::from_slice(target.as_slice())
    ///     .narrow()
    ///     .to_vec();
    ///
    /// assert_eq!(result.len(), 10);
    /// assert_eq!(result, vec!['ｶ','ﾞ','ｷ','ﾞ','ｸ','ﾞ','ｹ','ﾞ','ｺ','ﾞ'])
    /// ```
    pub fn narrow(&mut self) -> &Self {
        self.convet_type = Narrow;
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
        let raw = ConverterFactory::<T>::from_slice(self.target.as_slice());
        match self.convet_type {
            UpperCase => raw.upper_case().to_string(),
            LowerCase => raw.lower_case().to_string(),
            Hiragana => raw.hiragana().to_string(),
            Katakana => raw.katakana().to_string(),
            Wide => raw.wide().to_string(),
            Narrow => raw.narrow().to_string(),
            _ => raw.none().to_string(),
        }
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
        let raw = ConverterFactory::<T>::from_slice(self.target.as_slice());
        match self.convet_type {
            UpperCase => raw.upper_case().to_vec(),
            LowerCase => raw.lower_case().to_vec(),
            Hiragana => raw.hiragana().to_vec(),
            Katakana => raw.katakana().to_vec(),
            Wide => raw.wide().to_vec(),
            Narrow => raw.narrow().to_vec(),
            _ => raw.none().to_vec(),
        }
    }
}

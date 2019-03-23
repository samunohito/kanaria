use std::char;
use std::fmt::Debug;

pub trait UCSChar: Debug + Sized + Default + Copy + Ord + Eq {
    /// NULLを表します。
    const NULL: Self;

    /// 対象がlow以上high以下かどうかを判定します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::UCSChar;
    /// use kanaria::constants::*;
    ///
    /// // (0x0041 >= 0x0041) && (0x0041 <= 0x005A)
    /// let true_case = 'A'.is_in_range(NARROW_ASCII_LATIN_UPPER_CASE_A, NARROW_ASCII_LATIN_UPPER_CASE_Z);
    /// assert_eq!(true_case, true);
    ///
    /// // (0x0061 >= 0x0041) && (0x0061 <= 0x005A)
    /// let false_case = 'a'.is_in_range(NARROW_ASCII_LATIN_UPPER_CASE_A, NARROW_ASCII_LATIN_UPPER_CASE_Z);
    /// assert_eq!(false_case, false);
    /// ```
    #[inline]
    fn is_in_range(&self, low: u32, high: u32) -> bool;

    /// searchの中に対象を含むかどうかを判定します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::UCSChar;
    /// use kanaria::constants::*;
    ///
    /// let true_case = 'A'.is_contains(&[NARROW_ASCII_LATIN_UPPER_CASE_A, NARROW_ASCII_LATIN_UPPER_CASE_B, NARROW_ASCII_LATIN_UPPER_CASE_C]);
    /// assert_eq!(true_case, true);
    ///
    /// let false_case = 'D'.is_contains(&[NARROW_ASCII_LATIN_UPPER_CASE_A, NARROW_ASCII_LATIN_UPPER_CASE_B, NARROW_ASCII_LATIN_UPPER_CASE_C]);
    /// assert_eq!(false_case, false);
    /// ```
    #[inline]
    fn is_contains(&self, search: &[u32]) -> bool;

    /// targetと対象が同値であるかどうかを判定します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::UCSChar;
    /// use kanaria::constants::*;
    ///
    /// let true_case = 'A'.is_match(NARROW_ASCII_LATIN_UPPER_CASE_A);
    /// assert_eq!(true_case, true);
    ///
    /// let false_case = 'A'.is_match(NARROW_ASCII_LATIN_LOWER_CASE_A);
    /// assert_eq!(false_case, false);
    /// ```
    #[inline]
    fn is_match(&self, target: u32) -> bool;

    /// サロゲートペアであるかどうかを判定します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::UCSChar;
    ///
    /// // 𩸽(ほっけ)の上位サロゲート
    /// assert_eq!(0xD867u16.is_surrogate(), true);
    ///
    /// // 𩸽(ほっけ)の下位サロゲート
    /// assert_eq!(0xDE3Du16.is_surrogate(), true);
    ///
    /// // ひらがなの「あ」
    /// assert_eq!(0x3042u16.is_surrogate(), false);
    /// ```
    #[inline]
    fn is_surrogate(&self) -> bool;

    /// 上位サロゲートであるかどうかを判定します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::UCSChar;
    ///
    /// // 𩸽(ほっけ)の上位サロゲート
    /// assert_eq!(0xD867u16.is_high_surrogate(), true);
    ///
    /// // 𩸽(ほっけ)の下位サロゲート
    /// assert_eq!(0xDE3Du16.is_high_surrogate(), false);
    ///
    /// // ひらがなの「あ」
    /// assert_eq!(0x3042u16.is_high_surrogate(), false);
    /// ```
    #[inline]
    fn is_high_surrogate(&self) -> bool;

    /// 下位サロゲートであるかどうかを判定します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::UCSChar;
    ///
    /// // 𩸽(ほっけ)の上位サロゲート
    /// assert_eq!(0xD867u16.is_low_surrogate(), false);
    ///
    /// // 𩸽(ほっけ)の下位サロゲート
    /// assert_eq!(0xDE3Du16.is_low_surrogate(), true);
    ///
    /// // ひらがなの「あ」
    /// assert_eq!(0x3042u16.is_low_surrogate(), false);
    /// ```
    #[inline]
    fn is_low_surrogate(&self) -> bool;

    /// null文字であるかどうかを判定します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::UCSChar;
    ///
    /// assert_eq!(char::NULL.is_null(), true);
    /// assert_eq!('a'.is_null(), false);
    /// ```
    #[inline]
    fn is_null(&self) -> bool;

    /// 対象にvalueの値を加算した文字を取得します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::UCSChar;
    /// use kanaria::constants::*;
    ///
    /// let ret = 'あ'.addition(0x0060);
    /// assert_eq!(ret, 'ア');
    /// ```
    #[inline]
    fn addition(&self, value: u32) -> Self;

    /// 対象からvalueの値を減算した文字を取得します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::UCSChar;
    /// use kanaria::constants::*;
    ///
    /// let ret = 'ア'.subtraction(0x0060);
    /// assert_eq!(ret, 'あ');
    /// ```
    #[inline]
    fn subtraction(&self, value: u32) -> Self;

    /// 値をUnicodeスカラ値に変換します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::UCSChar;
    /// use kanaria::constants::*;
    ///
    /// let ret = 'A'.as_scalar();
    /// assert_eq!(ret, NARROW_ASCII_LATIN_UPPER_CASE_A);
    /// ```
    #[inline]
    fn as_scalar(&self) -> u32;

    /// Unicodeスカラ値から値を取り込みます。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::UCSChar;
    /// use kanaria::constants::*;
    ///
    /// let ret = char::from_scalar(NARROW_ASCII_LATIN_UPPER_CASE_A);
    /// assert_eq!(ret, 'A');
    /// ```
    #[inline]
    fn from_scalar<T>(value: T) -> Self where T: UCSChar;
}

impl UCSChar for char {
    const NULL: char = 0 as char;

    #[inline]
    fn is_in_range(&self, low: u32, high: u32) -> bool {
        let target_trans = *self as u32;
        target_trans >= low && target_trans <= high
    }

    #[inline]
    fn is_contains(&self, search: &[u32]) -> bool {
        let target_trans = *self as u32;
        search.contains(&target_trans)
    }

    #[inline]
    fn is_match(&self, target: u32) -> bool {
        (*self as u32) == target
    }

    #[inline]
    fn is_surrogate(&self) -> bool {
        Self::is_high_surrogate(self) || Self::is_low_surrogate(self)
    }

    #[inline]
    fn is_high_surrogate(&self) -> bool {
        let scalar = self.as_scalar();
        0xD800 <= scalar && scalar < 0xDC00
    }

    #[inline]
    fn is_low_surrogate(&self) -> bool {
        let scalar = self.as_scalar();
        0xDC00 <= scalar && scalar < 0xE000
    }

    #[inline]
    fn is_null(&self) -> bool {
        *self == Self::NULL
    }

    #[inline]
    fn addition(&self, value: u32) -> Self {
        unsafe { char::from_u32_unchecked((*self as u32) + value) }
    }

    #[inline]
    fn subtraction(&self, value: u32) -> Self {
        unsafe { char::from_u32_unchecked((*self as u32) - value) }
    }

    #[inline]
    fn as_scalar(&self) -> u32 {
        *self as u32
    }

    #[inline]
    fn from_scalar<T>(value: T) -> Self where T: UCSChar {
        unsafe { char::from_u32_unchecked(value.as_scalar()) }
    }
}

impl UCSChar for u16 {
    const NULL: u16 = 0;

    #[inline]
    fn is_in_range(&self, low: u32, high: u32) -> bool {
        let target_trans = *self as u32;
        target_trans >= low && target_trans <= high
    }

    #[inline]
    fn is_contains(&self, search: &[u32]) -> bool {
        let target_trans = *self as u32;
        search.contains(&target_trans)
    }

    #[inline]
    fn is_match(&self, target: u32) -> bool {
        (*self as u32) == target
    }

    #[inline]
    fn is_surrogate(&self) -> bool {
        Self::is_high_surrogate(self) || Self::is_low_surrogate(self)
    }

    #[inline]
    fn is_high_surrogate(&self) -> bool {
        let scalar = self.as_scalar();
        0xD800 <= scalar && scalar < 0xDC00
    }

    #[inline]
    fn is_low_surrogate(&self) -> bool {
        let scalar = self.as_scalar();
        0xDC00 <= scalar && scalar < 0xE000
    }

    #[inline]
    fn is_null(&self) -> bool {
        *self == Self::NULL
    }

    #[inline]
    fn addition(&self, value: u32) -> Self {
        ((*self as u32) + value) as u16
    }

    #[inline]
    fn subtraction(&self, value: u32) -> Self {
        ((*self as u32) - value) as u16
    }

    #[inline]
    fn as_scalar(&self) -> u32 {
        *self as u32
    }

    #[inline]
    fn from_scalar<T>(value: T) -> Self where T: UCSChar {
        value.as_scalar() as u16
    }
}

impl UCSChar for u32 {
    const NULL: u32 = 0;

    #[inline]
    fn is_in_range(&self, low: u32, high: u32) -> bool {
        let target_trans = *self;
        target_trans >= low && target_trans <= high
    }

    #[inline]
    fn is_contains(&self, search: &[u32]) -> bool {
        search.contains(self)
    }

    #[inline]
    fn is_match(&self, target: u32) -> bool {
        (*self) == target
    }

    #[inline]
    fn is_surrogate(&self) -> bool {
        Self::is_high_surrogate(self) || Self::is_low_surrogate(self)
    }

    #[inline]
    fn is_high_surrogate(&self) -> bool {
        let scalar = self.as_scalar();
        0xD800 <= scalar && scalar < 0xDC00
    }

    #[inline]
    fn is_low_surrogate(&self) -> bool {
        let scalar = self.as_scalar();
        0xDC00 <= scalar && scalar < 0xE000
    }

    #[inline]
    fn is_null(&self) -> bool {
        *self == Self::NULL
    }

    #[inline]
    fn addition(&self, value: u32) -> Self {
        ((*self) + value) as u32
    }

    #[inline]
    fn subtraction(&self, value: u32) -> Self {
        ((*self) - value) as u32
    }

    #[inline]
    fn as_scalar(&self) -> u32 {
        *self
    }

    #[inline]
    fn from_scalar<T>(value: T) -> Self where T: UCSChar {
        value.as_scalar()
    }
}
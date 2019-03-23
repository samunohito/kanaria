use crate::UCSChar;
use crate::constants::*;

const UNICODE_OFFSET_ALPHABET_UPPER_LOWER: u32 = 0x0020;

pub struct AsciiUtils;

impl AsciiUtils {
    /// ASCII文字かどうかを判定します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::utils::AsciiUtils;
    ///
    /// assert_eq!(AsciiUtils::is_ascii('a'), true);
    /// assert_eq!(AsciiUtils::is_ascii('A'), true);
    /// assert_eq!(AsciiUtils::is_ascii('@'), true);
    /// assert_eq!(AsciiUtils::is_ascii('0'), true);
    /// assert_eq!(AsciiUtils::is_ascii('ａ'), true);
    /// assert_eq!(AsciiUtils::is_ascii('Ａ'), true);
    /// assert_eq!(AsciiUtils::is_ascii('＠'), true);
    /// assert_eq!(AsciiUtils::is_ascii('０'), true);
    ///
    /// assert_eq!(AsciiUtils::is_ascii('あ'), false);
    /// ```
    #[inline]
    pub fn is_ascii<T>(target: T) -> bool where T: UCSChar {
        AsciiUtils::is_narrow_ascii(target) || AsciiUtils::is_wide_ascii(target)
    }

    /// 半角ASCII文字かどうかを判定します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::utils::AsciiUtils;
    ///
    /// assert_eq!(AsciiUtils::is_narrow_ascii('a'), true);
    /// assert_eq!(AsciiUtils::is_narrow_ascii('A'), true);
    /// assert_eq!(AsciiUtils::is_narrow_ascii('@'), true);
    /// assert_eq!(AsciiUtils::is_narrow_ascii('0'), true);
    ///
    /// assert_eq!(AsciiUtils::is_narrow_ascii('ａ'), false);
    /// assert_eq!(AsciiUtils::is_narrow_ascii('Ａ'), false);
    /// assert_eq!(AsciiUtils::is_narrow_ascii('＠'), false);
    /// assert_eq!(AsciiUtils::is_narrow_ascii('０'), false);
    /// ```
    #[inline]
    pub fn is_narrow_ascii<T>(target: T) -> bool where T: UCSChar {
        AsciiUtils::is_narrow_lower_case(target) ||
            AsciiUtils::is_narrow_upper_case(target) ||
            AsciiUtils::is_narrow_ascii_symbol(target) ||
            AsciiUtils::is_narrow_number(target)
    }

    /// 全角ASCII文字かどうかを判定します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::utils::AsciiUtils;
    ///
    /// assert_eq!(AsciiUtils::is_wide_ascii('ａ'), true);
    /// assert_eq!(AsciiUtils::is_wide_ascii('Ａ'), true);
    /// assert_eq!(AsciiUtils::is_wide_ascii('＠'), true);
    /// assert_eq!(AsciiUtils::is_wide_ascii('０'), true);
    ///
    /// assert_eq!(AsciiUtils::is_wide_ascii('a'), false);
    /// assert_eq!(AsciiUtils::is_wide_ascii('A'), false);
    /// assert_eq!(AsciiUtils::is_wide_ascii('@'), false);
    /// assert_eq!(AsciiUtils::is_wide_ascii('0'), false);
    /// ```
    #[inline]
    pub fn is_wide_ascii<T>(target: T) -> bool where T: UCSChar {
        AsciiUtils::is_wide_lower_case(target) ||
            AsciiUtils::is_wide_upper_case(target) ||
            AsciiUtils::is_wide_ascii_symbol(target) ||
            AsciiUtils::is_wide_number(target)
    }

    /// 英字の大文字かどうかを判定します。
    /// 半角・全角は区別しません。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::utils::AsciiUtils;
    ///
    /// assert_eq!(AsciiUtils::is_lower_case('a'), true);
    /// assert_eq!(AsciiUtils::is_lower_case('ａ'), true);
    ///
    /// assert_eq!(AsciiUtils::is_lower_case('Ａ'), false);
    /// assert_eq!(AsciiUtils::is_lower_case('A'), false);
    /// ```
    #[inline]
    pub fn is_lower_case<T>(target: T) -> bool where T: UCSChar {
        AsciiUtils::is_narrow_lower_case(target) || AsciiUtils::is_wide_lower_case(target)
    }

    /// 半角英字の大文字かどうかを判定します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::utils::AsciiUtils;
    ///
    /// assert_eq!(AsciiUtils::is_narrow_lower_case('a'), true);
    ///
    /// assert_eq!(AsciiUtils::is_narrow_lower_case('ａ'), false);
    /// assert_eq!(AsciiUtils::is_narrow_lower_case('Ａ'), false);
    /// assert_eq!(AsciiUtils::is_narrow_lower_case('A'), false);
    /// ```
    #[inline]
    pub fn is_narrow_lower_case<T>(target: T) -> bool where T: UCSChar {
        target.is_in_range(NARROW_ASCII_LATIN_LOWER_CASE_A, NARROW_ASCII_LATIN_LOWER_CASE_Z)
    }

    /// 全角英字の小文字かどうかを判定します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::utils::AsciiUtils;
    ///
    /// assert_eq!(AsciiUtils::is_wide_lower_case('ａ'), true);
    ///
    /// assert_eq!(AsciiUtils::is_wide_lower_case('a'), false);
    /// assert_eq!(AsciiUtils::is_wide_lower_case('Ａ'), false);
    /// assert_eq!(AsciiUtils::is_wide_lower_case('A'), false);
    /// ```
    #[inline]
    pub fn is_wide_lower_case<T>(target: T) -> bool where T: UCSChar {
        target.is_in_range(WIDE_ASCII_LATIN_LOWER_CASE_A, WIDE_ASCII_LATIN_LOWER_CASE_Z)
    }

    /// 英字の大文字かどうかを判定します。
    /// 半角・全角は区別しません。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::utils::AsciiUtils;
    ///
    /// assert_eq!(AsciiUtils::is_upper_case('A'), true);
    /// assert_eq!(AsciiUtils::is_upper_case('Ａ'), true);
    ///
    /// assert_eq!(AsciiUtils::is_upper_case('１'), false);
    /// assert_eq!(AsciiUtils::is_upper_case('1'), false);
    /// ```
    #[inline]
    pub fn is_upper_case<T>(target: T) -> bool where T: UCSChar {
        AsciiUtils::is_narrow_upper_case(target) || AsciiUtils::is_wide_upper_case(target)
    }

    /// 半角英字の大文字かどうかを判定します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::utils::AsciiUtils;
    ///
    /// assert_eq!(AsciiUtils::is_narrow_upper_case('A'), true);
    ///
    /// assert_eq!(AsciiUtils::is_narrow_upper_case('Ａ'), false);
    /// assert_eq!(AsciiUtils::is_narrow_upper_case('ａ'), false);
    /// assert_eq!(AsciiUtils::is_narrow_upper_case('a'), false);
    /// ```
    #[inline]
    pub fn is_narrow_upper_case<T>(target: T) -> bool where T: UCSChar {
        target.is_in_range(NARROW_ASCII_LATIN_UPPER_CASE_A, NARROW_ASCII_LATIN_UPPER_CASE_Z)
    }

    /// 全角英字の大文字かどうかを判定します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::utils::AsciiUtils;
    ///
    /// assert_eq!(AsciiUtils::is_wide_upper_case('Ａ'), true);
    ///
    /// assert_eq!(AsciiUtils::is_wide_upper_case('A'), false);
    /// assert_eq!(AsciiUtils::is_wide_upper_case('ａ'), false);
    /// assert_eq!(AsciiUtils::is_wide_upper_case('a'), false);
    /// ```
    #[inline]
    pub fn is_wide_upper_case<T>(target: T) -> bool where T: UCSChar {
        target.is_in_range(WIDE_ASCII_LATIN_UPPER_CASE_A, WIDE_ASCII_LATIN_UPPER_CASE_Z)
    }

    /// 記号かどうかを判定します。
    /// 半角・全角は区別しません。
    /// なお、ASCIIコードに属する記号のみを判定します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::utils::AsciiUtils;
    ///
    /// assert_eq!(AsciiUtils::is_ascii_symbol('@'), true);
    /// assert_eq!(AsciiUtils::is_ascii_symbol('＠'), true);
    ///
    /// // かな文字で使用されるような記号は半角・全角問わずtrueとならない（例は濁音記号）
    /// assert_eq!(AsciiUtils::is_ascii_symbol('゙'), false);
    /// assert_eq!(AsciiUtils::is_ascii_symbol('ﾞ'), false);
    /// ```
    #[inline]
    pub fn is_ascii_symbol<T>(target: T) -> bool where T: UCSChar {
        AsciiUtils::is_narrow_ascii_symbol(target) || AsciiUtils::is_wide_ascii_symbol(target)
    }

    /// 半角記号かどうかを判定します。
    /// なお、ASCIIコードに属する記号のみを判定します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::utils::AsciiUtils;
    ///
    /// assert_eq!(AsciiUtils::is_narrow_ascii_symbol('@'), true);
    ///
    /// assert_eq!(AsciiUtils::is_narrow_ascii_symbol('＠'), false);
    ///
    /// // かな文字で使用されるような記号は半角・全角問わずtrueとならない（例は濁音記号）
    /// assert_eq!(AsciiUtils::is_narrow_ascii_symbol('゙'), false);
    /// assert_eq!(AsciiUtils::is_narrow_ascii_symbol('ﾞ'), false);
    /// ```
    #[inline]
    pub fn is_narrow_ascii_symbol<T>(target: T) -> bool where T: UCSChar {
        target.is_in_range(NARROW_ASCII_SYMBOL_SPACE, NARROW_ASCII_SYMBOL_SOLIDUS) ||
            target.is_in_range(NARROW_ASCII_SYMBOL_COLON, NARROW_ASCII_SYMBOL_COMMERCIAL_AT) ||
            target.is_in_range(NARROW_ASCII_SYMBOL_LEFT_SQUARE_BRACKET, NARROW_ASCII_SYMBOL_GRAVE_ACCENT) ||
            target.is_in_range(NARROW_ASCII_SYMBOL_LEFT_CURLY_BRACKET, NARROW_ASCII_SYMBOL_TILDE)
    }

    /// 全角記号かどうかを判定します。
    /// なお、ASCIIコードに属する記号のみを判定します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::utils::AsciiUtils;
    ///
    /// assert_eq!(AsciiUtils::is_wide_ascii_symbol('＠'), true);
    ///
    /// assert_eq!(AsciiUtils::is_wide_ascii_symbol('@'), false);
    ///
    /// // かな文字で使用されるような記号は半角・全角問わずtrueとならない（例は濁音記号）
    /// assert_eq!(AsciiUtils::is_wide_ascii_symbol('゙'), false);
    /// assert_eq!(AsciiUtils::is_wide_ascii_symbol('ﾞ'), false);
    /// ```
    #[inline]
    pub fn is_wide_ascii_symbol<T>(target: T) -> bool where T: UCSChar {
        target.is_in_range(WIDE_ASCII_SYMBOL_EXCLAMATION_MARK, WIDE_ASCII_SYMBOL_SOLIDUS) ||
            target.is_in_range(WIDE_ASCII_SYMBOL_COLON, WIDE_ASCII_SYMBOL_COMMERCIAL_AT) ||
            target.is_in_range(WIDE_ASCII_SYMBOL_LEFT_SQUARE_BRACKET, WIDE_ASCII_SYMBOL_GRAVE_ACCENT) ||
            target.is_in_range(WIDE_ASCII_SYMBOL_LEFT_CURLY_BRACKET, WIDE_ASCII_SYMBOL_TILDE) ||
            target.is_match(WIDE_ASCII_SYMBOL_SPACE)
    }

    /// 数字かどうかを判定します。
    /// 半角・全角は区別しません。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::utils::AsciiUtils;
    ///
    /// assert_eq!(AsciiUtils::is_number('1'), true);
    /// assert_eq!(AsciiUtils::is_number('１'), true);
    ///
    /// assert_eq!(AsciiUtils::is_number('A'), false);
    /// assert_eq!(AsciiUtils::is_number('Ａ'), false);
    /// ```
    #[inline]
    pub fn is_number<T>(target: T) -> bool where T: UCSChar {
        AsciiUtils::is_narrow_number(target) || AsciiUtils::is_wide_number(target)
    }

    /// 半角数字かどうかを判定します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::utils::AsciiUtils;
    ///
    /// assert_eq!(AsciiUtils::is_narrow_number('1'), true);
    ///
    /// assert_eq!(AsciiUtils::is_narrow_number('１'), false);
    /// assert_eq!(AsciiUtils::is_narrow_number('A'), false);
    /// assert_eq!(AsciiUtils::is_narrow_number('Ａ'), false);
    /// ```
    #[inline]
    pub fn is_narrow_number<T>(target: T) -> bool where T: UCSChar {
        target.is_in_range(NARROW_ASCII_NUMBER_0, NARROW_ASCII_NUMBER_9)
    }

    /// 全角数字かどうかを判定します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::utils::AsciiUtils;
    ///
    /// assert_eq!(AsciiUtils::is_wide_number('１'), true);
    ///
    /// assert_eq!(AsciiUtils::is_wide_number('1'), false);
    /// assert_eq!(AsciiUtils::is_wide_number('A'), false);
    /// assert_eq!(AsciiUtils::is_wide_number('Ａ'), false);
    /// ```
    #[inline]
    pub fn is_wide_number<T>(target: T) -> bool where T: UCSChar {
        target.is_in_range(WIDE_ASCII_NUMBER_0, WIDE_ASCII_NUMBER_9)
    }

    /// 小文字を大文字に変換します。
    /// 半角・全角は区別しません。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::utils::AsciiUtils;
    ///
    /// let ret = AsciiUtils::convert_to_upper_case('a');
    /// assert_eq!(ret, 'A');
    /// ```
    #[inline]
    pub fn convert_to_upper_case<T>(target_char: T) -> T where T: UCSChar {
        if AsciiUtils::is_lower_case(target_char) {
            target_char.subtraction(UNICODE_OFFSET_ALPHABET_UPPER_LOWER)
        } else {
            target_char
        }
    }

    /// 大文字を小文字に変換します。
    /// 半角・全角は区別しません。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::utils::AsciiUtils;
    ///
    /// let ret = AsciiUtils::convert_to_lower_case('A');
    /// assert_eq!(ret, 'a');
    /// ```
    #[inline]
    pub fn convert_to_lower_case<T>(target_char: T) -> T where T: UCSChar {
        if AsciiUtils::is_upper_case(target_char) {
            target_char.addition(UNICODE_OFFSET_ALPHABET_UPPER_LOWER)
        } else {
            target_char
        }
    }
}
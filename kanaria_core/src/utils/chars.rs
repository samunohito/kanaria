use UCSChar;
use utils::{AsciiUtils, KanaUtils};

pub struct CharsUtils;

impl CharsUtils {
    /// 半角文字かどうかを判定します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::utils::CharsUtils;
    ///
    /// assert_eq!(CharsUtils::is_narrow('a'), true);
    /// assert_eq!(CharsUtils::is_narrow('A'), true);
    /// assert_eq!(CharsUtils::is_narrow('@'), true);
    /// assert_eq!(CharsUtils::is_narrow('0'), true);
    /// assert_eq!(CharsUtils::is_narrow('ｱ'), true);
    /// assert_eq!(CharsUtils::is_narrow('ｰ'), true);
    ///
    /// assert_eq!(CharsUtils::is_narrow('ａ'), false);
    /// assert_eq!(CharsUtils::is_narrow('Ａ'), false);
    /// assert_eq!(CharsUtils::is_narrow('＠'), false);
    /// assert_eq!(CharsUtils::is_narrow('０'), false);
    /// assert_eq!(CharsUtils::is_narrow('ア'), false);
    /// assert_eq!(CharsUtils::is_narrow('ー'), false);
    /// ```
    #[inline]
    pub fn is_narrow<T>(target: T) -> bool where T: UCSChar {
        AsciiUtils::is_narrow_ascii(target) ||
            KanaUtils::is_narrow_katakana(target) ||
            KanaUtils::is_narrow_jis_symbol(target)
    }

    /// 全角文字かどうかを判定します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::utils::CharsUtils;
    ///
    /// assert_eq!(CharsUtils::is_wide('ａ'), true);
    /// assert_eq!(CharsUtils::is_wide('Ａ'), true);
    /// assert_eq!(CharsUtils::is_wide('＠'), true);
    /// assert_eq!(CharsUtils::is_wide('０'), true);
    /// assert_eq!(CharsUtils::is_wide('ア'), true);
    /// assert_eq!(CharsUtils::is_wide('ー'), true);
    /// 
    /// assert_eq!(CharsUtils::is_wide('a'), false);
    /// assert_eq!(CharsUtils::is_wide('A'), false);
    /// assert_eq!(CharsUtils::is_wide('@'), false);
    /// assert_eq!(CharsUtils::is_wide('0'), false);
    /// assert_eq!(CharsUtils::is_wide('ｱ'), false);
    /// assert_eq!(CharsUtils::is_wide('ｰ'), false);
    ///
    /// ```
    #[inline]
    pub fn is_wide<T>(target: T) -> bool where T: UCSChar {
        AsciiUtils::is_wide_ascii(target) ||
            KanaUtils::is_wide_katakana(target) ||
            KanaUtils::is_wide_jis_symbol(target)
    }
}
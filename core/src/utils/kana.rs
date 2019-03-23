use crate::constants::*;
use crate::UCSChar;

/// かな<->カナ変換で使用します。
/// この数値を加減算することでそれぞれを相互変換します。
const UNICODE_OFFSET_HIRAGANA_ZENKAKU_KATAKANA: u32 = 0x0060;

pub struct KanaUtils;

impl KanaUtils {
    /// 対象がひらがなかどうかを判定します。
    /// 対象が下記のいずれかに該当するUnicode文字であればtrueになります。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::utils::KanaUtils;
    /// use kanaria::UCSChar;
    ///
    /// let true_case = KanaUtils::is_hiragana('あ');
    /// assert_eq!(true_case, true);
    ///
    /// let false_case = KanaUtils::is_hiragana('ア');
    /// assert_eq!(false_case, false);
    /// ```
    #[inline]
    pub fn is_hiragana<T>(target: T) -> bool where T: UCSChar {
        target.is_in_range(WIDE_KANA_HIRAGANA_SMALL_A, WIDE_KANA_HIRAGANA_SMALL_KE) ||
            target.is_in_range(WIDE_KANA_HIRAGANA_ITERATION_MARK, WIDE_KANA_HIRAGANA_DIGRAPH_YORI)
    }

    /// 対象がカタカナかどうかを判定します。半角・全角は区別しません。
    /// 対象が下記のいずれかに該当するUnicode文字であればtrueになります。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::utils::KanaUtils;
    /// use kanaria::UCSChar;
    ///
    /// let true_case1 = KanaUtils::is_katakana('ア');
    /// assert_eq!(true_case1, true);
    /// let true_case2 = KanaUtils::is_katakana('ｱ');
    /// assert_eq!(true_case2, true);
    ///
    /// let false_case = KanaUtils::is_katakana('あ');
    /// assert_eq!(false_case, false);
    /// ```
    #[inline]
    pub fn is_katakana<T>(target: T) -> bool where T: UCSChar {
        KanaUtils::is_narrow_katakana(target) || KanaUtils::is_wide_katakana(target)
    }

    /// 対象が半角カタカナかどうかを判定します。
    /// 対象が下記のいずれかに該当するUnicode文字であればtrueになります。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::utils::KanaUtils;
    /// use kanaria::UCSChar;
    ///
    /// let true_case = KanaUtils::is_narrow_katakana('ｱ');
    /// assert_eq!(true_case, true);
    ///
    /// let false_case = KanaUtils::is_narrow_katakana('ア');
    /// assert_eq!(false_case, false);
    /// ```
    #[inline]
    pub fn is_narrow_katakana<T>(target: T) -> bool where T: UCSChar {
        target.is_in_range(NARROW_KANA_KATAKANA_WO, NARROW_KANA_KATAKANA_SMALL_TU) ||
            target.is_in_range(NARROW_KANA_KATAKANA_A, NARROW_KANA_KATAKANA_N)
    }

    /// 対象が全角カタカナかどうかを判定します。
    /// 対象が下記のいずれかに該当するUnicode文字であればtrueになります。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::utils::KanaUtils;
    /// use kanaria::UCSChar;
    ///
    /// let true_case = KanaUtils::is_wide_katakana('ア');
    /// assert_eq!(true_case, true);
    ///
    /// let false_case = KanaUtils::is_wide_katakana('ｱ');
    /// assert_eq!(false_case, false);
    /// ```
    #[inline]
    pub fn is_wide_katakana<T>(target: T) -> bool where T: UCSChar {
        target.is_in_range(WIDE_KANA_KATAKANA_SMALL_A, WIDE_KANA_KATAKANA_VO) ||
            target.is_in_range(WIDE_KANA_KATAKANA_ITERATION_MARK, WIDE_KANA_KATAKANA_DIGRAPH_YORI)
    }

    /// 対象が半角記号かどうかを判定します。ASCIIコード内の記号はtrueになりません。
    /// 対象が下記のいずれかに該当するUnicode文字であればtrueになります。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::utils::KanaUtils;
    /// use kanaria::UCSChar;
    ///
    /// let true_case1 = KanaUtils::is_jis_symbol('ｰ');
    /// assert_eq!(true_case1, true);
    /// let true_case2 = KanaUtils::is_jis_symbol('ー');
    /// assert_eq!(true_case2, true);
    ///
    /// let false_case = KanaUtils::is_jis_symbol('あ');
    /// assert_eq!(false_case, false);
    /// ```
    #[inline]
    pub fn is_jis_symbol<T: UCSChar>(target: T) -> bool {
        KanaUtils::is_narrow_jis_symbol(target) || KanaUtils::is_wide_jis_symbol(target)
    }

    /// 対象が半角記号かどうかを判定します。ASCIIコード内の記号はtrueになりません。
    /// 対象が下記のいずれかに該当するUnicode文字であればtrueになります。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::utils::KanaUtils;
    /// use kanaria::UCSChar;
    ///
    /// let true_case = KanaUtils::is_narrow_jis_symbol('ｰ');
    /// assert_eq!(true_case, true);
    ///
    /// let false_case = KanaUtils::is_narrow_jis_symbol('ー');
    /// assert_eq!(false_case, false);
    /// ```
    #[inline]
    pub fn is_narrow_jis_symbol<T>(target: T) -> bool where T: UCSChar {
        target.is_in_range(NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK, NARROW_JIS_SYMBOL_KATAKANA_SEMI_VOICED_SOUND_MARK) ||
            target.is_contains(&[NARROW_JIS_SYMBOL_KATAKANA_MIDDLE_DOT, NARROW_JIS_SYMBOL_PROLONGED_SOUND_MARK])
    }

    /// 対象が全角記号かどうかを判定します。ASCIIコード内の記号はtrueになりません。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::utils::KanaUtils;
    /// use kanaria::UCSChar;
    ///
    /// let true_case = KanaUtils::is_wide_jis_symbol('ー');
    /// assert_eq!(true_case, true);
    ///
    /// let false_case = KanaUtils::is_wide_jis_symbol('ｰ');
    /// assert_eq!(false_case, false);
    /// ```
    #[inline]
    pub fn is_wide_jis_symbol<T>(target: T) -> bool where T: UCSChar {
        target.is_in_range(WIDE_JIS_SYMBOL_KATAKANA_SEMI_VOICED_SOUND_MARK, WIDE_JIS_SYMBOL_KATAKANA_SEMI_VOICED_SOUND_MARK) ||
            target.is_in_range(WIDE_KANA_HIRAGANA_ITERATION_MARK, WIDE_KANA_HIRAGANA_VOICED_ITERATION_MARK) ||
            target.is_in_range(WIDE_KANA_KATAKANA_ITERATION_MARK, WIDE_KANA_KATAKANA_VOICED_ITERATION_MARK) ||
            target.is_contains(&[
                WIDE_JIS_SYMBOL_MIDDLE_DOT,
                WIDE_JIS_SYMBOL_MIDDLE_DOT,
                WIDE_JIS_SYMBOL_KATAKANA_MIDDLE_DOT,
                WIDE_JIS_SYMBOL_PROLONGED_SOUND_MARK,
                WIDE_KANA_HIRAGANA_DIGRAPH_YORI,
                WIDE_KANA_KATAKANA_DIGRAPH_YORI,
            ])
    }

    /// ひらがなに変換可能な全角カタカナかどうかを判定します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::utils::KanaUtils;
    /// use kanaria::UCSChar;
    ///
    /// let true_case = KanaUtils::is_can_convert_hiragana('ア');
    /// assert_eq!(true_case, true);
    /// ```
    #[inline]
    pub fn is_can_convert_hiragana<T>(target: T) -> bool where T: UCSChar {
        target.is_in_range(WIDE_KANA_KATAKANA_SMALL_A, WIDE_KANA_KATAKANA_SMALL_KE) ||
            target.is_in_range(WIDE_KANA_KATAKANA_ITERATION_MARK, WIDE_KANA_KATAKANA_DIGRAPH_YORI)
    }

    /// 全角カタカナをひらがなに変換します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::utils::KanaUtils;
    /// use kanaria::UCSChar;
    ///
    /// let ret = KanaUtils::convert_to_hiragana('ア');
    /// assert_eq!(ret, 'あ');
    /// ```
    #[inline]
    pub fn convert_to_hiragana<T>(target_char: T) -> T where T: UCSChar {
        if KanaUtils::is_can_convert_hiragana(target_char) {
            target_char.subtraction(UNICODE_OFFSET_HIRAGANA_ZENKAKU_KATAKANA)
        } else {
            target_char
        }
    }

    /// ひらがなを全角カタカナに変換します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::utils::KanaUtils;
    /// use kanaria::UCSChar;
    ///
    /// let ret = KanaUtils::convert_to_katakana('あ');
    /// assert_eq!(ret, 'ア');
    /// ```
    #[inline]
    pub fn convert_to_katakana<T>(target_char: T) -> T where T: UCSChar {
        if KanaUtils::is_hiragana(target_char) {
            target_char.addition(UNICODE_OFFSET_HIRAGANA_ZENKAKU_KATAKANA)
        } else {
            target_char
        }
    }
}
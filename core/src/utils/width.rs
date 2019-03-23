use crate::constants::*;
use crate::UCSChar;

pub struct WidthUtils;

impl WidthUtils {
    /// 全角文字を半角に変換します。
    /// 全角カタカナの濁音文字など、戻り値が2文字分になる場合もあります。
    /// 戻り値が1文字のときは2文字目にはヌル文字相当の値が入っています。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::utils::WidthUtils;
    /// use kanaria::UCSChar;
    ///
    /// let (ret1_a, ret1_b) = WidthUtils::convert_to_narrow('ア');
    /// assert_eq!(ret1_a, 'ｱ');
    /// assert_eq!(ret1_b, char::NULL);
    ///
    /// let (ret2_a, ret2_b) = WidthUtils::convert_to_narrow('ガ');
    /// assert_eq!(ret2_a, 'ｶ');
    /// assert_eq!(ret2_b, 'ﾞ');
    /// ```
    #[inline]
    pub fn convert_to_narrow<T>(target: T) -> (T, T) where T: UCSChar {
        let target_scalar = target.as_scalar();
        let first;
        let second;

        match target_scalar {
            WIDE_ASCII_SYMBOL_SPACE => {
                first = NARROW_ASCII_SYMBOL_SPACE;
                second = u32::NULL;
            }
            WIDE_ASCII_SYMBOL_EXCLAMATION_MARK => {
                first = NARROW_ASCII_SYMBOL_EXCLAMATION_MARK;
                second = u32::NULL;
            }
            WIDE_ASCII_SYMBOL_QUOTATION_MARK => {
                first = NARROW_ASCII_SYMBOL_QUOTATION_MARK;
                second = u32::NULL;
            }
            WIDE_ASCII_SYMBOL_NUMBER_SIGN => {
                first = NARROW_ASCII_SYMBOL_NUMBER_SIGN;
                second = u32::NULL;
            }
            WIDE_ASCII_SYMBOL_DOLLAR_SIGN => {
                first = NARROW_ASCII_SYMBOL_DOLLAR_SIGN;
                second = u32::NULL;
            }
            WIDE_ASCII_SYMBOL_PERCENT_SIGN => {
                first = NARROW_ASCII_SYMBOL_PERCENT_SIGN;
                second = u32::NULL;
            }
            WIDE_ASCII_SYMBOL_AMPERSAND => {
                first = NARROW_ASCII_SYMBOL_AMPERSAND;
                second = u32::NULL;
            }
            WIDE_ASCII_SYMBOL_APOSTROPHE => {
                first = NARROW_ASCII_SYMBOL_APOSTROPHE;
                second = u32::NULL;
            }
            WIDE_ASCII_SYMBOL_LEFT_PARENTHESIS => {
                first = NARROW_ASCII_SYMBOL_LEFT_PARENTHESIS;
                second = u32::NULL;
            }
            WIDE_ASCII_SYMBOL_RIGHT_PARENTHESIS => {
                first = NARROW_ASCII_SYMBOL_RIGHT_PARENTHESIS;
                second = u32::NULL;
            }
            WIDE_ASCII_SYMBOL_ASTERISK => {
                first = NARROW_ASCII_SYMBOL_ASTERISK;
                second = u32::NULL;
            }
            WIDE_ASCII_SYMBOL_PLUS_SIGN => {
                first = NARROW_ASCII_SYMBOL_PLUS_SIGN;
                second = u32::NULL;
            }
            WIDE_ASCII_SYMBOL_COMMA => {
                first = NARROW_ASCII_SYMBOL_COMMA;
                second = u32::NULL;
            }
            WIDE_ASCII_SYMBOL_HYPHEN_MINUS => {
                first = NARROW_ASCII_SYMBOL_HYPHEN_MINUS;
                second = u32::NULL;
            }
            WIDE_ASCII_SYMBOL_FULL_STOP => {
                first = NARROW_ASCII_SYMBOL_FULL_STOP;
                second = u32::NULL;
            }
            WIDE_ASCII_SYMBOL_SOLIDUS => {
                first = NARROW_ASCII_SYMBOL_SOLIDUS;
                second = u32::NULL;
            }
            WIDE_ASCII_NUMBER_0 => {
                first = NARROW_ASCII_NUMBER_0;
                second = u32::NULL;
            }
            WIDE_ASCII_NUMBER_1 => {
                first = NARROW_ASCII_NUMBER_1;
                second = u32::NULL;
            }
            WIDE_ASCII_NUMBER_2 => {
                first = NARROW_ASCII_NUMBER_2;
                second = u32::NULL;
            }
            WIDE_ASCII_NUMBER_3 => {
                first = NARROW_ASCII_NUMBER_3;
                second = u32::NULL;
            }
            WIDE_ASCII_NUMBER_4 => {
                first = NARROW_ASCII_NUMBER_4;
                second = u32::NULL;
            }
            WIDE_ASCII_NUMBER_5 => {
                first = NARROW_ASCII_NUMBER_5;
                second = u32::NULL;
            }
            WIDE_ASCII_NUMBER_6 => {
                first = NARROW_ASCII_NUMBER_6;
                second = u32::NULL;
            }
            WIDE_ASCII_NUMBER_7 => {
                first = NARROW_ASCII_NUMBER_7;
                second = u32::NULL;
            }
            WIDE_ASCII_NUMBER_8 => {
                first = NARROW_ASCII_NUMBER_8;
                second = u32::NULL;
            }
            WIDE_ASCII_NUMBER_9 => {
                first = NARROW_ASCII_NUMBER_9;
                second = u32::NULL;
            }
            WIDE_ASCII_SYMBOL_COLON => {
                first = NARROW_ASCII_SYMBOL_COLON;
                second = u32::NULL;
            }
            WIDE_ASCII_SYMBOL_SEMICOLON => {
                first = NARROW_ASCII_SYMBOL_SEMICOLON;
                second = u32::NULL;
            }
            WIDE_ASCII_SYMBOL_LESS_THAN_SIGN => {
                first = NARROW_ASCII_SYMBOL_LESS_THAN_SIGN;
                second = u32::NULL;
            }
            WIDE_ASCII_SYMBOL_EQUALS_SIGN => {
                first = NARROW_ASCII_SYMBOL_EQUALS_SIGN;
                second = u32::NULL;
            }
            WIDE_ASCII_SYMBOL_GREATER_THAN_SIGN => {
                first = NARROW_ASCII_SYMBOL_GREATER_THAN_SIGN;
                second = u32::NULL;
            }
            WIDE_ASCII_SYMBOL_QUESTION_MARK => {
                first = NARROW_ASCII_SYMBOL_QUESTION_MARK;
                second = u32::NULL;
            }
            WIDE_ASCII_SYMBOL_COMMERCIAL_AT => {
                first = NARROW_ASCII_SYMBOL_COMMERCIAL_AT;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_UPPER_CASE_A => {
                first = NARROW_ASCII_LATIN_UPPER_CASE_A;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_UPPER_CASE_B => {
                first = NARROW_ASCII_LATIN_UPPER_CASE_B;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_UPPER_CASE_C => {
                first = NARROW_ASCII_LATIN_UPPER_CASE_C;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_UPPER_CASE_D => {
                first = NARROW_ASCII_LATIN_UPPER_CASE_D;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_UPPER_CASE_E => {
                first = NARROW_ASCII_LATIN_UPPER_CASE_E;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_UPPER_CASE_F => {
                first = NARROW_ASCII_LATIN_UPPER_CASE_F;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_UPPER_CASE_G => {
                first = NARROW_ASCII_LATIN_UPPER_CASE_G;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_UPPER_CASE_H => {
                first = NARROW_ASCII_LATIN_UPPER_CASE_H;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_UPPER_CASE_I => {
                first = NARROW_ASCII_LATIN_UPPER_CASE_I;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_UPPER_CASE_J => {
                first = NARROW_ASCII_LATIN_UPPER_CASE_J;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_UPPER_CASE_K => {
                first = NARROW_ASCII_LATIN_UPPER_CASE_K;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_UPPER_CASE_L => {
                first = NARROW_ASCII_LATIN_UPPER_CASE_L;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_UPPER_CASE_M => {
                first = NARROW_ASCII_LATIN_UPPER_CASE_M;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_UPPER_CASE_N => {
                first = NARROW_ASCII_LATIN_UPPER_CASE_N;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_UPPER_CASE_O => {
                first = NARROW_ASCII_LATIN_UPPER_CASE_O;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_UPPER_CASE_P => {
                first = NARROW_ASCII_LATIN_UPPER_CASE_P;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_UPPER_CASE_Q => {
                first = NARROW_ASCII_LATIN_UPPER_CASE_Q;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_UPPER_CASE_R => {
                first = NARROW_ASCII_LATIN_UPPER_CASE_R;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_UPPER_CASE_S => {
                first = NARROW_ASCII_LATIN_UPPER_CASE_S;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_UPPER_CASE_T => {
                first = NARROW_ASCII_LATIN_UPPER_CASE_T;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_UPPER_CASE_U => {
                first = NARROW_ASCII_LATIN_UPPER_CASE_U;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_UPPER_CASE_V => {
                first = NARROW_ASCII_LATIN_UPPER_CASE_V;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_UPPER_CASE_W => {
                first = NARROW_ASCII_LATIN_UPPER_CASE_W;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_UPPER_CASE_X => {
                first = NARROW_ASCII_LATIN_UPPER_CASE_X;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_UPPER_CASE_Y => {
                first = NARROW_ASCII_LATIN_UPPER_CASE_Y;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_UPPER_CASE_Z => {
                first = NARROW_ASCII_LATIN_UPPER_CASE_Z;
                second = u32::NULL;
            }
            WIDE_ASCII_SYMBOL_LEFT_SQUARE_BRACKET => {
                first = NARROW_ASCII_SYMBOL_LEFT_SQUARE_BRACKET;
                second = u32::NULL;
            }
            WIDE_ASCII_SYMBOL_REVERSE_SOLIDUS => {
                first = NARROW_ASCII_SYMBOL_REVERSE_SOLIDUS;
                second = u32::NULL;
            }
            WIDE_ASCII_SYMBOL_RIGHT_SQUARE_BRACKET => {
                first = NARROW_ASCII_SYMBOL_RIGHT_SQUARE_BRACKET;
                second = u32::NULL;
            }
            WIDE_ASCII_SYMBOL_CIRCUMFLEX_ACCENT => {
                first = NARROW_ASCII_SYMBOL_CIRCUMFLEX_ACCENT;
                second = u32::NULL;
            }
            WIDE_ASCII_SYMBOL_LOW_LINE => {
                first = NARROW_ASCII_SYMBOL_LOW_LINE;
                second = u32::NULL;
            }
            WIDE_ASCII_SYMBOL_GRAVE_ACCENT => {
                first = NARROW_ASCII_SYMBOL_GRAVE_ACCENT;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_LOWER_CASE_A => {
                first = NARROW_ASCII_LATIN_LOWER_CASE_A;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_LOWER_CASE_B => {
                first = NARROW_ASCII_LATIN_LOWER_CASE_B;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_LOWER_CASE_C => {
                first = NARROW_ASCII_LATIN_LOWER_CASE_C;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_LOWER_CASE_D => {
                first = NARROW_ASCII_LATIN_LOWER_CASE_D;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_LOWER_CASE_E => {
                first = NARROW_ASCII_LATIN_LOWER_CASE_E;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_LOWER_CASE_F => {
                first = NARROW_ASCII_LATIN_LOWER_CASE_F;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_LOWER_CASE_G => {
                first = NARROW_ASCII_LATIN_LOWER_CASE_G;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_LOWER_CASE_H => {
                first = NARROW_ASCII_LATIN_LOWER_CASE_H;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_LOWER_CASE_I => {
                first = NARROW_ASCII_LATIN_LOWER_CASE_I;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_LOWER_CASE_J => {
                first = NARROW_ASCII_LATIN_LOWER_CASE_J;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_LOWER_CASE_K => {
                first = NARROW_ASCII_LATIN_LOWER_CASE_K;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_LOWER_CASE_L => {
                first = NARROW_ASCII_LATIN_LOWER_CASE_L;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_LOWER_CASE_M => {
                first = NARROW_ASCII_LATIN_LOWER_CASE_M;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_LOWER_CASE_N => {
                first = NARROW_ASCII_LATIN_LOWER_CASE_N;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_LOWER_CASE_O => {
                first = NARROW_ASCII_LATIN_LOWER_CASE_O;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_LOWER_CASE_P => {
                first = NARROW_ASCII_LATIN_LOWER_CASE_P;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_LOWER_CASE_Q => {
                first = NARROW_ASCII_LATIN_LOWER_CASE_Q;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_LOWER_CASE_R => {
                first = NARROW_ASCII_LATIN_LOWER_CASE_R;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_LOWER_CASE_S => {
                first = NARROW_ASCII_LATIN_LOWER_CASE_S;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_LOWER_CASE_T => {
                first = NARROW_ASCII_LATIN_LOWER_CASE_T;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_LOWER_CASE_U => {
                first = NARROW_ASCII_LATIN_LOWER_CASE_U;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_LOWER_CASE_V => {
                first = NARROW_ASCII_LATIN_LOWER_CASE_V;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_LOWER_CASE_W => {
                first = NARROW_ASCII_LATIN_LOWER_CASE_W;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_LOWER_CASE_X => {
                first = NARROW_ASCII_LATIN_LOWER_CASE_X;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_LOWER_CASE_Y => {
                first = NARROW_ASCII_LATIN_LOWER_CASE_Y;
                second = u32::NULL;
            }
            WIDE_ASCII_LATIN_LOWER_CASE_Z => {
                first = NARROW_ASCII_LATIN_LOWER_CASE_Z;
                second = u32::NULL;
            }
            WIDE_ASCII_SYMBOL_LEFT_CURLY_BRACKET => {
                first = NARROW_ASCII_SYMBOL_LEFT_CURLY_BRACKET;
                second = u32::NULL;
            }
            WIDE_ASCII_SYMBOL_VERTICAL_LINE => {
                first = NARROW_ASCII_SYMBOL_VERTICAL_LINE;
                second = u32::NULL;
            }
            WIDE_ASCII_SYMBOL_RIGHT_CURLY_BRACKET => {
                first = NARROW_ASCII_SYMBOL_RIGHT_CURLY_BRACKET;
                second = u32::NULL;
            }
            WIDE_ASCII_SYMBOL_TILDE => {
                first = NARROW_ASCII_SYMBOL_TILDE;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_SMALL_A => {
                first = NARROW_KANA_KATAKANA_SMALL_A;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_A => {
                first = NARROW_KANA_KATAKANA_A;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_SMALL_I => {
                first = NARROW_KANA_KATAKANA_SMALL_I;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_I => {
                first = NARROW_KANA_KATAKANA_I;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_SMALL_U => {
                first = NARROW_KANA_KATAKANA_SMALL_U;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_U => {
                first = NARROW_KANA_KATAKANA_U;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_SMALL_E => {
                first = NARROW_KANA_KATAKANA_SMALL_E;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_E => {
                first = NARROW_KANA_KATAKANA_E;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_SMALL_O => {
                first = NARROW_KANA_KATAKANA_SMALL_O;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_O => {
                first = NARROW_KANA_KATAKANA_O;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_KA => {
                first = NARROW_KANA_KATAKANA_KA;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_KI => {
                first = NARROW_KANA_KATAKANA_KI;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_KU => {
                first = NARROW_KANA_KATAKANA_KU;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_KE => {
                first = NARROW_KANA_KATAKANA_KE;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_KO => {
                first = NARROW_KANA_KATAKANA_KO;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_SA => {
                first = NARROW_KANA_KATAKANA_SA;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_SI => {
                first = NARROW_KANA_KATAKANA_SI;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_SU => {
                first = NARROW_KANA_KATAKANA_SU;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_SE => {
                first = NARROW_KANA_KATAKANA_SE;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_SO => {
                first = NARROW_KANA_KATAKANA_SO;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_TA => {
                first = NARROW_KANA_KATAKANA_TA;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_TI => {
                first = NARROW_KANA_KATAKANA_TI;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_SMALL_TU => {
                first = NARROW_KANA_KATAKANA_SMALL_TU;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_TU => {
                first = NARROW_KANA_KATAKANA_TU;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_TE => {
                first = NARROW_KANA_KATAKANA_TE;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_TO => {
                first = NARROW_KANA_KATAKANA_TO;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_NA => {
                first = NARROW_KANA_KATAKANA_NA;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_NI => {
                first = NARROW_KANA_KATAKANA_NI;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_NU => {
                first = NARROW_KANA_KATAKANA_NU;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_NE => {
                first = NARROW_KANA_KATAKANA_NE;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_NO => {
                first = NARROW_KANA_KATAKANA_NO;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_HA => {
                first = NARROW_KANA_KATAKANA_HA;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_HI => {
                first = NARROW_KANA_KATAKANA_HI;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_HU => {
                first = NARROW_KANA_KATAKANA_HU;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_HE => {
                first = NARROW_KANA_KATAKANA_HE;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_HO => {
                first = NARROW_KANA_KATAKANA_HO;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_MA => {
                first = NARROW_KANA_KATAKANA_MA;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_MI => {
                first = NARROW_KANA_KATAKANA_MI;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_MU => {
                first = NARROW_KANA_KATAKANA_MU;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_ME => {
                first = NARROW_KANA_KATAKANA_ME;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_MO => {
                first = NARROW_KANA_KATAKANA_MO;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_SMALL_YA => {
                first = NARROW_KANA_KATAKANA_SMALL_YA;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_YA => {
                first = NARROW_KANA_KATAKANA_YA;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_SMALL_YU => {
                first = NARROW_KANA_KATAKANA_SMALL_YU;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_YU => {
                first = NARROW_KANA_KATAKANA_YU;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_SMALL_YO => {
                first = NARROW_KANA_KATAKANA_SMALL_YO;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_YO => {
                first = NARROW_KANA_KATAKANA_YO;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_RA => {
                first = NARROW_KANA_KATAKANA_RA;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_RI => {
                first = NARROW_KANA_KATAKANA_RI;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_RU => {
                first = NARROW_KANA_KATAKANA_RU;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_RE => {
                first = NARROW_KANA_KATAKANA_RE;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_RO => {
                first = NARROW_KANA_KATAKANA_RO;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_WA => {
                first = NARROW_KANA_KATAKANA_WA;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_WO => {
                first = NARROW_KANA_KATAKANA_WO;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_N => {
                first = NARROW_KANA_KATAKANA_N;
                second = u32::NULL;
            }
            WIDE_JIS_SYMBOL_LEFTWARDS_ARROW => {
                first = NARROW_JIS_SYMBOL_LEFTWARDS_ARROW;
                second = u32::NULL;
            }
            WIDE_JIS_SYMBOL_UPWARDS_ARROW => {
                first = NARROW_JIS_SYMBOL_UPWARDS_ARROW;
                second = u32::NULL;
            }
            WIDE_JIS_SYMBOL_RIGHTWARDS_ARROW => {
                first = NARROW_JIS_SYMBOL_RIGHTWARDS_ARROW;
                second = u32::NULL;
            }
            WIDE_JIS_SYMBOL_DOWNWARDS_ARROW => {
                first = NARROW_JIS_SYMBOL_DOWNWARDS_ARROW;
                second = u32::NULL;
            }
            WIDE_JIS_SYMBOL_FORMS_LIGHT_VERTICAL => {
                first = NARROW_JIS_SYMBOL_FORMS_LIGHT_VERTICAL;
                second = u32::NULL;
            }
            WIDE_JIS_SYMBOL_BLACK_SQUARE => {
                first = NARROW_JIS_SYMBOL_BLACK_SQUARE;
                second = u32::NULL;
            }
            WIDE_JIS_SYMBOL_WHITE_CIRCLE => {
                first = NARROW_JIS_SYMBOL_WHITE_CIRCLE;
                second = u32::NULL;
            }
            WIDE_JIS_SYMBOL_LEFT_WHITE_PARENTHESIS => {
                first = NARROW_JIS_SYMBOL_LEFT_WHITE_PARENTHESIS;
                second = u32::NULL;
            }
            WIDE_JIS_SYMBOL_RIGHT_WHITE_PARENTHESIS => {
                first = NARROW_JIS_SYMBOL_RIGHT_WHITE_PARENTHESIS;
                second = u32::NULL;
            }
            WIDE_JIS_SYMBOL_IDEOGRAPHIC_COMMA => {
                first = NARROW_JIS_SYMBOL_IDEOGRAPHIC_COMMA;
                second = u32::NULL;
            }
            WIDE_JIS_SYMBOL_IDEOGRAPHIC_FULL_STOP => {
                first = NARROW_JIS_SYMBOL_IDEOGRAPHIC_FULL_STOP;
                second = u32::NULL;
            }
            WIDE_JIS_SYMBOL_LEFT_CORNER_BRACKET => {
                first = NARROW_JIS_SYMBOL_LEFT_CORNER_BRACKET;
                second = u32::NULL;
            }
            WIDE_JIS_SYMBOL_RIGHT_CORNER_BRACKET => {
                first = NARROW_JIS_SYMBOL_RIGHT_CORNER_BRACKET;
                second = u32::NULL;
            }
            WIDE_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK => {
                first = NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK;
                second = u32::NULL;
            }
            WIDE_JIS_SYMBOL_KATAKANA_SEMI_VOICED_SOUND_MARK => {
                first = NARROW_JIS_SYMBOL_KATAKANA_SEMI_VOICED_SOUND_MARK;
                second = u32::NULL;
            }
            WIDE_JIS_SYMBOL_KATAKANA_MIDDLE_DOT => {
                first = NARROW_JIS_SYMBOL_KATAKANA_MIDDLE_DOT;
                second = u32::NULL;
            }
            WIDE_JIS_SYMBOL_PROLONGED_SOUND_MARK => {
                first = NARROW_JIS_SYMBOL_PROLONGED_SOUND_MARK;
                second = u32::NULL;
            }
            WIDE_JIS_SYMBOL_CENT_SIGN => {
                first = NARROW_JIS_SYMBOL_CENT_SIGN;
                second = u32::NULL;
            }
            WIDE_JIS_SYMBOL_POUND_SIGN => {
                first = NARROW_JIS_SYMBOL_POUND_SIGN;
                second = u32::NULL;
            }
            WIDE_JIS_SYMBOL_NOT_SIGN => {
                first = NARROW_JIS_SYMBOL_NOT_SIGN;
                second = u32::NULL;
            }
            WIDE_JIS_SYMBOL_MACRON => {
                first = NARROW_JIS_SYMBOL_MACRON;
                second = u32::NULL;
            }
            WIDE_JIS_SYMBOL_BROKEN_BAR => {
                first = NARROW_JIS_SYMBOL_BROKEN_BAR;
                second = u32::NULL;
            }
            WIDE_JIS_SYMBOL_YEN_SIGN => {
                first = NARROW_JIS_SYMBOL_YEN_SIGN;
                second = u32::NULL;
            }
            WIDE_JIS_SYMBOL_WON_SIGN => {
                first = NARROW_JIS_SYMBOL_WON_SIGN;
                second = u32::NULL;
            }
            WIDE_KANA_KATAKANA_GA => {
                first = NARROW_KANA_KATAKANA_KA;
                second = NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK;
            }
            WIDE_KANA_KATAKANA_GI => {
                first = NARROW_KANA_KATAKANA_KI;
                second = NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK;
            }
            WIDE_KANA_KATAKANA_GU => {
                first = NARROW_KANA_KATAKANA_KU;
                second = NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK;
            }
            WIDE_KANA_KATAKANA_GE => {
                first = NARROW_KANA_KATAKANA_KE;
                second = NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK;
            }
            WIDE_KANA_KATAKANA_GO => {
                first = NARROW_KANA_KATAKANA_KO;
                second = NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK;
            }
            WIDE_KANA_KATAKANA_ZA => {
                first = NARROW_KANA_KATAKANA_SA;
                second = NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK;
            }
            WIDE_KANA_KATAKANA_ZI => {
                first = NARROW_KANA_KATAKANA_SI;
                second = NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK;
            }
            WIDE_KANA_KATAKANA_ZU => {
                first = NARROW_KANA_KATAKANA_SU;
                second = NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK;
            }
            WIDE_KANA_KATAKANA_ZE => {
                first = NARROW_KANA_KATAKANA_SE;
                second = NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK;
            }
            WIDE_KANA_KATAKANA_ZO => {
                first = NARROW_KANA_KATAKANA_SO;
                second = NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK;
            }
            WIDE_KANA_KATAKANA_DA => {
                first = NARROW_KANA_KATAKANA_TA;
                second = NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK;
            }
            WIDE_KANA_KATAKANA_DI => {
                first = NARROW_KANA_KATAKANA_TI;
                second = NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK;
            }
            WIDE_KANA_KATAKANA_DU => {
                first = NARROW_KANA_KATAKANA_TU;
                second = NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK;
            }
            WIDE_KANA_KATAKANA_DE => {
                first = NARROW_KANA_KATAKANA_TE;
                second = NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK;
            }
            WIDE_KANA_KATAKANA_DO => {
                first = NARROW_KANA_KATAKANA_TO;
                second = NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK;
            }
            WIDE_KANA_KATAKANA_BA => {
                first = NARROW_KANA_KATAKANA_HA;
                second = NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK;
            }
            WIDE_KANA_KATAKANA_PA => {
                first = NARROW_KANA_KATAKANA_HA;
                second = NARROW_JIS_SYMBOL_KATAKANA_SEMI_VOICED_SOUND_MARK;
            }
            WIDE_KANA_KATAKANA_BI => {
                first = NARROW_KANA_KATAKANA_HI;
                second = NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK;
            }
            WIDE_KANA_KATAKANA_PI => {
                first = NARROW_KANA_KATAKANA_HI;
                second = NARROW_JIS_SYMBOL_KATAKANA_SEMI_VOICED_SOUND_MARK;
            }
            WIDE_KANA_KATAKANA_BU => {
                first = NARROW_KANA_KATAKANA_HU;
                second = NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK;
            }
            WIDE_KANA_KATAKANA_PU => {
                first = NARROW_KANA_KATAKANA_HU;
                second = NARROW_JIS_SYMBOL_KATAKANA_SEMI_VOICED_SOUND_MARK;
            }
            WIDE_KANA_KATAKANA_BE => {
                first = NARROW_KANA_KATAKANA_HE;
                second = NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK;
            }
            WIDE_KANA_KATAKANA_PE => {
                first = NARROW_KANA_KATAKANA_HE;
                second = NARROW_JIS_SYMBOL_KATAKANA_SEMI_VOICED_SOUND_MARK;
            }
            WIDE_KANA_KATAKANA_BO => {
                first = NARROW_KANA_KATAKANA_HO;
                second = NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK;
            }
            WIDE_KANA_KATAKANA_PO => {
                first = NARROW_KANA_KATAKANA_HO;
                second = NARROW_JIS_SYMBOL_KATAKANA_SEMI_VOICED_SOUND_MARK;
            }
            WIDE_KANA_KATAKANA_VU => {
                first = NARROW_KANA_KATAKANA_U;
                second = NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK;
            }
            WIDE_KANA_KATAKANA_VA => {
                first = NARROW_KANA_KATAKANA_WA;
                second = NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK;
            }
            WIDE_KANA_KATAKANA_VO => {
                first = NARROW_KANA_KATAKANA_WO;
                second = NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK;
            }
            _ => {
                first = target_scalar;
                second = u32::NULL;
            }
        }

        return (T::from_scalar(first), T::from_scalar(second));
    }

    /// 全角文字を半角に変換します。
    /// 一文字目と二文字目が結合可能だった場合、結合済みの文字列を返却します。
    /// 文字列が結合された場合、2つ目の戻り値にtrueが入っています。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::utils::WidthUtils;
    /// use kanaria::UCSChar;
    ///
    /// // アの濁音付きは存在しないため、2文字目の濁音は無視される
    /// assert_eq!(WidthUtils::convert_to_wide('ｱ', 'ﾞ'), ('ア', false));
    ///
    /// // アとイは別の文字なので結合できない
    /// assert_eq!(WidthUtils::convert_to_wide('ｱ', 'ｲ'), ('ア', false));
    ///
    /// // カと濁音記号はガに変換可能
    /// assert_eq!(WidthUtils::convert_to_wide('ｶ', 'ﾞ'), ('ガ', true));
    ///
    /// // カと半濁音記号は不可能
    /// assert_eq!(WidthUtils::convert_to_wide('ｶ', 'ﾟ'), ('カ', false));
    /// ```
    #[inline]
    pub fn convert_to_wide<T>(target: T, next: T) -> (T, bool) where T: UCSChar {
        let target_scalar = target.as_scalar();
        let result;
        let is_pad;

        match target_scalar {
            NARROW_ASCII_SYMBOL_SPACE => {
                result = WIDE_ASCII_SYMBOL_SPACE;
                is_pad = false;
            }
            NARROW_ASCII_SYMBOL_EXCLAMATION_MARK => {
                result = WIDE_ASCII_SYMBOL_EXCLAMATION_MARK;
                is_pad = false;
            }
            NARROW_ASCII_SYMBOL_QUOTATION_MARK => {
                result = WIDE_ASCII_SYMBOL_QUOTATION_MARK;
                is_pad = false;
            }
            NARROW_ASCII_SYMBOL_NUMBER_SIGN => {
                result = WIDE_ASCII_SYMBOL_NUMBER_SIGN;
                is_pad = false;
            }
            NARROW_ASCII_SYMBOL_DOLLAR_SIGN => {
                result = WIDE_ASCII_SYMBOL_DOLLAR_SIGN;
                is_pad = false;
            }
            NARROW_ASCII_SYMBOL_PERCENT_SIGN => {
                result = WIDE_ASCII_SYMBOL_PERCENT_SIGN;
                is_pad = false;
            }
            NARROW_ASCII_SYMBOL_AMPERSAND => {
                result = WIDE_ASCII_SYMBOL_AMPERSAND;
                is_pad = false;
            }
            NARROW_ASCII_SYMBOL_APOSTROPHE => {
                result = WIDE_ASCII_SYMBOL_APOSTROPHE;
                is_pad = false;
            }
            NARROW_ASCII_SYMBOL_LEFT_PARENTHESIS => {
                result = WIDE_ASCII_SYMBOL_LEFT_PARENTHESIS;
                is_pad = false;
            }
            NARROW_ASCII_SYMBOL_RIGHT_PARENTHESIS => {
                result = WIDE_ASCII_SYMBOL_RIGHT_PARENTHESIS;
                is_pad = false;
            }
            NARROW_ASCII_SYMBOL_ASTERISK => {
                result = WIDE_ASCII_SYMBOL_ASTERISK;
                is_pad = false;
            }
            NARROW_ASCII_SYMBOL_PLUS_SIGN => {
                result = WIDE_ASCII_SYMBOL_PLUS_SIGN;
                is_pad = false;
            }
            NARROW_ASCII_SYMBOL_COMMA => {
                result = WIDE_ASCII_SYMBOL_COMMA;
                is_pad = false;
            }
            NARROW_ASCII_SYMBOL_HYPHEN_MINUS => {
                result = WIDE_ASCII_SYMBOL_HYPHEN_MINUS;
                is_pad = false;
            }
            NARROW_ASCII_SYMBOL_FULL_STOP => {
                result = WIDE_ASCII_SYMBOL_FULL_STOP;
                is_pad = false;
            }
            NARROW_ASCII_SYMBOL_SOLIDUS => {
                result = WIDE_ASCII_SYMBOL_SOLIDUS;
                is_pad = false;
            }
            NARROW_ASCII_NUMBER_0 => {
                result = WIDE_ASCII_NUMBER_0;
                is_pad = false;
            }
            NARROW_ASCII_NUMBER_1 => {
                result = WIDE_ASCII_NUMBER_1;
                is_pad = false;
            }
            NARROW_ASCII_NUMBER_2 => {
                result = WIDE_ASCII_NUMBER_2;
                is_pad = false;
            }
            NARROW_ASCII_NUMBER_3 => {
                result = WIDE_ASCII_NUMBER_3;
                is_pad = false;
            }
            NARROW_ASCII_NUMBER_4 => {
                result = WIDE_ASCII_NUMBER_4;
                is_pad = false;
            }
            NARROW_ASCII_NUMBER_5 => {
                result = WIDE_ASCII_NUMBER_5;
                is_pad = false;
            }
            NARROW_ASCII_NUMBER_6 => {
                result = WIDE_ASCII_NUMBER_6;
                is_pad = false;
            }
            NARROW_ASCII_NUMBER_7 => {
                result = WIDE_ASCII_NUMBER_7;
                is_pad = false;
            }
            NARROW_ASCII_NUMBER_8 => {
                result = WIDE_ASCII_NUMBER_8;
                is_pad = false;
            }
            NARROW_ASCII_NUMBER_9 => {
                result = WIDE_ASCII_NUMBER_9;
                is_pad = false;
            }
            NARROW_ASCII_SYMBOL_COLON => {
                result = WIDE_ASCII_SYMBOL_COLON;
                is_pad = false;
            }
            NARROW_ASCII_SYMBOL_SEMICOLON => {
                result = WIDE_ASCII_SYMBOL_SEMICOLON;
                is_pad = false;
            }
            NARROW_ASCII_SYMBOL_LESS_THAN_SIGN => {
                result = WIDE_ASCII_SYMBOL_LESS_THAN_SIGN;
                is_pad = false;
            }
            NARROW_ASCII_SYMBOL_EQUALS_SIGN => {
                result = WIDE_ASCII_SYMBOL_EQUALS_SIGN;
                is_pad = false;
            }
            NARROW_ASCII_SYMBOL_GREATER_THAN_SIGN => {
                result = WIDE_ASCII_SYMBOL_GREATER_THAN_SIGN;
                is_pad = false;
            }
            NARROW_ASCII_SYMBOL_QUESTION_MARK => {
                result = WIDE_ASCII_SYMBOL_QUESTION_MARK;
                is_pad = false;
            }
            NARROW_ASCII_SYMBOL_COMMERCIAL_AT => {
                result = WIDE_ASCII_SYMBOL_COMMERCIAL_AT;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_UPPER_CASE_A => {
                result = WIDE_ASCII_LATIN_UPPER_CASE_A;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_UPPER_CASE_B => {
                result = WIDE_ASCII_LATIN_UPPER_CASE_B;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_UPPER_CASE_C => {
                result = WIDE_ASCII_LATIN_UPPER_CASE_C;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_UPPER_CASE_D => {
                result = WIDE_ASCII_LATIN_UPPER_CASE_D;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_UPPER_CASE_E => {
                result = WIDE_ASCII_LATIN_UPPER_CASE_E;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_UPPER_CASE_F => {
                result = WIDE_ASCII_LATIN_UPPER_CASE_F;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_UPPER_CASE_G => {
                result = WIDE_ASCII_LATIN_UPPER_CASE_G;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_UPPER_CASE_H => {
                result = WIDE_ASCII_LATIN_UPPER_CASE_H;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_UPPER_CASE_I => {
                result = WIDE_ASCII_LATIN_UPPER_CASE_I;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_UPPER_CASE_J => {
                result = WIDE_ASCII_LATIN_UPPER_CASE_J;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_UPPER_CASE_K => {
                result = WIDE_ASCII_LATIN_UPPER_CASE_K;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_UPPER_CASE_L => {
                result = WIDE_ASCII_LATIN_UPPER_CASE_L;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_UPPER_CASE_M => {
                result = WIDE_ASCII_LATIN_UPPER_CASE_M;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_UPPER_CASE_N => {
                result = WIDE_ASCII_LATIN_UPPER_CASE_N;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_UPPER_CASE_O => {
                result = WIDE_ASCII_LATIN_UPPER_CASE_O;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_UPPER_CASE_P => {
                result = WIDE_ASCII_LATIN_UPPER_CASE_P;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_UPPER_CASE_Q => {
                result = WIDE_ASCII_LATIN_UPPER_CASE_Q;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_UPPER_CASE_R => {
                result = WIDE_ASCII_LATIN_UPPER_CASE_R;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_UPPER_CASE_S => {
                result = WIDE_ASCII_LATIN_UPPER_CASE_S;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_UPPER_CASE_T => {
                result = WIDE_ASCII_LATIN_UPPER_CASE_T;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_UPPER_CASE_U => {
                result = WIDE_ASCII_LATIN_UPPER_CASE_U;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_UPPER_CASE_V => {
                result = WIDE_ASCII_LATIN_UPPER_CASE_V;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_UPPER_CASE_W => {
                result = WIDE_ASCII_LATIN_UPPER_CASE_W;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_UPPER_CASE_X => {
                result = WIDE_ASCII_LATIN_UPPER_CASE_X;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_UPPER_CASE_Y => {
                result = WIDE_ASCII_LATIN_UPPER_CASE_Y;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_UPPER_CASE_Z => {
                result = WIDE_ASCII_LATIN_UPPER_CASE_Z;
                is_pad = false;
            }
            NARROW_ASCII_SYMBOL_LEFT_SQUARE_BRACKET => {
                result = WIDE_ASCII_SYMBOL_LEFT_SQUARE_BRACKET;
                is_pad = false;
            }
            NARROW_ASCII_SYMBOL_REVERSE_SOLIDUS => {
                result = WIDE_ASCII_SYMBOL_REVERSE_SOLIDUS;
                is_pad = false;
            }
            NARROW_ASCII_SYMBOL_RIGHT_SQUARE_BRACKET => {
                result = WIDE_ASCII_SYMBOL_RIGHT_SQUARE_BRACKET;
                is_pad = false;
            }
            NARROW_ASCII_SYMBOL_CIRCUMFLEX_ACCENT => {
                result = WIDE_ASCII_SYMBOL_CIRCUMFLEX_ACCENT;
                is_pad = false;
            }
            NARROW_ASCII_SYMBOL_LOW_LINE => {
                result = WIDE_ASCII_SYMBOL_LOW_LINE;
                is_pad = false;
            }
            NARROW_ASCII_SYMBOL_GRAVE_ACCENT => {
                result = WIDE_ASCII_SYMBOL_GRAVE_ACCENT;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_LOWER_CASE_A => {
                result = WIDE_ASCII_LATIN_LOWER_CASE_A;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_LOWER_CASE_B => {
                result = WIDE_ASCII_LATIN_LOWER_CASE_B;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_LOWER_CASE_C => {
                result = WIDE_ASCII_LATIN_LOWER_CASE_C;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_LOWER_CASE_D => {
                result = WIDE_ASCII_LATIN_LOWER_CASE_D;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_LOWER_CASE_E => {
                result = WIDE_ASCII_LATIN_LOWER_CASE_E;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_LOWER_CASE_F => {
                result = WIDE_ASCII_LATIN_LOWER_CASE_F;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_LOWER_CASE_G => {
                result = WIDE_ASCII_LATIN_LOWER_CASE_G;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_LOWER_CASE_H => {
                result = WIDE_ASCII_LATIN_LOWER_CASE_H;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_LOWER_CASE_I => {
                result = WIDE_ASCII_LATIN_LOWER_CASE_I;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_LOWER_CASE_J => {
                result = WIDE_ASCII_LATIN_LOWER_CASE_J;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_LOWER_CASE_K => {
                result = WIDE_ASCII_LATIN_LOWER_CASE_K;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_LOWER_CASE_L => {
                result = WIDE_ASCII_LATIN_LOWER_CASE_L;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_LOWER_CASE_M => {
                result = WIDE_ASCII_LATIN_LOWER_CASE_M;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_LOWER_CASE_N => {
                result = WIDE_ASCII_LATIN_LOWER_CASE_N;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_LOWER_CASE_O => {
                result = WIDE_ASCII_LATIN_LOWER_CASE_O;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_LOWER_CASE_P => {
                result = WIDE_ASCII_LATIN_LOWER_CASE_P;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_LOWER_CASE_Q => {
                result = WIDE_ASCII_LATIN_LOWER_CASE_Q;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_LOWER_CASE_R => {
                result = WIDE_ASCII_LATIN_LOWER_CASE_R;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_LOWER_CASE_S => {
                result = WIDE_ASCII_LATIN_LOWER_CASE_S;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_LOWER_CASE_T => {
                result = WIDE_ASCII_LATIN_LOWER_CASE_T;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_LOWER_CASE_U => {
                result = WIDE_ASCII_LATIN_LOWER_CASE_U;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_LOWER_CASE_V => {
                result = WIDE_ASCII_LATIN_LOWER_CASE_V;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_LOWER_CASE_W => {
                result = WIDE_ASCII_LATIN_LOWER_CASE_W;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_LOWER_CASE_X => {
                result = WIDE_ASCII_LATIN_LOWER_CASE_X;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_LOWER_CASE_Y => {
                result = WIDE_ASCII_LATIN_LOWER_CASE_Y;
                is_pad = false;
            }
            NARROW_ASCII_LATIN_LOWER_CASE_Z => {
                result = WIDE_ASCII_LATIN_LOWER_CASE_Z;
                is_pad = false;
            }
            NARROW_ASCII_SYMBOL_LEFT_CURLY_BRACKET => {
                result = WIDE_ASCII_SYMBOL_LEFT_CURLY_BRACKET;
                is_pad = false;
            }
            NARROW_ASCII_SYMBOL_VERTICAL_LINE => {
                result = WIDE_ASCII_SYMBOL_VERTICAL_LINE;
                is_pad = false;
            }
            NARROW_ASCII_SYMBOL_RIGHT_CURLY_BRACKET => {
                result = WIDE_ASCII_SYMBOL_RIGHT_CURLY_BRACKET;
                is_pad = false;
            }
            NARROW_ASCII_SYMBOL_TILDE => {
                result = WIDE_ASCII_SYMBOL_TILDE;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_SMALL_A => {
                result = WIDE_KANA_KATAKANA_SMALL_A;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_A => {
                result = WIDE_KANA_KATAKANA_A;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_SMALL_I => {
                result = WIDE_KANA_KATAKANA_SMALL_I;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_I => {
                result = WIDE_KANA_KATAKANA_I;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_SMALL_U => {
                result = WIDE_KANA_KATAKANA_SMALL_U;
                is_pad = false;
            }

            NARROW_KANA_KATAKANA_SMALL_E => {
                result = WIDE_KANA_KATAKANA_SMALL_E;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_E => {
                result = WIDE_KANA_KATAKANA_E;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_SMALL_O => {
                result = WIDE_KANA_KATAKANA_SMALL_O;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_O => {
                result = WIDE_KANA_KATAKANA_O;
                is_pad = false;
            }

            NARROW_KANA_KATAKANA_SMALL_TU => {
                result = WIDE_KANA_KATAKANA_SMALL_TU;
                is_pad = false;
            }

            NARROW_KANA_KATAKANA_NA => {
                result = WIDE_KANA_KATAKANA_NA;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_NI => {
                result = WIDE_KANA_KATAKANA_NI;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_NU => {
                result = WIDE_KANA_KATAKANA_NU;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_NE => {
                result = WIDE_KANA_KATAKANA_NE;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_NO => {
                result = WIDE_KANA_KATAKANA_NO;
                is_pad = false;
            }

            NARROW_KANA_KATAKANA_MA => {
                result = WIDE_KANA_KATAKANA_MA;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_MI => {
                result = WIDE_KANA_KATAKANA_MI;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_MU => {
                result = WIDE_KANA_KATAKANA_MU;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_ME => {
                result = WIDE_KANA_KATAKANA_ME;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_MO => {
                result = WIDE_KANA_KATAKANA_MO;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_SMALL_YA => {
                result = WIDE_KANA_KATAKANA_SMALL_YA;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_YA => {
                result = WIDE_KANA_KATAKANA_YA;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_SMALL_YU => {
                result = WIDE_KANA_KATAKANA_SMALL_YU;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_YU => {
                result = WIDE_KANA_KATAKANA_YU;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_SMALL_YO => {
                result = WIDE_KANA_KATAKANA_SMALL_YO;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_YO => {
                result = WIDE_KANA_KATAKANA_YO;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_RA => {
                result = WIDE_KANA_KATAKANA_RA;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_RI => {
                result = WIDE_KANA_KATAKANA_RI;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_RU => {
                result = WIDE_KANA_KATAKANA_RU;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_RE => {
                result = WIDE_KANA_KATAKANA_RE;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_RO => {
                result = WIDE_KANA_KATAKANA_RO;
                is_pad = false;
            }

            NARROW_KANA_KATAKANA_N => {
                result = WIDE_KANA_KATAKANA_N;
                is_pad = false;
            }
            NARROW_JIS_SYMBOL_LEFTWARDS_ARROW => {
                result = WIDE_JIS_SYMBOL_LEFTWARDS_ARROW;
                is_pad = false;
            }
            NARROW_JIS_SYMBOL_UPWARDS_ARROW => {
                result = WIDE_JIS_SYMBOL_UPWARDS_ARROW;
                is_pad = false;
            }
            NARROW_JIS_SYMBOL_RIGHTWARDS_ARROW => {
                result = WIDE_JIS_SYMBOL_RIGHTWARDS_ARROW;
                is_pad = false;
            }
            NARROW_JIS_SYMBOL_DOWNWARDS_ARROW => {
                result = WIDE_JIS_SYMBOL_DOWNWARDS_ARROW;
                is_pad = false;
            }
            NARROW_JIS_SYMBOL_FORMS_LIGHT_VERTICAL => {
                result = WIDE_JIS_SYMBOL_FORMS_LIGHT_VERTICAL;
                is_pad = false;
            }
            NARROW_JIS_SYMBOL_BLACK_SQUARE => {
                result = WIDE_JIS_SYMBOL_BLACK_SQUARE;
                is_pad = false;
            }
            NARROW_JIS_SYMBOL_WHITE_CIRCLE => {
                result = WIDE_JIS_SYMBOL_WHITE_CIRCLE;
                is_pad = false;
            }
            NARROW_JIS_SYMBOL_LEFT_WHITE_PARENTHESIS => {
                result = WIDE_JIS_SYMBOL_LEFT_WHITE_PARENTHESIS;
                is_pad = false;
            }
            NARROW_JIS_SYMBOL_RIGHT_WHITE_PARENTHESIS => {
                result = WIDE_JIS_SYMBOL_RIGHT_WHITE_PARENTHESIS;
                is_pad = false;
            }
            NARROW_JIS_SYMBOL_IDEOGRAPHIC_COMMA => {
                result = WIDE_JIS_SYMBOL_IDEOGRAPHIC_COMMA;
                is_pad = false;
            }
            NARROW_JIS_SYMBOL_IDEOGRAPHIC_FULL_STOP => {
                result = WIDE_JIS_SYMBOL_IDEOGRAPHIC_FULL_STOP;
                is_pad = false;
            }
            NARROW_JIS_SYMBOL_LEFT_CORNER_BRACKET => {
                result = WIDE_JIS_SYMBOL_LEFT_CORNER_BRACKET;
                is_pad = false;
            }
            NARROW_JIS_SYMBOL_RIGHT_CORNER_BRACKET => {
                result = WIDE_JIS_SYMBOL_RIGHT_CORNER_BRACKET;
                is_pad = false;
            }
            NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK => {
                result = WIDE_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK;
                is_pad = false;
            }
            NARROW_JIS_SYMBOL_KATAKANA_SEMI_VOICED_SOUND_MARK => {
                result = WIDE_JIS_SYMBOL_KATAKANA_SEMI_VOICED_SOUND_MARK;
                is_pad = false;
            }
            NARROW_JIS_SYMBOL_KATAKANA_MIDDLE_DOT => {
                result = WIDE_JIS_SYMBOL_KATAKANA_MIDDLE_DOT;
                is_pad = false;
            }
            NARROW_JIS_SYMBOL_PROLONGED_SOUND_MARK => {
                result = WIDE_JIS_SYMBOL_PROLONGED_SOUND_MARK;
                is_pad = false;
            }
            NARROW_JIS_SYMBOL_CENT_SIGN => {
                result = WIDE_JIS_SYMBOL_CENT_SIGN;
                is_pad = false;
            }
            NARROW_JIS_SYMBOL_POUND_SIGN => {
                result = WIDE_JIS_SYMBOL_POUND_SIGN;
                is_pad = false;
            }
            NARROW_JIS_SYMBOL_NOT_SIGN => {
                result = WIDE_JIS_SYMBOL_NOT_SIGN;
                is_pad = false;
            }
            NARROW_JIS_SYMBOL_MACRON => {
                result = WIDE_JIS_SYMBOL_MACRON;
                is_pad = false;
            }
            NARROW_JIS_SYMBOL_BROKEN_BAR => {
                result = WIDE_JIS_SYMBOL_BROKEN_BAR;
                is_pad = false;
            }
            NARROW_JIS_SYMBOL_YEN_SIGN => {
                result = WIDE_JIS_SYMBOL_YEN_SIGN;
                is_pad = false;
            }
            NARROW_JIS_SYMBOL_WON_SIGN => {
                result = WIDE_JIS_SYMBOL_WON_SIGN;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_KA => if next.is_match(NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK) {
                result = WIDE_KANA_KATAKANA_GA;
                is_pad = true;
            } else {
                result = WIDE_KANA_KATAKANA_KA;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_KI => if next.is_match(NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK) {
                result = WIDE_KANA_KATAKANA_GI;
                is_pad = true;
            } else {
                result = WIDE_KANA_KATAKANA_KI;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_KU => if next.is_match(NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK) {
                result = WIDE_KANA_KATAKANA_GU;
                is_pad = true;
            } else {
                result = WIDE_KANA_KATAKANA_KU;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_KE => if next.is_match(NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK) {
                result = WIDE_KANA_KATAKANA_GE;
                is_pad = true;
            } else {
                result = WIDE_KANA_KATAKANA_KE;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_KO => if next.is_match(NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK) {
                result = WIDE_KANA_KATAKANA_GO;
                is_pad = true;
            } else {
                result = WIDE_KANA_KATAKANA_KO;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_SA => if next.is_match(NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK) {
                result = WIDE_KANA_KATAKANA_ZA;
                is_pad = true;
            } else {
                result = WIDE_KANA_KATAKANA_SA;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_SI => if next.is_match(NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK) {
                result = WIDE_KANA_KATAKANA_ZI;
                is_pad = true;
            } else {
                result = WIDE_KANA_KATAKANA_SI;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_SU => if next.is_match(NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK) {
                result = WIDE_KANA_KATAKANA_ZU;
                is_pad = true;
            } else {
                result = WIDE_KANA_KATAKANA_SU;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_SE => if next.is_match(NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK) {
                result = WIDE_KANA_KATAKANA_ZE;
                is_pad = true;
            } else {
                result = WIDE_KANA_KATAKANA_SE;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_SO => if next.is_match(NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK) {
                result = WIDE_KANA_KATAKANA_ZO;
                is_pad = true;
            } else {
                result = WIDE_KANA_KATAKANA_SO;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_TA => if next.is_match(NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK) {
                result = WIDE_KANA_KATAKANA_DA;
                is_pad = true;
            } else {
                result = WIDE_KANA_KATAKANA_TA;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_TI => if next.is_match(NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK) {
                result = WIDE_KANA_KATAKANA_DI;
                is_pad = true;
            } else {
                result = WIDE_KANA_KATAKANA_TI;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_TU => if next.is_match(NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK) {
                result = WIDE_KANA_KATAKANA_DU;
                is_pad = true;
            } else {
                result = WIDE_KANA_KATAKANA_TU;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_TE => if next.is_match(NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK) {
                result = WIDE_KANA_KATAKANA_DE;
                is_pad = true;
            } else {
                result = WIDE_KANA_KATAKANA_TE;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_TO => if next.is_match(NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK) {
                result = WIDE_KANA_KATAKANA_DO;
                is_pad = true;
            } else {
                result = WIDE_KANA_KATAKANA_TO;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_HA => if next.is_match(NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK) {
                result = WIDE_KANA_KATAKANA_BA;
                is_pad = true;
            } else if next.is_match(NARROW_JIS_SYMBOL_KATAKANA_SEMI_VOICED_SOUND_MARK) {
                result = WIDE_KANA_KATAKANA_PA;
                is_pad = true;
            } else {
                result = WIDE_KANA_KATAKANA_HA;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_HI => if next.is_match(NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK) {
                result = WIDE_KANA_KATAKANA_BI;
                is_pad = true;
            } else if next.is_match(NARROW_JIS_SYMBOL_KATAKANA_SEMI_VOICED_SOUND_MARK) {
                result = WIDE_KANA_KATAKANA_PI;
                is_pad = true;
            } else {
                result = WIDE_KANA_KATAKANA_HI;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_HU => if next.is_match(NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK) {
                result = WIDE_KANA_KATAKANA_BU;
                is_pad = true;
            } else if next.is_match(NARROW_JIS_SYMBOL_KATAKANA_SEMI_VOICED_SOUND_MARK) {
                result = WIDE_KANA_KATAKANA_PU;
                is_pad = true;
            } else {
                result = WIDE_KANA_KATAKANA_HU;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_HE => if next.is_match(NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK) {
                result = WIDE_KANA_KATAKANA_BE;
                is_pad = true;
            } else if next.is_match(NARROW_JIS_SYMBOL_KATAKANA_SEMI_VOICED_SOUND_MARK) {
                result = WIDE_KANA_KATAKANA_PE;
                is_pad = true;
            } else {
                result = WIDE_KANA_KATAKANA_HE;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_HO => if next.is_match(NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK) {
                result = WIDE_KANA_KATAKANA_BO;
                is_pad = true;
            } else if next.is_match(NARROW_JIS_SYMBOL_KATAKANA_SEMI_VOICED_SOUND_MARK) {
                result = WIDE_KANA_KATAKANA_PO;
                is_pad = true;
            } else {
                result = WIDE_KANA_KATAKANA_HO;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_U => if next.is_match(NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK) {
                result = WIDE_KANA_KATAKANA_VU;
                is_pad = true;
            } else {
                result = WIDE_KANA_KATAKANA_U;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_WA => if next.is_match(NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK) {
                result = WIDE_KANA_KATAKANA_VA;
                is_pad = true;
            } else {
                result = WIDE_KANA_KATAKANA_WA;
                is_pad = false;
            }
            NARROW_KANA_KATAKANA_WO => if next.is_match(NARROW_JIS_SYMBOL_KATAKANA_VOICED_SOUND_MARK) {
                result = WIDE_KANA_KATAKANA_VO;
                is_pad = true;
            } else {
                result = WIDE_KANA_KATAKANA_WO;
                is_pad = false;
            }
            _ => {
                result = target_scalar;
                is_pad = false;
            }
        }

        return (T::from_scalar(result), is_pad);
    }
}
use kanaria::constants::*;
use kanaria::utils::AsciiUtils;

#[test]
pub fn is_narrow_lower_case() {
    check(IS_NARROW_LOWER_CASE_ACCEPT, IS_NARROW_LOWER_CASE_DENY, AsciiUtils::is_narrow_lower_case)
}

#[test]
pub fn is_wide_lower_case() {
    check(IS_WIDE_LOWER_CASE_ACCEPT, IS_WIDE_LOWER_CASE_DENY, AsciiUtils::is_wide_lower_case)
}

#[test]
pub fn is_narrow_upper_case() {
    check(IS_NARROW_UPPER_CASE_ACCEPT, IS_NARROW_UPPER_CASE_DENY, AsciiUtils::is_narrow_upper_case)
}

#[test]
pub fn is_wide_upper_case() {
    check(IS_WIDE_UPPER_CASE_ACCEPT, IS_WIDE_UPPER_CASE_DENY, AsciiUtils::is_wide_upper_case)
}

#[test]
pub fn is_narrow_ascii_symbol() {
    check(IS_NARROW_ASCII_SYMBOL_ACCEPT, IS_NARROW_ASCII_SYMBOL_DENY, AsciiUtils::is_narrow_ascii_symbol)
}

#[test]
pub fn is_wide_ascii_symbol() {
    check(IS_WIDE_ASCII_SYMBOL_ACCEPT, IS_WIDE_ASCII_SYMBOL_DENY, AsciiUtils::is_wide_ascii_symbol)
}

#[test]
pub fn is_narrow_number() {
    check(IS_NARROW_NUMBER_ACCEPT, IS_NARROW_NUMBER_DENY, AsciiUtils::is_narrow_number)
}

#[test]
pub fn is_wide_number() {
    check(IS_WIDE_NUMBER_ACCEPT, IS_WIDE_NUMBER_DENY, AsciiUtils::is_wide_number)
}

#[test]
pub fn convert_to_upper_case() {
    UPPER_LOWER_LIST.iter().for_each(|item| {
        assert_eq!(AsciiUtils::convert_to_upper_case(item.lower), item.upper)
    })
}

#[test]
pub fn convert_to_lower_case() {
    UPPER_LOWER_LIST.iter().for_each(|item| {
        assert_eq!(AsciiUtils::convert_to_lower_case(item.upper), item.lower)
    })
}

pub fn check(accept: &[u32], deny: &[u32], func: fn(u32) -> bool) {
    use std::char;
    accept.iter().for_each(|item| {
        println!("accept check ... 0x{:X} {:?}", *item, unsafe { char::from_u32_unchecked(*item) });
        assert_eq!(true, func(*item))
    });
    deny.iter().for_each(|item| {
        println!("accept check ... 0x{:X} {:?}", *item, unsafe { char::from_u32_unchecked(*item) });
        assert_eq!(false, func(*item))
    });
}

struct Alphabet {
    pub upper: char,
    pub lower: char,
}

impl Alphabet {
    pub const fn create(upper: char, lower: char) -> Self {
        Self {
            upper,
            lower,
        }
    }
}

static UPPER_LOWER_LIST: &[Alphabet] = &[
    Alphabet::create('A', 'a'),
    Alphabet::create('B', 'b'),
    Alphabet::create('C', 'c'),
    Alphabet::create('D', 'd'),
    Alphabet::create('E', 'e'),
    Alphabet::create('F', 'f'),
    Alphabet::create('G', 'g'),
    Alphabet::create('H', 'h'),
    Alphabet::create('I', 'i'),
    Alphabet::create('J', 'j'),
    Alphabet::create('K', 'k'),
    Alphabet::create('L', 'l'),
    Alphabet::create('M', 'm'),
    Alphabet::create('N', 'n'),
    Alphabet::create('O', 'o'),
    Alphabet::create('P', 'p'),
    Alphabet::create('Q', 'q'),
    Alphabet::create('R', 'r'),
    Alphabet::create('S', 's'),
    Alphabet::create('T', 't'),
    Alphabet::create('U', 'u'),
    Alphabet::create('V', 'v'),
    Alphabet::create('W', 'w'),
    Alphabet::create('X', 'x'),
    Alphabet::create('Y', 'y'),
    Alphabet::create('Z', 'z'),
    Alphabet::create('Ａ', 'ａ'),
    Alphabet::create('Ｂ', 'ｂ'),
    Alphabet::create('Ｃ', 'ｃ'),
    Alphabet::create('Ｄ', 'ｄ'),
    Alphabet::create('Ｅ', 'ｅ'),
    Alphabet::create('Ｆ', 'ｆ'),
    Alphabet::create('Ｇ', 'ｇ'),
    Alphabet::create('Ｈ', 'ｈ'),
    Alphabet::create('Ｉ', 'ｉ'),
    Alphabet::create('Ｊ', 'ｊ'),
    Alphabet::create('Ｋ', 'ｋ'),
    Alphabet::create('Ｌ', 'ｌ'),
    Alphabet::create('Ｍ', 'ｍ'),
    Alphabet::create('Ｎ', 'ｎ'),
    Alphabet::create('Ｏ', 'ｏ'),
    Alphabet::create('Ｐ', 'ｐ'),
    Alphabet::create('Ｑ', 'ｑ'),
    Alphabet::create('Ｒ', 'ｒ'),
    Alphabet::create('Ｓ', 'ｓ'),
    Alphabet::create('Ｔ', 'ｔ'),
    Alphabet::create('Ｕ', 'ｕ'),
    Alphabet::create('Ｖ', 'ｖ'),
    Alphabet::create('Ｗ', 'ｗ'),
    Alphabet::create('Ｘ', 'ｘ'),
    Alphabet::create('Ｙ', 'ｙ'),
    Alphabet::create('Ｚ', 'ｚ'),
];

static IS_NARROW_LOWER_CASE_ACCEPT: &[u32] = &[
    NARROW_ASCII_LATIN_LOWER_CASE_A,
    NARROW_ASCII_LATIN_LOWER_CASE_B,
    NARROW_ASCII_LATIN_LOWER_CASE_C,
    NARROW_ASCII_LATIN_LOWER_CASE_D,
    NARROW_ASCII_LATIN_LOWER_CASE_E,
    NARROW_ASCII_LATIN_LOWER_CASE_F,
    NARROW_ASCII_LATIN_LOWER_CASE_G,
    NARROW_ASCII_LATIN_LOWER_CASE_H,
    NARROW_ASCII_LATIN_LOWER_CASE_I,
    NARROW_ASCII_LATIN_LOWER_CASE_J,
    NARROW_ASCII_LATIN_LOWER_CASE_K,
    NARROW_ASCII_LATIN_LOWER_CASE_L,
    NARROW_ASCII_LATIN_LOWER_CASE_M,
    NARROW_ASCII_LATIN_LOWER_CASE_N,
    NARROW_ASCII_LATIN_LOWER_CASE_O,
    NARROW_ASCII_LATIN_LOWER_CASE_P,
    NARROW_ASCII_LATIN_LOWER_CASE_Q,
    NARROW_ASCII_LATIN_LOWER_CASE_R,
    NARROW_ASCII_LATIN_LOWER_CASE_S,
    NARROW_ASCII_LATIN_LOWER_CASE_T,
    NARROW_ASCII_LATIN_LOWER_CASE_U,
    NARROW_ASCII_LATIN_LOWER_CASE_V,
    NARROW_ASCII_LATIN_LOWER_CASE_W,
    NARROW_ASCII_LATIN_LOWER_CASE_X,
    NARROW_ASCII_LATIN_LOWER_CASE_Y,
    NARROW_ASCII_LATIN_LOWER_CASE_Z,

];
static IS_NARROW_LOWER_CASE_DENY: &[u32] = &[
    NARROW_ASCII_LATIN_LOWER_CASE_A - 1,
    NARROW_ASCII_LATIN_LOWER_CASE_Z + 1,

];

static IS_WIDE_LOWER_CASE_ACCEPT: &[u32] = &[
    WIDE_ASCII_LATIN_LOWER_CASE_A,
    WIDE_ASCII_LATIN_LOWER_CASE_B,
    WIDE_ASCII_LATIN_LOWER_CASE_C,
    WIDE_ASCII_LATIN_LOWER_CASE_D,
    WIDE_ASCII_LATIN_LOWER_CASE_E,
    WIDE_ASCII_LATIN_LOWER_CASE_F,
    WIDE_ASCII_LATIN_LOWER_CASE_G,
    WIDE_ASCII_LATIN_LOWER_CASE_H,
    WIDE_ASCII_LATIN_LOWER_CASE_I,
    WIDE_ASCII_LATIN_LOWER_CASE_J,
    WIDE_ASCII_LATIN_LOWER_CASE_K,
    WIDE_ASCII_LATIN_LOWER_CASE_L,
    WIDE_ASCII_LATIN_LOWER_CASE_M,
    WIDE_ASCII_LATIN_LOWER_CASE_N,
    WIDE_ASCII_LATIN_LOWER_CASE_O,
    WIDE_ASCII_LATIN_LOWER_CASE_P,
    WIDE_ASCII_LATIN_LOWER_CASE_Q,
    WIDE_ASCII_LATIN_LOWER_CASE_R,
    WIDE_ASCII_LATIN_LOWER_CASE_S,
    WIDE_ASCII_LATIN_LOWER_CASE_T,
    WIDE_ASCII_LATIN_LOWER_CASE_U,
    WIDE_ASCII_LATIN_LOWER_CASE_V,
    WIDE_ASCII_LATIN_LOWER_CASE_W,
    WIDE_ASCII_LATIN_LOWER_CASE_X,
    WIDE_ASCII_LATIN_LOWER_CASE_Y,
    WIDE_ASCII_LATIN_LOWER_CASE_Z,

];
static IS_WIDE_LOWER_CASE_DENY: &[u32] = &[
    WIDE_ASCII_LATIN_LOWER_CASE_A - 1,
    WIDE_ASCII_LATIN_LOWER_CASE_Z + 1,

];

static IS_NARROW_UPPER_CASE_ACCEPT: &[u32] = &[
    NARROW_ASCII_LATIN_UPPER_CASE_A,
    NARROW_ASCII_LATIN_UPPER_CASE_B,
    NARROW_ASCII_LATIN_UPPER_CASE_C,
    NARROW_ASCII_LATIN_UPPER_CASE_D,
    NARROW_ASCII_LATIN_UPPER_CASE_E,
    NARROW_ASCII_LATIN_UPPER_CASE_F,
    NARROW_ASCII_LATIN_UPPER_CASE_G,
    NARROW_ASCII_LATIN_UPPER_CASE_H,
    NARROW_ASCII_LATIN_UPPER_CASE_I,
    NARROW_ASCII_LATIN_UPPER_CASE_J,
    NARROW_ASCII_LATIN_UPPER_CASE_K,
    NARROW_ASCII_LATIN_UPPER_CASE_L,
    NARROW_ASCII_LATIN_UPPER_CASE_M,
    NARROW_ASCII_LATIN_UPPER_CASE_N,
    NARROW_ASCII_LATIN_UPPER_CASE_O,
    NARROW_ASCII_LATIN_UPPER_CASE_P,
    NARROW_ASCII_LATIN_UPPER_CASE_Q,
    NARROW_ASCII_LATIN_UPPER_CASE_R,
    NARROW_ASCII_LATIN_UPPER_CASE_S,
    NARROW_ASCII_LATIN_UPPER_CASE_T,
    NARROW_ASCII_LATIN_UPPER_CASE_U,
    NARROW_ASCII_LATIN_UPPER_CASE_V,
    NARROW_ASCII_LATIN_UPPER_CASE_W,
    NARROW_ASCII_LATIN_UPPER_CASE_X,
    NARROW_ASCII_LATIN_UPPER_CASE_Y,
    NARROW_ASCII_LATIN_UPPER_CASE_Z,

];
static IS_NARROW_UPPER_CASE_DENY: &[u32] = &[
    NARROW_ASCII_LATIN_UPPER_CASE_A - 1,
    NARROW_ASCII_LATIN_UPPER_CASE_Z + 1,

];

static IS_WIDE_UPPER_CASE_ACCEPT: &[u32] = &[
    WIDE_ASCII_LATIN_UPPER_CASE_A,
    WIDE_ASCII_LATIN_UPPER_CASE_B,
    WIDE_ASCII_LATIN_UPPER_CASE_C,
    WIDE_ASCII_LATIN_UPPER_CASE_D,
    WIDE_ASCII_LATIN_UPPER_CASE_E,
    WIDE_ASCII_LATIN_UPPER_CASE_F,
    WIDE_ASCII_LATIN_UPPER_CASE_G,
    WIDE_ASCII_LATIN_UPPER_CASE_H,
    WIDE_ASCII_LATIN_UPPER_CASE_I,
    WIDE_ASCII_LATIN_UPPER_CASE_J,
    WIDE_ASCII_LATIN_UPPER_CASE_K,
    WIDE_ASCII_LATIN_UPPER_CASE_L,
    WIDE_ASCII_LATIN_UPPER_CASE_M,
    WIDE_ASCII_LATIN_UPPER_CASE_N,
    WIDE_ASCII_LATIN_UPPER_CASE_O,
    WIDE_ASCII_LATIN_UPPER_CASE_P,
    WIDE_ASCII_LATIN_UPPER_CASE_Q,
    WIDE_ASCII_LATIN_UPPER_CASE_R,
    WIDE_ASCII_LATIN_UPPER_CASE_S,
    WIDE_ASCII_LATIN_UPPER_CASE_T,
    WIDE_ASCII_LATIN_UPPER_CASE_U,
    WIDE_ASCII_LATIN_UPPER_CASE_V,
    WIDE_ASCII_LATIN_UPPER_CASE_W,
    WIDE_ASCII_LATIN_UPPER_CASE_X,
    WIDE_ASCII_LATIN_UPPER_CASE_Y,
    WIDE_ASCII_LATIN_UPPER_CASE_Z,

];
static IS_WIDE_UPPER_CASE_DENY: &[u32] = &[
    WIDE_ASCII_LATIN_UPPER_CASE_A - 1,
    WIDE_ASCII_LATIN_UPPER_CASE_Z + 1,

];

static IS_NARROW_ASCII_SYMBOL_ACCEPT: &[u32] = &[
    NARROW_ASCII_SYMBOL_SPACE,
    NARROW_ASCII_SYMBOL_EXCLAMATION_MARK,
    NARROW_ASCII_SYMBOL_QUOTATION_MARK,
    NARROW_ASCII_SYMBOL_NUMBER_SIGN,
    NARROW_ASCII_SYMBOL_DOLLAR_SIGN,
    NARROW_ASCII_SYMBOL_PERCENT_SIGN,
    NARROW_ASCII_SYMBOL_AMPERSAND,
    NARROW_ASCII_SYMBOL_APOSTROPHE,
    NARROW_ASCII_SYMBOL_LEFT_PARENTHESIS,
    NARROW_ASCII_SYMBOL_RIGHT_PARENTHESIS,
    NARROW_ASCII_SYMBOL_ASTERISK,
    NARROW_ASCII_SYMBOL_PLUS_SIGN,
    NARROW_ASCII_SYMBOL_COMMA,
    NARROW_ASCII_SYMBOL_HYPHEN_MINUS,
    NARROW_ASCII_SYMBOL_FULL_STOP,
    NARROW_ASCII_SYMBOL_SOLIDUS,
    NARROW_ASCII_SYMBOL_COLON,
    NARROW_ASCII_SYMBOL_SEMICOLON,
    NARROW_ASCII_SYMBOL_LESS_THAN_SIGN,
    NARROW_ASCII_SYMBOL_EQUALS_SIGN,
    NARROW_ASCII_SYMBOL_GREATER_THAN_SIGN,
    NARROW_ASCII_SYMBOL_QUESTION_MARK,
    NARROW_ASCII_SYMBOL_COMMERCIAL_AT,
    NARROW_ASCII_SYMBOL_LEFT_SQUARE_BRACKET,
    NARROW_ASCII_SYMBOL_REVERSE_SOLIDUS,
    NARROW_ASCII_SYMBOL_RIGHT_SQUARE_BRACKET,
    NARROW_ASCII_SYMBOL_CIRCUMFLEX_ACCENT,
    NARROW_ASCII_SYMBOL_LOW_LINE,
    NARROW_ASCII_SYMBOL_GRAVE_ACCENT,
    NARROW_ASCII_SYMBOL_LEFT_CURLY_BRACKET,
    NARROW_ASCII_SYMBOL_VERTICAL_LINE,
    NARROW_ASCII_SYMBOL_RIGHT_CURLY_BRACKET,
    NARROW_ASCII_SYMBOL_TILDE,

];
static IS_NARROW_ASCII_SYMBOL_DENY: &[u32] = &[
    NARROW_ASCII_SYMBOL_SPACE - 1,
    NARROW_ASCII_SYMBOL_SOLIDUS + 1,
    NARROW_ASCII_SYMBOL_COLON - 1,
    NARROW_ASCII_SYMBOL_COMMERCIAL_AT + 1,
    NARROW_ASCII_SYMBOL_LEFT_SQUARE_BRACKET - 1,
    NARROW_ASCII_SYMBOL_GRAVE_ACCENT + 1,
    NARROW_ASCII_SYMBOL_LEFT_CURLY_BRACKET - 1,
    NARROW_ASCII_SYMBOL_TILDE + 1,

];

static IS_WIDE_ASCII_SYMBOL_ACCEPT: &[u32] = &[
    WIDE_ASCII_SYMBOL_SPACE,
    WIDE_ASCII_SYMBOL_EXCLAMATION_MARK,
    WIDE_ASCII_SYMBOL_QUOTATION_MARK,
    WIDE_ASCII_SYMBOL_NUMBER_SIGN,
    WIDE_ASCII_SYMBOL_DOLLAR_SIGN,
    WIDE_ASCII_SYMBOL_PERCENT_SIGN,
    WIDE_ASCII_SYMBOL_AMPERSAND,
    WIDE_ASCII_SYMBOL_APOSTROPHE,
    WIDE_ASCII_SYMBOL_LEFT_PARENTHESIS,
    WIDE_ASCII_SYMBOL_RIGHT_PARENTHESIS,
    WIDE_ASCII_SYMBOL_ASTERISK,
    WIDE_ASCII_SYMBOL_PLUS_SIGN,
    WIDE_ASCII_SYMBOL_COMMA,
    WIDE_ASCII_SYMBOL_HYPHEN_MINUS,
    WIDE_ASCII_SYMBOL_FULL_STOP,
    WIDE_ASCII_SYMBOL_SOLIDUS,
    WIDE_ASCII_SYMBOL_COLON,
    WIDE_ASCII_SYMBOL_SEMICOLON,
    WIDE_ASCII_SYMBOL_LESS_THAN_SIGN,
    WIDE_ASCII_SYMBOL_EQUALS_SIGN,
    WIDE_ASCII_SYMBOL_GREATER_THAN_SIGN,
    WIDE_ASCII_SYMBOL_QUESTION_MARK,
    WIDE_ASCII_SYMBOL_COMMERCIAL_AT,
    WIDE_ASCII_SYMBOL_LEFT_SQUARE_BRACKET,
    WIDE_ASCII_SYMBOL_REVERSE_SOLIDUS,
    WIDE_ASCII_SYMBOL_RIGHT_SQUARE_BRACKET,
    WIDE_ASCII_SYMBOL_CIRCUMFLEX_ACCENT,
    WIDE_ASCII_SYMBOL_LOW_LINE,
    WIDE_ASCII_SYMBOL_GRAVE_ACCENT,
    WIDE_ASCII_SYMBOL_LEFT_CURLY_BRACKET,
    WIDE_ASCII_SYMBOL_VERTICAL_LINE,
    WIDE_ASCII_SYMBOL_RIGHT_CURLY_BRACKET,
    WIDE_ASCII_SYMBOL_TILDE,

];
static IS_WIDE_ASCII_SYMBOL_DENY: &[u32] = &[
    WIDE_ASCII_SYMBOL_SPACE + 1,
    WIDE_ASCII_SYMBOL_EXCLAMATION_MARK - 1,
    WIDE_ASCII_SYMBOL_SOLIDUS + 1,
    WIDE_ASCII_SYMBOL_COLON - 1,
    WIDE_ASCII_SYMBOL_COMMERCIAL_AT + 1,
    WIDE_ASCII_SYMBOL_LEFT_SQUARE_BRACKET - 1,
    WIDE_ASCII_SYMBOL_GRAVE_ACCENT + 1,
    WIDE_ASCII_SYMBOL_LEFT_CURLY_BRACKET - 1,
    WIDE_ASCII_SYMBOL_TILDE + 1,

];

static IS_NARROW_NUMBER_ACCEPT: &[u32] = &[
    NARROW_ASCII_NUMBER_0,
    NARROW_ASCII_NUMBER_1,
    NARROW_ASCII_NUMBER_2,
    NARROW_ASCII_NUMBER_3,
    NARROW_ASCII_NUMBER_4,
    NARROW_ASCII_NUMBER_5,
    NARROW_ASCII_NUMBER_6,
    NARROW_ASCII_NUMBER_7,
    NARROW_ASCII_NUMBER_8,
    NARROW_ASCII_NUMBER_9,

];
static IS_NARROW_NUMBER_DENY: &[u32] = &[
    WIDE_ASCII_NUMBER_0 - 1,
    WIDE_ASCII_NUMBER_9 + 1,

];

static IS_WIDE_NUMBER_ACCEPT: &[u32] = &[
    WIDE_ASCII_NUMBER_0,
    WIDE_ASCII_NUMBER_1,
    WIDE_ASCII_NUMBER_2,
    WIDE_ASCII_NUMBER_3,
    WIDE_ASCII_NUMBER_4,
    WIDE_ASCII_NUMBER_5,
    WIDE_ASCII_NUMBER_6,
    WIDE_ASCII_NUMBER_7,
    WIDE_ASCII_NUMBER_8,
    WIDE_ASCII_NUMBER_9,

];
static IS_WIDE_NUMBER_DENY: &[u32] = &[
    NARROW_ASCII_NUMBER_0 - 1,
    NARROW_ASCII_NUMBER_9 + 1,

];
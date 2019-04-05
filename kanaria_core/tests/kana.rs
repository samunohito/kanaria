extern crate kanaria;

use kanaria::utils::KanaUtils;
use kanaria::constants::*;

#[test]
pub fn is_hiragana() {
    check(IS_HIRAGANA_ACCEPT, IS_HIRAGANA_DENY, KanaUtils::is_hiragana)
}

#[test]
pub fn is_narrow_katakana() {
    check(IS_NARROW_KATAKANA_ACCEPT, IS_NARROW_KATAKANA_DENY, KanaUtils::is_narrow_katakana)
}

#[test]
pub fn is_wide_katakana() {
    check(IS_WIDE_KATAKANA_ACCEPT, IS_WIDE_KATAKANA_DENY, KanaUtils::is_wide_katakana)
}

#[test]
pub fn is_narrow_jis_symbol() {
    // TODO:該当文字多すぎ…全部入れる？
}

#[test]
pub fn is_wide_jis_symbol() {
    // TODO:該当文字多すぎ…全部入れる？
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

#[test]
fn convert_to_hiragana() {
    HIRAGANA_KATAKANA_LIST.iter().for_each(|item| {
        assert_eq!(KanaUtils::convert_to_hiragana(item.katakana), item.hiragana)
    })
}

#[test]
fn convert_to_katakana() {
    HIRAGANA_KATAKANA_LIST.iter().for_each(|item| {
        assert_eq!(KanaUtils::convert_to_katakana(item.hiragana), item.katakana)
    })
}

struct Kana {
    pub hiragana: char,
    pub katakana: char,
}

impl Kana {
    pub const fn create(hiragana: char, katakana: char) -> Self {
        Self {
            hiragana,
            katakana,
        }
    }
}

static HIRAGANA_KATAKANA_LIST: &[Kana] = &[
    Kana::create('ぁ', 'ァ'),
    Kana::create('あ', 'ア'),
    Kana::create('ぃ', 'ィ'),
    Kana::create('い', 'イ'),
    Kana::create('ぅ', 'ゥ'),
    Kana::create('う', 'ウ'),
    Kana::create('ぇ', 'ェ'),
    Kana::create('え', 'エ'),
    Kana::create('ぉ', 'ォ'),
    Kana::create('ぐ', 'グ'),
    Kana::create('け', 'ケ'),
    Kana::create('げ', 'ゲ'),
    Kana::create('こ', 'コ'),
    Kana::create('ご', 'ゴ'),
    Kana::create('さ', 'サ'),
    Kana::create('ざ', 'ザ'),
    Kana::create('し', 'シ'),
    Kana::create('じ', 'ジ'),
    Kana::create('す', 'ス'),
    Kana::create('だ', 'ダ'),
    Kana::create('ち', 'チ'),
    Kana::create('ぢ', 'ヂ'),
    Kana::create('っ', 'ッ'),
    Kana::create('つ', 'ツ'),
    Kana::create('づ', 'ヅ'),
    Kana::create('て', 'テ'),
    Kana::create('で', 'デ'),
    Kana::create('と', 'ト'),
    Kana::create('ど', 'ド'),
    Kana::create('ば', 'バ'),
    Kana::create('ぱ', 'パ'),
    Kana::create('ひ', 'ヒ'),
    Kana::create('び', 'ビ'),
    Kana::create('ぴ', 'ピ'),
    Kana::create('ふ', 'フ'),
    Kana::create('ぶ', 'ブ'),
    Kana::create('ぷ', 'プ'),
    Kana::create('へ', 'ヘ'),
    Kana::create('べ', 'ベ'),
    Kana::create('む', 'ム'),
    Kana::create('め', 'メ'),
    Kana::create('も', 'モ'),
    Kana::create('ゃ', 'ャ'),
    Kana::create('や', 'ヤ'),
    Kana::create('ゅ', 'ュ'),
    Kana::create('ゆ', 'ユ'),
    Kana::create('ょ', 'ョ'),
    Kana::create('よ', 'ヨ'),
    Kana::create('ら', 'ラ'),
    Kana::create('ゐ', 'ヰ'),
    Kana::create('ゑ', 'ヱ'),
    Kana::create('を', 'ヲ'),
    Kana::create('ん', 'ン'),
    Kana::create('ゔ', 'ヴ'),
    Kana::create('ゕ', 'ヵ'),
    Kana::create('ゖ', 'ヶ'),
    Kana::create('お', 'オ'),
    Kana::create('か', 'カ'),
    Kana::create('が', 'ガ'),
    Kana::create('き', 'キ'),
    Kana::create('ぎ', 'ギ'),
    Kana::create('く', 'ク'),
    Kana::create('ず', 'ズ'),
    Kana::create('せ', 'セ'),
    Kana::create('ぜ', 'ゼ'),
    Kana::create('そ', 'ソ'),
    Kana::create('ぞ', 'ゾ'),
    Kana::create('た', 'タ'),
    Kana::create('な', 'ナ'),
    Kana::create('に', 'ニ'),
    Kana::create('ぬ', 'ヌ'),
    Kana::create('ね', 'ネ'),
    Kana::create('の', 'ノ'),
    Kana::create('は', 'ハ'),
    Kana::create('ぺ', 'ペ'),
    Kana::create('ほ', 'ホ'),
    Kana::create('ぼ', 'ボ'),
    Kana::create('ぽ', 'ポ'),
    Kana::create('ま', 'マ'),
    Kana::create('み', 'ミ'),
    Kana::create('り', 'リ'),
    Kana::create('る', 'ル'),
    Kana::create('れ', 'レ'),
    Kana::create('ろ', 'ロ'),
    Kana::create('ゎ', 'ヮ'),
    Kana::create('わ', 'ワ'),
    Kana::create('ゝ', 'ヽ'),
    Kana::create('ゞ', 'ヾ'),
    Kana::create('ゟ', 'ヿ'),
];

static IS_HIRAGANA_ACCEPT: &[u32] = &[
    WIDE_KANA_HIRAGANA_SMALL_A,
    WIDE_KANA_HIRAGANA_A,
    WIDE_KANA_HIRAGANA_SMALL_I,
    WIDE_KANA_HIRAGANA_I,
    WIDE_KANA_HIRAGANA_SMALL_U,
    WIDE_KANA_HIRAGANA_U,
    WIDE_KANA_HIRAGANA_SMALL_E,
    WIDE_KANA_HIRAGANA_E,
    WIDE_KANA_HIRAGANA_SMALL_O,
    WIDE_KANA_HIRAGANA_O,
    WIDE_KANA_HIRAGANA_KA,
    WIDE_KANA_HIRAGANA_GA,
    WIDE_KANA_HIRAGANA_KI,
    WIDE_KANA_HIRAGANA_GI,
    WIDE_KANA_HIRAGANA_KU,
    WIDE_KANA_HIRAGANA_GU,
    WIDE_KANA_HIRAGANA_KE,
    WIDE_KANA_HIRAGANA_GE,
    WIDE_KANA_HIRAGANA_KO,
    WIDE_KANA_HIRAGANA_GO,
    WIDE_KANA_HIRAGANA_SA,
    WIDE_KANA_HIRAGANA_ZA,
    WIDE_KANA_HIRAGANA_SI,
    WIDE_KANA_HIRAGANA_ZI,
    WIDE_KANA_HIRAGANA_SU,
    WIDE_KANA_HIRAGANA_ZU,
    WIDE_KANA_HIRAGANA_SE,
    WIDE_KANA_HIRAGANA_ZE,
    WIDE_KANA_HIRAGANA_SO,
    WIDE_KANA_HIRAGANA_ZO,
    WIDE_KANA_HIRAGANA_TA,
    WIDE_KANA_HIRAGANA_DA,
    WIDE_KANA_HIRAGANA_TI,
    WIDE_KANA_HIRAGANA_DI,
    WIDE_KANA_HIRAGANA_SMALL_TU,
    WIDE_KANA_HIRAGANA_TU,
    WIDE_KANA_HIRAGANA_DU,
    WIDE_KANA_HIRAGANA_TE,
    WIDE_KANA_HIRAGANA_DE,
    WIDE_KANA_HIRAGANA_TO,
    WIDE_KANA_HIRAGANA_DO,
    WIDE_KANA_HIRAGANA_NA,
    WIDE_KANA_HIRAGANA_NI,
    WIDE_KANA_HIRAGANA_NU,
    WIDE_KANA_HIRAGANA_NE,
    WIDE_KANA_HIRAGANA_NO,
    WIDE_KANA_HIRAGANA_HA,
    WIDE_KANA_HIRAGANA_BA,
    WIDE_KANA_HIRAGANA_PA,
    WIDE_KANA_HIRAGANA_HI,
    WIDE_KANA_HIRAGANA_BI,
    WIDE_KANA_HIRAGANA_PI,
    WIDE_KANA_HIRAGANA_HU,
    WIDE_KANA_HIRAGANA_BU,
    WIDE_KANA_HIRAGANA_PU,
    WIDE_KANA_HIRAGANA_HE,
    WIDE_KANA_HIRAGANA_BE,
    WIDE_KANA_HIRAGANA_PE,
    WIDE_KANA_HIRAGANA_HO,
    WIDE_KANA_HIRAGANA_BO,
    WIDE_KANA_HIRAGANA_PO,
    WIDE_KANA_HIRAGANA_MA,
    WIDE_KANA_HIRAGANA_MI,
    WIDE_KANA_HIRAGANA_MU,
    WIDE_KANA_HIRAGANA_ME,
    WIDE_KANA_HIRAGANA_MO,
    WIDE_KANA_HIRAGANA_SMALL_YA,
    WIDE_KANA_HIRAGANA_YA,
    WIDE_KANA_HIRAGANA_SMALL_YU,
    WIDE_KANA_HIRAGANA_YU,
    WIDE_KANA_HIRAGANA_SMALL_YO,
    WIDE_KANA_HIRAGANA_YO,
    WIDE_KANA_HIRAGANA_RA,
    WIDE_KANA_HIRAGANA_RI,
    WIDE_KANA_HIRAGANA_RU,
    WIDE_KANA_HIRAGANA_RE,
    WIDE_KANA_HIRAGANA_RO,
    WIDE_KANA_HIRAGANA_SMALL_WA,
    WIDE_KANA_HIRAGANA_WA,
    WIDE_KANA_HIRAGANA_WI,
    WIDE_KANA_HIRAGANA_WE,
    WIDE_KANA_HIRAGANA_WO,
    WIDE_KANA_HIRAGANA_N,
    WIDE_KANA_HIRAGANA_VU,
    WIDE_KANA_HIRAGANA_SMALL_KA,
    WIDE_KANA_HIRAGANA_SMALL_KE,
    WIDE_KANA_HIRAGANA_ITERATION_MARK,
    WIDE_KANA_HIRAGANA_VOICED_ITERATION_MARK,
    WIDE_KANA_HIRAGANA_DIGRAPH_YORI,
];
static IS_HIRAGANA_DENY: &[u32] = &[
    WIDE_KANA_HIRAGANA_SMALL_A - 1,
    WIDE_KANA_HIRAGANA_SMALL_KE + 1,
    WIDE_KANA_HIRAGANA_ITERATION_MARK - 1,
    WIDE_KANA_HIRAGANA_DIGRAPH_YORI + 1,

];

static IS_NARROW_KATAKANA_ACCEPT: &[u32] = &[
    NARROW_KANA_KATAKANA_WO,
    NARROW_KANA_KATAKANA_SMALL_A,
    NARROW_KANA_KATAKANA_SMALL_I,
    NARROW_KANA_KATAKANA_SMALL_U,
    NARROW_KANA_KATAKANA_SMALL_E,
    NARROW_KANA_KATAKANA_SMALL_O,
    NARROW_KANA_KATAKANA_SMALL_YA,
    NARROW_KANA_KATAKANA_SMALL_YU,
    NARROW_KANA_KATAKANA_SMALL_YO,
    NARROW_KANA_KATAKANA_SMALL_TU,
    NARROW_KANA_KATAKANA_A,
    NARROW_KANA_KATAKANA_I,
    NARROW_KANA_KATAKANA_U,
    NARROW_KANA_KATAKANA_E,
    NARROW_KANA_KATAKANA_O,
    NARROW_KANA_KATAKANA_KA,
    NARROW_KANA_KATAKANA_KI,
    NARROW_KANA_KATAKANA_KU,
    NARROW_KANA_KATAKANA_KE,
    NARROW_KANA_KATAKANA_KO,
    NARROW_KANA_KATAKANA_SA,
    NARROW_KANA_KATAKANA_SI,
    NARROW_KANA_KATAKANA_SU,
    NARROW_KANA_KATAKANA_SE,
    NARROW_KANA_KATAKANA_SO,
    NARROW_KANA_KATAKANA_TA,
    NARROW_KANA_KATAKANA_TI,
    NARROW_KANA_KATAKANA_TU,
    NARROW_KANA_KATAKANA_TE,
    NARROW_KANA_KATAKANA_TO,
    NARROW_KANA_KATAKANA_NA,
    NARROW_KANA_KATAKANA_NI,
    NARROW_KANA_KATAKANA_NU,
    NARROW_KANA_KATAKANA_NE,
    NARROW_KANA_KATAKANA_NO,
    NARROW_KANA_KATAKANA_HA,
    NARROW_KANA_KATAKANA_HI,
    NARROW_KANA_KATAKANA_HU,
    NARROW_KANA_KATAKANA_HE,
    NARROW_KANA_KATAKANA_HO,
    NARROW_KANA_KATAKANA_MA,
    NARROW_KANA_KATAKANA_MI,
    NARROW_KANA_KATAKANA_MU,
    NARROW_KANA_KATAKANA_ME,
    NARROW_KANA_KATAKANA_MO,
    NARROW_KANA_KATAKANA_YA,
    NARROW_KANA_KATAKANA_YU,
    NARROW_KANA_KATAKANA_YO,
    NARROW_KANA_KATAKANA_RA,
    NARROW_KANA_KATAKANA_RI,
    NARROW_KANA_KATAKANA_RU,
    NARROW_KANA_KATAKANA_RE,
    NARROW_KANA_KATAKANA_RO,
    NARROW_KANA_KATAKANA_WA,
    NARROW_KANA_KATAKANA_N,

];
static IS_NARROW_KATAKANA_DENY: &[u32] = &[
    NARROW_KANA_KATAKANA_WO - 1,
    NARROW_KANA_KATAKANA_SMALL_TU + 1,
    NARROW_KANA_KATAKANA_A - 1,
    NARROW_KANA_KATAKANA_N + 1,
];

static IS_WIDE_KATAKANA_ACCEPT: &[u32] = &[
    WIDE_KANA_KATAKANA_SMALL_A,
    WIDE_KANA_KATAKANA_A,
    WIDE_KANA_KATAKANA_SMALL_I,
    WIDE_KANA_KATAKANA_I,
    WIDE_KANA_KATAKANA_SMALL_U,
    WIDE_KANA_KATAKANA_U,
    WIDE_KANA_KATAKANA_SMALL_E,
    WIDE_KANA_KATAKANA_E,
    WIDE_KANA_KATAKANA_SMALL_O,
    WIDE_KANA_KATAKANA_O,
    WIDE_KANA_KATAKANA_KA,
    WIDE_KANA_KATAKANA_GA,
    WIDE_KANA_KATAKANA_KI,
    WIDE_KANA_KATAKANA_GI,
    WIDE_KANA_KATAKANA_KU,
    WIDE_KANA_KATAKANA_GU,
    WIDE_KANA_KATAKANA_KE,
    WIDE_KANA_KATAKANA_GE,
    WIDE_KANA_KATAKANA_KO,
    WIDE_KANA_KATAKANA_GO,
    WIDE_KANA_KATAKANA_SA,
    WIDE_KANA_KATAKANA_ZA,
    WIDE_KANA_KATAKANA_SI,
    WIDE_KANA_KATAKANA_ZI,
    WIDE_KANA_KATAKANA_SU,
    WIDE_KANA_KATAKANA_ZU,
    WIDE_KANA_KATAKANA_SE,
    WIDE_KANA_KATAKANA_ZE,
    WIDE_KANA_KATAKANA_SO,
    WIDE_KANA_KATAKANA_ZO,
    WIDE_KANA_KATAKANA_TA,
    WIDE_KANA_KATAKANA_DA,
    WIDE_KANA_KATAKANA_TI,
    WIDE_KANA_KATAKANA_DI,
    WIDE_KANA_KATAKANA_SMALL_TU,
    WIDE_KANA_KATAKANA_TU,
    WIDE_KANA_KATAKANA_DU,
    WIDE_KANA_KATAKANA_TE,
    WIDE_KANA_KATAKANA_DE,
    WIDE_KANA_KATAKANA_TO,
    WIDE_KANA_KATAKANA_DO,
    WIDE_KANA_KATAKANA_NA,
    WIDE_KANA_KATAKANA_NI,
    WIDE_KANA_KATAKANA_NU,
    WIDE_KANA_KATAKANA_NE,
    WIDE_KANA_KATAKANA_NO,
    WIDE_KANA_KATAKANA_HA,
    WIDE_KANA_KATAKANA_BA,
    WIDE_KANA_KATAKANA_PA,
    WIDE_KANA_KATAKANA_HI,
    WIDE_KANA_KATAKANA_BI,
    WIDE_KANA_KATAKANA_PI,
    WIDE_KANA_KATAKANA_HU,
    WIDE_KANA_KATAKANA_BU,
    WIDE_KANA_KATAKANA_PU,
    WIDE_KANA_KATAKANA_HE,
    WIDE_KANA_KATAKANA_BE,
    WIDE_KANA_KATAKANA_PE,
    WIDE_KANA_KATAKANA_HO,
    WIDE_KANA_KATAKANA_BO,
    WIDE_KANA_KATAKANA_PO,
    WIDE_KANA_KATAKANA_MA,
    WIDE_KANA_KATAKANA_MI,
    WIDE_KANA_KATAKANA_MU,
    WIDE_KANA_KATAKANA_ME,
    WIDE_KANA_KATAKANA_MO,
    WIDE_KANA_KATAKANA_SMALL_YA,
    WIDE_KANA_KATAKANA_YA,
    WIDE_KANA_KATAKANA_SMALL_YU,
    WIDE_KANA_KATAKANA_YU,
    WIDE_KANA_KATAKANA_SMALL_YO,
    WIDE_KANA_KATAKANA_YO,
    WIDE_KANA_KATAKANA_RA,
    WIDE_KANA_KATAKANA_RI,
    WIDE_KANA_KATAKANA_RU,
    WIDE_KANA_KATAKANA_RE,
    WIDE_KANA_KATAKANA_RO,
    WIDE_KANA_KATAKANA_SMALL_WA,
    WIDE_KANA_KATAKANA_WA,
    WIDE_KANA_KATAKANA_WI,
    WIDE_KANA_KATAKANA_WE,
    WIDE_KANA_KATAKANA_WO,
    WIDE_KANA_KATAKANA_N,
    WIDE_KANA_KATAKANA_VU,
    WIDE_KANA_KATAKANA_SMALL_KA,
    WIDE_KANA_KATAKANA_SMALL_KE,
    WIDE_KANA_KATAKANA_VA,
    WIDE_KANA_KATAKANA_VI,
    WIDE_KANA_KATAKANA_VE,
    WIDE_KANA_KATAKANA_VO,
    WIDE_KANA_KATAKANA_ITERATION_MARK,
    WIDE_KANA_KATAKANA_VOICED_ITERATION_MARK,
    WIDE_KANA_KATAKANA_DIGRAPH_YORI,

];
static IS_WIDE_KATAKANA_DENY: &[u32] = &[
    WIDE_KANA_KATAKANA_SMALL_A - 1,
    WIDE_KANA_KATAKANA_VO + 1,
    WIDE_KANA_KATAKANA_ITERATION_MARK - 1,
    WIDE_KANA_KATAKANA_DIGRAPH_YORI + 1,
];

//static IS_NARROW_JIS_SYMBOL_ACCEPT: &[u32] = &[
//
//];
//static IS_NARROW_JIS_SYMBOL_DENY: &[u32] = &[
//
//];
//
//static IS_WIDE_JIS_SYMBOL_ACCEPT: &[u32] = &[
//
//];
//static IS_WIDE_JIS_SYMBOL_DENY: &[u32] = &[
//
//];

extern crate kanaria;

use kanaria::string::{ConvertParameter, ConvertType, TextSearch, UCSStr};
use kanaria::utils::{CharExtend, CharsUtils, ConvertTarget};

#[test]
fn example_sentence_1() {
    let hiragana = "ちたたぷ　とてとて";
    let katakana = "チタタプ　トテトテ";

    assert_eq!(UCSStr::from_str(katakana).hiragana().to_string(), hiragana.to_string());
    assert_eq!(UCSStr::from_str(hiragana).katakana().to_string(), katakana.to_string());
}

#[test]
fn example_sentence_2() {
    let hiragana = "吾輩は😺猫である😺";
    let katakana = "吾輩ハ😺猫デアル😺";

    assert_eq!(UCSStr::from_str(katakana).hiragana().to_string(), hiragana.to_string());
    assert_eq!(UCSStr::from_str(hiragana).katakana().to_string(), katakana.to_string());
}

#[test]
fn example_sentence_3() {
    let hankaku = "ﾁﾀﾀﾌﾟ ﾄﾃﾄﾃFoooo!!!11!";
    let zenkaku = "チタタプ　トテトテＦｏｏｏｏ！！！１１！";

    assert_eq!(UCSStr::from_str(zenkaku).narrow(ConvertTarget::ALL).to_string(), hankaku.to_string());
    assert_eq!(UCSStr::from_str(hankaku).wide(ConvertTarget::ALL).to_string(), zenkaku.to_string());
}

#[test]
fn example_sentence_4() {
    let hankaku = "吾輩ﾊ😺猫ﾃﾞｱﾙ😺";
    let zenkaku = "吾輩ハ😺猫デアル😺";

    assert_eq!(UCSStr::from_str(zenkaku).narrow(ConvertTarget::ALL).to_string(), hankaku.to_string());
    assert_eq!(UCSStr::from_str(hankaku).wide(ConvertTarget::ALL).to_string(), zenkaku.to_string());
}

#[test]
fn example_sentence_5() {
    let source = "吾輩は😺猫である😺";
    let expect = "吾輩ﾊ😺猫ﾃﾞｱﾙ😺";
    let expect2 = "吾輩ハ😺猫デアル😺";

    assert_eq!(expect.to_string(), UCSStr::from_str(source).katakana().narrow(ConvertTarget::ALL).to_string());
    assert_eq!(expect2.to_string(), UCSStr::from_str(source).katakana().narrow(ConvertTarget::NUMBER | ConvertTarget::SYMBOL | ConvertTarget::ALPHABET).to_string());
}

#[test]
fn example_sentence10() {
    let source = "吾輩は😺猫である😺";
    let source_chars: Vec<char> = source.chars().map(|c| c).collect();
    assert_eq!(source_chars[3], '😺');
    assert_eq!(source_chars[8], '😺');

    let ucs_str = UCSStr::from_str(source);

    assert_eq!(true, ucs_str.is_contains("😺猫"));
    assert_eq!(false, ucs_str.is_contains("😺犬"));
    assert_eq!(ucs_str.index_of("😺猫"), 3);
    assert_eq!(ucs_str.index_of("😺犬"), -1);
    assert_eq!(ucs_str.index_of_all("😺"), vec![3, 8]);
    assert_eq!(ucs_str.index_of_all("犬"), vec![]);
}

#[test]
fn example_sentence11() {
    let mut source = "吾輩は😺猫である😺".to_char_vec();
    let act = CharsUtils::replace(source.as_mut_slice(), "😺".to_char_vec().as_slice(), "🐩".to_char_vec().as_slice());

    assert_eq!(act, "吾輩は🐩猫である🐩".to_char_vec());
    assert_eq!(source, "吾輩は😺猫である😺".to_char_vec());
}
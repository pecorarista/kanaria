extern crate kanaria;

use kanaria::UCSStr;

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

    assert_eq!(UCSStr::from_str(zenkaku).narrow().to_string(), hankaku.to_string());
    assert_eq!(UCSStr::from_str(hankaku).wide().to_string(), zenkaku.to_string());
}

#[test]
fn example_sentence_4() {
    let hankaku = "吾輩ﾊ😺猫ﾃﾞｱﾙ😺";
    let zenkaku = "吾輩ハ😺猫デアル😺";

    assert_eq!(UCSStr::from_str(zenkaku).narrow().to_string(), hankaku.to_string());
    assert_eq!(UCSStr::from_str(hankaku).wide().to_string(), zenkaku.to_string());
}

#[test]
fn example_sentence_5() {
    let source = "吾輩は😺猫である😺";
    let expect = "吾輩ﾊ😺猫ﾃﾞｱﾙ😺";

    assert_eq!(expect.to_string(), UCSStr::from_str(source).katakana().narrow().to_string());
}
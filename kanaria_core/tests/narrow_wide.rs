extern crate kanaria;

use kanaria::UCSChar;
use kanaria::utils::WidthUtils;

#[test]
fn convert_to_wide() {
    WIDE_NARROW_LIST.iter().for_each(|item| {
        let mut iter = item.narrow.chars();
        let first = if let Some(c) = iter.next() {
            c
        } else {
            panic!()
        };

        let second = if let Some(c) = iter.next() {
            c
        } else {
            char::NULL
        };

        let (result, _) = WidthUtils::convert_to_wide(first, second);
        assert_eq!(result.to_string().as_str(), item.wide)
    })
}

#[test]
fn convert_to_narrow() {
    WIDE_NARROW_LIST.iter().for_each(|item| {
        let (first, second) = WidthUtils::convert_to_narrow(item.wide.chars().next().unwrap());

        let mut result = first.to_string();
        if second != char::NULL {
            result.push(second);
        }

        assert_eq!(result.as_str(), item.narrow)
    })
}

struct Pair<'a> {
    pub wide: &'a str,
    pub narrow: &'a str,
}

impl<'a> Pair<'a> {
    pub const fn create(wide: &'a str, narrow: &'a str) -> Self {
        Self {
            wide,
            narrow,
        }
    }
}

static WIDE_NARROW_LIST: &[Pair] = &[
    Pair::create("←", "￩"),
    Pair::create("↑", "￪"),
    Pair::create("→", "￫"),
    Pair::create("↓", "￬"),
    Pair::create("│", "￨"),
    Pair::create("■", "￭"),
    Pair::create("○", "￮"),
    Pair::create("⦅", "｟"),
    Pair::create("⦆", "｠"),
    Pair::create("　", " "),
    Pair::create("、", "､"),
    Pair::create("。", "｡"),
    Pair::create("「", "｢"),
    Pair::create("」", "｣"),
    Pair::create("゙", "ﾞ"),
    Pair::create("゚", "ﾟ"),
    Pair::create("ァ", "ｧ"),
    Pair::create("ア", "ｱ"),
    Pair::create("ィ", "ｨ"),
    Pair::create("イ", "ｲ"),
    Pair::create("ゥ", "ｩ"),
    Pair::create("ウ", "ｳ"),
    Pair::create("ェ", "ｪ"),
    Pair::create("エ", "ｴ"),
    Pair::create("ォ", "ｫ"),
    Pair::create("オ", "ｵ"),
    Pair::create("カ", "ｶ"),
    Pair::create("キ", "ｷ"),
    Pair::create("ク", "ｸ"),
    Pair::create("ケ", "ｹ"),
    Pair::create("コ", "ｺ"),
    Pair::create("サ", "ｻ"),
    Pair::create("シ", "ｼ"),
    Pair::create("ス", "ｽ"),
    Pair::create("セ", "ｾ"),
    Pair::create("ソ", "ｿ"),
    Pair::create("タ", "ﾀ"),
    Pair::create("チ", "ﾁ"),
    Pair::create("ッ", "ｯ"),
    Pair::create("ツ", "ﾂ"),
    Pair::create("テ", "ﾃ"),
    Pair::create("ト", "ﾄ"),
    Pair::create("ナ", "ﾅ"),
    Pair::create("ニ", "ﾆ"),
    Pair::create("ヌ", "ﾇ"),
    Pair::create("ネ", "ﾈ"),
    Pair::create("ノ", "ﾉ"),
    Pair::create("ハ", "ﾊ"),
    Pair::create("ヒ", "ﾋ"),
    Pair::create("フ", "ﾌ"),
    Pair::create("ヘ", "ﾍ"),
    Pair::create("ホ", "ﾎ"),
    Pair::create("マ", "ﾏ"),
    Pair::create("ミ", "ﾐ"),
    Pair::create("ム", "ﾑ"),
    Pair::create("メ", "ﾒ"),
    Pair::create("モ", "ﾓ"),
    Pair::create("ャ", "ｬ"),
    Pair::create("ヤ", "ﾔ"),
    Pair::create("ュ", "ｭ"),
    Pair::create("ユ", "ﾕ"),
    Pair::create("ョ", "ｮ"),
    Pair::create("ヨ", "ﾖ"),
    Pair::create("ラ", "ﾗ"),
    Pair::create("リ", "ﾘ"),
    Pair::create("ル", "ﾙ"),
    Pair::create("レ", "ﾚ"),
    Pair::create("ロ", "ﾛ"),
    Pair::create("ワ", "ﾜ"),
    Pair::create("ヲ", "ｦ"),
    Pair::create("ン", "ﾝ"),
    Pair::create("・", "･"),
    Pair::create("ー", "ｰ"),
    Pair::create("！", "!"),
    Pair::create("＂", "\""),
    Pair::create("＃", "#"),
    Pair::create("＄", "$"),
    Pair::create("％", "%"),
    Pair::create("＆", "&"),
    Pair::create("＇", "'"),
    Pair::create("（", "("),
    Pair::create("）", ")"),
    Pair::create("＊", "*"),
    Pair::create("＋", "+"),
    Pair::create("，", ","),
    Pair::create("－", "-"),
    Pair::create("．", "."),
    Pair::create("／", "/"),
    Pair::create("０", "0"),
    Pair::create("１", "1"),
    Pair::create("２", "2"),
    Pair::create("３", "3"),
    Pair::create("４", "4"),
    Pair::create("５", "5"),
    Pair::create("６", "6"),
    Pair::create("７", "7"),
    Pair::create("８", "8"),
    Pair::create("９", "9"),
    Pair::create("：", ":"),
    Pair::create("；", ";"),
    Pair::create("＜", "<"),
    Pair::create("＝", "="),
    Pair::create("＞", ">"),
    Pair::create("？", "?"),
    Pair::create("＠", "@"),
    Pair::create("Ａ", "A"),
    Pair::create("Ｂ", "B"),
    Pair::create("Ｃ", "C"),
    Pair::create("Ｄ", "D"),
    Pair::create("Ｅ", "E"),
    Pair::create("Ｆ", "F"),
    Pair::create("Ｇ", "G"),
    Pair::create("Ｈ", "H"),
    Pair::create("Ｉ", "I"),
    Pair::create("Ｊ", "J"),
    Pair::create("Ｋ", "K"),
    Pair::create("Ｌ", "L"),
    Pair::create("Ｍ", "M"),
    Pair::create("Ｎ", "N"),
    Pair::create("Ｏ", "O"),
    Pair::create("Ｐ", "P"),
    Pair::create("Ｑ", "Q"),
    Pair::create("Ｒ", "R"),
    Pair::create("Ｓ", "S"),
    Pair::create("Ｔ", "T"),
    Pair::create("Ｕ", "U"),
    Pair::create("Ｖ", "V"),
    Pair::create("Ｗ", "W"),
    Pair::create("Ｘ", "X"),
    Pair::create("Ｙ", "Y"),
    Pair::create("Ｚ", "Z"),
    Pair::create("［", "["),
    Pair::create("＼", "\\"),
    Pair::create("］", "]"),
    Pair::create("＾", "^"),
    Pair::create("＿", "_"),
    Pair::create("｀", "`"),
    Pair::create("ａ", "a"),
    Pair::create("ｂ", "b"),
    Pair::create("ｃ", "c"),
    Pair::create("ｄ", "d"),
    Pair::create("ｅ", "e"),
    Pair::create("ｆ", "f"),
    Pair::create("ｇ", "g"),
    Pair::create("ｈ", "h"),
    Pair::create("ｉ", "i"),
    Pair::create("ｊ", "j"),
    Pair::create("ｋ", "k"),
    Pair::create("ｌ", "l"),
    Pair::create("ｍ", "m"),
    Pair::create("ｎ", "n"),
    Pair::create("ｏ", "o"),
    Pair::create("ｐ", "p"),
    Pair::create("ｑ", "q"),
    Pair::create("ｒ", "r"),
    Pair::create("ｓ", "s"),
    Pair::create("ｔ", "t"),
    Pair::create("ｕ", "u"),
    Pair::create("ｖ", "v"),
    Pair::create("ｗ", "w"),
    Pair::create("ｘ", "x"),
    Pair::create("ｙ", "y"),
    Pair::create("ｚ", "z"),
    Pair::create("｛", "{"),
    Pair::create("｜", "|"),
    Pair::create("｝", "}"),
    Pair::create("～", "~"),
    Pair::create("￠", "¢"),
    Pair::create("￡", "£"),
    Pair::create("￢", "¬"),
    Pair::create("￣", "¯"),
    Pair::create("￤", "¦"),
    Pair::create("￥", "¥"),
    Pair::create("￦", "₩"),
    Pair::create("ガ", "ｶﾞ"),
    Pair::create("ギ", "ｷﾞ"),
    Pair::create("グ", "ｸﾞ"),
    Pair::create("ゲ", "ｹﾞ"),
    Pair::create("ゴ", "ｺﾞ"),
    Pair::create("ザ", "ｻﾞ"),
    Pair::create("ジ", "ｼﾞ"),
    Pair::create("ズ", "ｽﾞ"),
    Pair::create("ゼ", "ｾﾞ"),
    Pair::create("ゾ", "ｿﾞ"),
    Pair::create("ダ", "ﾀﾞ"),
    Pair::create("ヂ", "ﾁﾞ"),
    Pair::create("ヅ", "ﾂﾞ"),
    Pair::create("デ", "ﾃﾞ"),
    Pair::create("ド", "ﾄﾞ"),
    Pair::create("バ", "ﾊﾞ"),
    Pair::create("パ", "ﾊﾟ"),
    Pair::create("ビ", "ﾋﾞ"),
    Pair::create("ピ", "ﾋﾟ"),
    Pair::create("ブ", "ﾌﾞ"),
    Pair::create("プ", "ﾌﾟ"),
    Pair::create("ベ", "ﾍﾞ"),
    Pair::create("ペ", "ﾍﾟ"),
    Pair::create("ボ", "ﾎﾞ"),
    Pair::create("ポ", "ﾎﾟ"),
    Pair::create("ヴ", "ｳﾞ"),
    Pair::create("ヷ", "ﾜﾞ"),
    Pair::create("ヺ", "ｦﾞ"),
];

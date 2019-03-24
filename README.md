Kanaria
====

このライブラリは、ひらがな・カタカナ、半角・全角の相互変換や判別を始めとした機能を提供します。

## Description

変換処理は次のものをご用意しています。

- 変換処理 
  - ひらがな<->カタカナの変換
  - 半角<->全角の変換（ガ<->ｶﾞのように、濁音記号の結合も行います）
  - アルファベットの大文字<->小文字変換

判定処理は次のものをご用意しています。<br>
ひがらなを除き、それぞれ半角のみ、全角のみ、半角・全角区別なしの物があります。
- 判定処理
  - ひらがな
  - カタカナ
  - 数字か
  - アルファベット
  - 記号

## Demo / Usage

UCSStrに文字列を読み込ませ、変換先の設定を行い（この例だとカタカナに変換後、さらに半角に変換）、<br>
Stringとして吐き出しているサンプルです。
```rust
let source = "吾輩は😺猫である😺";
let expect = "吾輩ﾊ😺猫ﾃﾞｱﾙ😺";

assert_eq!(expect.to_string(), UCSStr::from_str(source).katakana().narrow().to_string());
```

また、上記のようにメソッドチェーンによる連続した変換設定はできませんが、<br>
次のような形でも変換できます。<br>
```rust
use kanaria::converter::{Converter, ConverterFactory};
let target = vec!['あ', 'い', 'う', 'え', 'お'];
let mut result = Vec::<char>::with_capacity(target.len());
unsafe {
    // ほかにも、UCSStrと同じようにVec<T>やStringに出力する機能もあります
    let len = ConverterFactory::from_slice(target.as_slice())
        .katakana()
        .write_to_ptr(result.as_mut_ptr());
    result.set_len(len);
};
assert_eq!(result, vec!['ア', 'イ', 'ウ', 'エ', 'オ']);
```
この例は、半角文字を全角文字に変換しています。<br>
この形式での変換は生ポインタ（\*mut u16など)への書き込みを行うことができます。<br>
連続での変換はせず、なおかつ速度を求める場合はこちらのほうが便利です。<br>
<br>
ちなみに、他言語向けのラッパーライブラリも別途ご用意しています。<br>
以下はC#の例です（VB.NETからも同じ要領で使用できるはずです）。
```C#
var katakana = "吾輩ハ😺猫デアル😺";
var hiragana = "吾輩は😺猫である😺";
Assert.AreEqual(katakana, UcsString.From(hiragana).Katakana().ToString());
Assert.AreEqual(hiragana, UcsString.From(katakana).Hiragana().ToString());
```

## Installation
※準備中

## API
次のページをご参照ください（これはRust向けのものです）。<br>
https://sam-osamu.github.io/kanaria/docs/kanaria/index.html <br>
<br>
RustDocのようにサンプルを交えての記載まではできていませんが、<br>
各言語のラッパー関数にも同様の関数コメントをご用意しています。<br>

## Licence
[MIT](https://opensource.org/licenses/mit-license.php)

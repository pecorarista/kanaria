use std::slice::from_raw_parts;

use crate::converter::{Converter, ConverterFactory, ConverterUtils};
use crate::str::ConvertType::{Hiragana, Katakana, LowerCase, Narrow, None, UpperCase, Wide};
use crate::UCSChar;

enum ConvertType {
    /// アルファベット大文字に変換します。
    UpperCase,
    /// アルファベット小文字に変換します。
    LowerCase,
    /// ひらがなに変換します。
    Hiragana,
    /// カタカナに変換します。
    Katakana,
    /// 半角文字に変換します。
    Narrow,
    /// 全角文字に変換します。
    Wide,
    /// 変換操作を行いません。
    None,
}

pub struct UCSStr<T> where T: UCSChar {
    /// 変換対象文字列を保持します。
    target: Vec<T>,
    /// 変換先の種別を保持します。
    convet_types: Vec<ConvertType>,
}

impl UCSStr<u16> {
    /// 変換対象の文字列からUCSStr構造体を生成します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::UCSStr;
    ///
    /// let result = UCSStr::from_str("こんにちは");
    /// ```
    pub fn from_str<U>(s: &U) -> Self where U: AsRef<str> + ?Sized {
        Self {
            target: s.as_ref().encode_utf16().map(|scalar| { u16::from_scalar(scalar) }).collect::<Vec<u16>>(),
            convet_types: vec![],
        }
    }
}

impl<'a, T> UCSStr<T> where T: UCSChar {
    /// 変換対象の文字列からUCSStr構造体を生成します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::UCSStr;
    ///
    /// let target = vec!['あ', 'い', 'う', 'え', 'お'];
    /// let result = UCSStr::from_slice(target.as_slice());
    /// ```
    pub fn from_slice(source: &'a [T]) -> Self {
        Self {
            target: source.to_vec(),
            convet_types: vec![],
        }
    }

    /// 変換対象の文字列からUCSStr構造体を生成します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::UCSStr;
    ///
    /// let target = vec!['あ', 'い', 'う', 'え', 'お'];
    /// let result = unsafe { UCSStr::from_raw(target.as_ptr(), target.len()) };
    /// ```
    pub unsafe fn from_raw(source: *const T, len: usize) -> Self where T: UCSChar {
        Self::from_slice(from_raw_parts(source, len))
    }

    /// 文字列を大文字に変換するように設定します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::UCSStr;
    ///
    /// let target = vec!['a', 'b', 'c'];
    /// let result = UCSStr::from_slice(target.as_slice())
    ///     .upper_case()
    ///     .to_vec();
    ///
    /// assert_eq!(result, vec!['A', 'B', 'C']);
    /// ```
    pub fn upper_case(&mut self) -> &mut Self {
        self.convet_types.push(UpperCase);
        return self;
    }

    /// 文字列を小文字に変換するように設定します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::UCSStr;
    ///
    /// let target = vec!['A', 'B', 'C'];
    /// let result = UCSStr::from_slice(target.as_slice())
    ///     .lower_case()
    ///     .to_vec();
    ///
    /// assert_eq!(result, vec!['a', 'b', 'c']);
    /// ```
    pub fn lower_case(&mut self) -> &mut Self {
        self.convet_types.push(LowerCase);
        return self;
    }

    /// 文字列をひらがなに変換するように設定します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::UCSStr;
    ///
    /// let target = vec!['ア', 'イ', 'ウ', 'エ', 'オ'];
    /// let result = UCSStr::from_slice(target.as_slice())
    ///     .hiragana()
    ///     .to_vec();
    ///
    /// assert_eq!(result, vec!['あ', 'い', 'う', 'え', 'お']);
    /// ```
    pub fn hiragana(&mut self) -> &mut Self {
        self.convet_types.push(Hiragana);
        return self;
    }

    /// 文字列を全角カタカナに変換するように設定します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::UCSStr;
    ///
    /// let target = vec!['あ', 'い', 'う', 'え', 'お'];
    /// let result = UCSStr::from_slice(target.as_slice())
    ///     .katakana()
    ///     .to_vec();
    ///
    /// assert_eq!(result, vec!['ア', 'イ', 'ウ', 'エ', 'オ']);
    /// ```
    pub fn katakana(&mut self) -> &mut Self {
        self.convet_types.push(Katakana);
        return self;
    }

    /// 文字列を全角に変換するように設定します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::UCSStr;
    ///
    /// let target = vec!['ﾌ', 'ｼ', 'ﾞ', 'ｻ', 'ﾝ'];
    /// let result = UCSStr::from_slice(target.as_slice())
    ///     .wide()
    ///     .to_vec();
    ///
    /// assert_eq!(result.len(), 4);
    /// assert_eq!(result, vec!['フ','ジ','サ','ン']);
    /// ```
    pub fn wide(&mut self) -> &mut Self {
        self.convet_types.push(Wide);
        return self;
    }

    /// 文字列を半角に変換するように設定します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::UCSStr;
    ///
    /// let target = vec!['ガ', 'ギ', 'グ', 'ゲ', 'ゴ'];
    /// let result = UCSStr::from_slice(target.as_slice())
    ///     .narrow()
    ///     .to_vec();
    ///
    /// assert_eq!(result.len(), 10);
    /// assert_eq!(result, vec!['ｶ','ﾞ','ｷ','ﾞ','ｸ','ﾞ','ｹ','ﾞ','ｺ','ﾞ'])
    /// ```
    pub fn narrow(&mut self) -> &mut Self {
        self.convet_types.push(Narrow);
        return self;
    }

    /// 文字列を変換せず、そのままとするように設定します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::UCSStr;
    ///
    /// let target = vec!['あ', 'い', 'う', 'え', 'お'];
    /// let result1 = UCSStr::from_slice(target.as_slice())
    ///     .none()
    ///     .to_vec();
    ///
    /// assert_eq!(result1, vec!['あ', 'い', 'う', 'え', 'お']);
    ///
    /// // カタカナに変換後、何もしない設定なのでカタカナに変換された文字が出てくる
    /// let result2 = UCSStr::from_slice(target.as_slice())
    ///     .katakana()
    ///     .none()
    ///     .to_vec();
    ///
    /// assert_eq!(result2, vec!['ア', 'イ', 'ウ', 'エ', 'オ']);
    /// ```
    pub fn none(&mut self) -> &mut Self {
        self.convet_types.push(None);
        return self;
    }

    /// 文字列を変換し、Stringとして返却します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::UCSStr;
    ///
    /// let target = vec!['あ', 'い', 'う', 'え', 'お'];
    /// let result = UCSStr::from_slice(target.as_slice())
    ///     .katakana()
    ///     .to_string();
    ///
    /// assert_eq!(result.as_str(), "アイウエオ")
    /// ```
    pub fn to_string(&self) -> String {
        ConverterUtils::build_string(self.to_vec())
    }

    /// 文字列を変換し、Vecとして返却します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::UCSStr;
    ///
    /// let target = vec!['あ', 'い', 'う', 'え', 'お'];
    /// let result = UCSStr::from_slice(target.as_slice())
    ///     .katakana()
    ///     .to_vec();
    ///
    /// assert_eq!(result, vec!['ア', 'イ', 'ウ', 'エ', 'オ'])
    /// ```
    pub fn to_vec(&self) -> Vec<T> {
        let mut buffer = self.target.clone();

        self.convet_types.iter().for_each(|convert_type| {
            let factory = ConverterFactory::<T>::from_slice(buffer.as_slice());
            buffer = match convert_type {
                UpperCase => factory.upper_case().to_vec(),
                LowerCase => factory.lower_case().to_vec(),
                Hiragana => factory.hiragana().to_vec(),
                Katakana => factory.katakana().to_vec(),
                Wide => factory.wide().to_vec(),
                Narrow => factory.narrow().to_vec(),
                None => factory.none().to_vec(),
            }
        });

        return buffer;
    }
}

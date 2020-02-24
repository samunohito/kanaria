use utils::ConvertTarget;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ConvertType {
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

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ConvertParameter {
    /// 変換先の種別を保持します。
    pub convert_type: ConvertType,
    /// 半角・全角変換の対象を設定します。
    /// 大文字・小文字、ひらがな・かたかな変換時は使用されません。
    pub width_convert_target: ConvertTarget,
}

impl ConvertParameter {
    /// 変換時のパラメータを設定します。
    pub fn from(convert_type: ConvertType, width_convert_target: ConvertTarget) -> Self {
        Self {
            convert_type,
            width_convert_target,
        }
    }
}
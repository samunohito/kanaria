use crate::UCSChar;

pub trait Converter<T> where T: UCSChar {
    /// 文字列を変換し、Stringとして返却します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::converter::{Converter, ConverterFactory};
    ///
    /// let target = vec!['あ', 'い', 'う', 'え', 'お'];
    /// let result = ConverterFactory::from_slice(target.as_slice())
    ///     .katakana()
    ///     .to_string();
    ///
    /// assert_eq!(result.as_str(), "アイウエオ")
    /// ```
    fn to_string(&mut self) -> String;

    /// 文字列を変換し、Vecとして返却します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::converter::{Converter, ConverterFactory};
    ///
    /// let target = vec!['あ', 'い', 'う', 'え', 'お'];
    /// let result = ConverterFactory::from_slice(target.as_slice())
    ///     .katakana()
    ///     .to_vec();
    ///
    /// assert_eq!(result, vec!['ア', 'イ', 'ウ', 'エ', 'オ'])
    /// ```
    fn to_vec(&mut self) -> Vec<T>;

    /// 文字列を変換し、target_ptrに書き込みます。
    /// target_ptrは、元になった文字列と同じ長さ以上の領域を設定する必要があります。
    /// また、この関数は終端文字のnullを書き込みません。
    /// 必要な場合は呼び出し側でtarget_ptrの終端にnullを追加してください。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::converter::{Converter, ConverterFactory};
    ///
    /// let target = vec!['あ', 'い', 'う', 'え', 'お'];
    /// let mut result = Vec::<char>::with_capacity(target.len());
    ///
    /// unsafe {
    ///     let len = ConverterFactory::from_slice(target.as_slice())
    ///         .katakana()
    ///         .write_to_ptr(result.as_mut_ptr());
    ///     result.set_len(len);
    /// };
    ///
    /// assert_eq!(result, vec!['ア', 'イ', 'ウ', 'エ', 'オ']);
    /// ```
    unsafe fn write_to_ptr(&self, target_ptr: *mut T) -> usize;
}
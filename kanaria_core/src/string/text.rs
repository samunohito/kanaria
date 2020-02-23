use string::string::UCSStr;
use utils::{CharsUtils, CharExtend};

pub trait TextSearch<T> {
    /// このインスタンスの中に引数に与えられた文字列が含まれているかをチェックします。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::string::{UCSStr,TextSearch};
    ///
    /// let target = UCSStr::from_str("あいうえお");
    /// assert_eq!(target.is_contains("うえ"), true);
    /// assert_eq!(target.is_contains("うえか"), false);
    /// ```
    fn is_contains(&self, search: T) -> bool {
        self.index_of(search) >= 0
    }
    /// 引数に与えられた文字列が登場する位置を返します。
    /// 見つからない場合、-1を返却します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::string::{UCSStr,TextSearch};
    ///
    /// let target = UCSStr::from_str("あいうえお");
    /// assert_eq!(target.index_of("うえ"), 3);
    /// assert_eq!(target.index_of("うえか"), -1);
    /// ```
    fn index_of(&self, search: T) -> isize;
    /// 引数に与えられた文字列が登場するすべての位置を返します。
    /// 見つからない場合、空のVecを返却します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::string::{UCSStr,TextSearch};
    ///
    /// let target = UCSStr::from_str("あいうえおあいうえお");
    /// assert_eq!(target.index_of_all("うえ"), vec![3, 8]);
    /// assert_eq!(target.index_of_all("うえか"), vec![]);
    /// ```
    fn index_of_all(&self, search: T) -> Vec<usize>;
}

impl TextSearch<&str> for UCSStr<char> {
    fn index_of(&self, search: &str) -> isize {
        CharsUtils::index_of(self.target.as_slice(), search.to_char_vec().as_slice())
    }

    fn index_of_all(&self, search: &str) -> Vec<usize> {
        CharsUtils::index_of_all(self.target.as_slice(), search.to_char_vec().as_slice())
    }
}

impl TextSearch<&str> for UCSStr<u16> {
    fn index_of(&self, search: &str) -> isize {
        CharsUtils::index_of(self.target.as_slice(), search.to_u16_vec().as_slice())
    }

    fn index_of_all(&self, search: &str) -> Vec<usize> {
        CharsUtils::index_of_all(self.target.as_slice(), search.to_u16_vec().as_slice())
    }
}

impl TextSearch<&str> for UCSStr<u32> {
    fn index_of(&self, search: &str) -> isize {
        CharsUtils::index_of(self.target.as_slice(), search.to_u32_vec().as_slice())
    }

    fn index_of_all(&self, search: &str) -> Vec<usize> {
        CharsUtils::index_of_all(self.target.as_slice(), search.to_u32_vec().as_slice())
    }
}

pub trait TextEdit<T, U> {
    fn replace(&self, old_text: T, new_text: T) -> U;
    fn reverse(&self) -> U;
}

impl TextEdit<&str, String> for UCSStr<u16> {
    fn replace(&self, old_text: &str, new_text: &str) -> String {
        unimplemented!()
    }

    fn reverse(&self) -> String {
        unimplemented!()
    }
}

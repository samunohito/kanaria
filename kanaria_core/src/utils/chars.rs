use std::collections::VecDeque;

use UCSChar;
use utils::{AsciiUtils, KanaUtils};

pub struct CharsUtils;

impl CharsUtils {
    /// 半角文字かどうかを判定します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::utils::CharsUtils;
    ///
    /// assert_eq!(CharsUtils::is_narrow('a'), true);
    /// assert_eq!(CharsUtils::is_narrow('A'), true);
    /// assert_eq!(CharsUtils::is_narrow('@'), true);
    /// assert_eq!(CharsUtils::is_narrow('0'), true);
    /// assert_eq!(CharsUtils::is_narrow('ｱ'), true);
    /// assert_eq!(CharsUtils::is_narrow('ｰ'), true);
    ///
    /// assert_eq!(CharsUtils::is_narrow('ａ'), false);
    /// assert_eq!(CharsUtils::is_narrow('Ａ'), false);
    /// assert_eq!(CharsUtils::is_narrow('＠'), false);
    /// assert_eq!(CharsUtils::is_narrow('０'), false);
    /// assert_eq!(CharsUtils::is_narrow('ア'), false);
    /// assert_eq!(CharsUtils::is_narrow('ー'), false);
    /// ```
    #[inline]
    pub fn is_narrow<T>(target: T) -> bool where T: UCSChar {
        AsciiUtils::is_narrow_ascii(target) ||
            KanaUtils::is_narrow_katakana(target) ||
            KanaUtils::is_narrow_jis_symbol(target)
    }

    /// 全角文字かどうかを判定します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::utils::CharsUtils;
    ///
    /// assert_eq!(CharsUtils::is_wide('ａ'), true);
    /// assert_eq!(CharsUtils::is_wide('Ａ'), true);
    /// assert_eq!(CharsUtils::is_wide('＠'), true);
    /// assert_eq!(CharsUtils::is_wide('０'), true);
    /// assert_eq!(CharsUtils::is_wide('ア'), true);
    /// assert_eq!(CharsUtils::is_wide('ー'), true);
    /// 
    /// assert_eq!(CharsUtils::is_wide('a'), false);
    /// assert_eq!(CharsUtils::is_wide('A'), false);
    /// assert_eq!(CharsUtils::is_wide('@'), false);
    /// assert_eq!(CharsUtils::is_wide('0'), false);
    /// assert_eq!(CharsUtils::is_wide('ｱ'), false);
    /// assert_eq!(CharsUtils::is_wide('ｰ'), false);
    ///
    /// ```
    #[inline]
    pub fn is_wide<T>(target: T) -> bool where T: UCSChar {
        AsciiUtils::is_wide_ascii(target) ||
            KanaUtils::is_wide_katakana(target) ||
            KanaUtils::is_wide_jis_symbol(target)
    }

    /// searchの中に対象を含むかどうかを判定します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::utils::CharsUtils;
    /// use kanaria::constants::*;
    ///
    /// let true_case = CharsUtils::is_contains('A', &['A', 'B', 'C']);
    /// assert_eq!(true_case, true);
    ///
    /// let false_case = CharsUtils::is_contains('D', &['A', 'B', 'C']);
    /// assert_eq!(false_case, false);
    /// ```
    #[inline]
    pub fn is_contains<T>(target: T, search: &[T]) -> bool where T: UCSChar {
        search.contains(&target)
    }

    /// 引数に与えられた文字列が登場する位置を返します。
    /// 見つからない場合、-1を返却します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::string::{UCSStr,TextSearch};
    /// use kanaria::utils::{CharsUtils, CharExtend};
    ///
    /// let target = "あいうえお".to_char_vec().as_slice();
    /// assert_eq!(CharsUtils::index_of(target, "うえ".to_char_vec().as_slice()), 3);
    /// assert_eq!(CharsUtils::index_of(target, "うえか".to_char_vec().as_slice()), -1);
    /// ```
    #[inline]
    pub fn index_of<T>(target: &[T], search: &[T]) -> isize where T: UCSChar {
        if search.len() <= 0 {
            return -1;
        }

        let mut pos: usize = 0;
        loop {
            let first = target.iter().skip(pos).position(|&c| c == search[0]);
            if first.is_none() {
                return -1;
            }

            pos = pos + first.unwrap();
            let last_idx = search.len();
            for i in 0..last_idx {
                let iter_pos = pos + i;
                if iter_pos >= target.len() {
                    return -1;
                } else if target[iter_pos] != search[i] {
                    pos = iter_pos + 1;
                    break;
                } else if (last_idx - 1) == i && target[iter_pos] == search[i] {
                    return pos as isize;
                }
            }
        }
    }

    /// 引数に与えられた文字列が登場するすべての位置を返します。
    /// 見つからない場合、空のVecを返却します。
    ///
    /// # Examples
    ///
    /// ```
    /// use kanaria::string::{UCSStr, TextSearch};
    /// use kanaria::utils::{CharsUtils, CharExtend};
    ///
    /// let target = "あいうえおあいうえお".to_char_vec().as_slice();
    /// assert_eq!(CharsUtils::index_of_all(target, "うえ".to_char_vec().as_slice()), vec![3, 8]);
    /// assert_eq!(CharsUtils::index_of_all(target, "うえか".to_char_vec().as_slice()), vec![]);
    /// ```
    #[inline]
    pub fn index_of_all<T>(target: &[T], search: &[T]) -> Vec<usize> where T: UCSChar {
        if search.len() <= 0 {
            return vec![];
        }

        let mut ret: Vec<usize> = vec![];
        let mut pos: usize = 0;
        loop {
            let first = target.iter().skip(pos).position(|&c| c == search[0]);
            if first.is_none() {
                return ret;
            }

            pos = pos + first.unwrap();
            let last_idx = search.len();
            for i in 0..last_idx {
                let iter_pos = pos + i;
                if iter_pos >= target.len() {
                    return ret;
                } else if target[iter_pos] != search[i] {
                    pos = iter_pos + 1;
                    break;
                } else if (last_idx - 1) == i && target[iter_pos] == search[i] {
                    ret.push(pos);
                    pos = iter_pos + 1;
                    break;
                }
            }
        }
    }

    pub fn reverse<T>(target: &[T]) -> Vec<T> where T: UCSChar {
        let mut rev = Vec::from(target);
        rev.reverse();

        return rev;
    }

    pub fn replace<T>(target: &[T], old_char: &[T], new_char: &[T]) -> Vec<T> where T: UCSChar {
        let mut poses = VecDeque::from(CharsUtils::index_of_all(target, old_char));
        if poses.is_empty() {
            return Vec::from(target);
        }

        let len_diff = (old_char.len() - new_char.len()) * poses.len();
        let new_len = target.len() - len_diff;
        let mut ret: Vec<T> = Vec::with_capacity(new_len);

        let mut target_pos: usize = 0;
        let mut old_char_pos: usize = poses.pop_front().unwrap() as usize;
        loop {
            if target_pos >= new_len {
                break;
            } else if target_pos == old_char_pos {
                new_char.iter().for_each(|c| ret.push(*c));
                target_pos += old_char.len();
                let pos = poses.pop_front();
                if pos.is_some() {
                    old_char_pos = pos.unwrap() as usize;
                }
            } else {
                let sub_opt = target.get(target_pos..old_char_pos);
                if sub_opt.is_some() {
                    let sub = sub_opt.unwrap();
                    sub.iter().for_each(|c| ret.push(*c));
                    target_pos += sub.len();
                }
            }
        }

        return ret;
    }
}
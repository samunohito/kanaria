using System;
using System.Collections.Generic;
using System.Linq;
using Kanaria.Common.Generic;
using Kanaria.Common.Linq;

namespace Kanaria.KanaConverter.Internal
{
    internal static class Resources
    {
        /// <summary>
        /// 同じインデックスの文字同士でペアを生成する
        /// </summary>
        private static readonly Func<char, char, Pair<char, char>> PAIR_MAKER_CC = 
            (first, second) => new Pair<char, char>(first, second);

        /// <summary>
        /// お互いに対応するstring値同士でペアを生成する
        /// </summary>
        private static readonly Func<string, string, Pair<string, string>> PAIR_MAKER_SS = 
            (first, second) => new Pair<string, string>(first, second);

        /// <summary>
        /// KanaType一覧
        /// </summary>
        public static readonly IEnumerable<KanaType> KANA_TYPE_LIST =
            Enum.GetValues(typeof(KanaType))
                .Cast<KanaType>()
                .ToArray();

        /// <summary>
        /// WidthType一覧
        /// </summary>
        public static readonly IEnumerable<WidthType> WIDTH_TYPE_LIST =
            Enum.GetValues(typeof(WidthType))
                .Cast<WidthType>()
                .ToArray();

        /// <summary>
        /// ひらがなと濁音をセットにして1つのstringへ分割した中間値配列
        /// </summary>
        public static readonly string[] HIRAGANA_ZEN_INCOMPLETE_DAKUON_CHUNK = 
            Const.HIRAGANA_ZEN_INCOMPLETE_DAKUON
                .Chunk(2)
                .Select(x => new string(x.ToArray()))
                .ToArray();

        /// <summary>
        /// 全角カタカナと濁音をセットにして1つのstringへ分割した中間値配列
        /// </summary>
        public static readonly string[] KATAKANA_ZEN_INCOMPLETE_DAKUON_CHUNK = 
            Const.KATAKANA_ZEN_INCOMPLETE_DAKUON
                .Chunk(2)
                .Select(x => new string(x.ToArray()))
                .ToArray();

        /// <summary>
        /// 半角カタカナと濁音をセットにして1つのstringへ分割した中間値配列
        /// </summary>
        public static readonly string[] KATAKANA_HAN_INCOMPLETE_DAKUON_CHUNK =
            Const.KATAKANA_HAN_DAKUON
                .Chunk(2)
                .Select(x => new string(x.ToArray()))
                .ToArray();

        /// <summary>
        /// ひらがな<->かたかな用テーブル
        /// </summary>
        public static readonly IDictionary<KanaType, IEnumerable<Pair<char, char>>> KANA_TABLE =
            new Dictionary<KanaType, IEnumerable<Pair<char, char>>>()
            {
                {
                    KanaType.Hiragana,
                    Enumerable.Zip(Const.KATAKANA_ZEN_SEION + Const.KATAKANA_ZEN_DAKUON,
                        Const.HIRAGANA_SEION + Const.HIRAGANA_DAKUON, PAIR_MAKER_CC).ToArray()
                },
                {
                    KanaType.Katakana,
                    Enumerable.Zip(Const.HIRAGANA_SEION + Const.HIRAGANA_DAKUON,
                        Const.KATAKANA_ZEN_SEION + Const.KATAKANA_ZEN_DAKUON, PAIR_MAKER_CC).ToArray()
                },
            };

        /// <summary>
        /// 半角<->全角用テーブル
        /// </summary>
        public static readonly IDictionary<KanaType, IDictionary<WidthType, IEnumerable<Pair<char, char>>>> WIDTH_TABLE
            = new Dictionary<KanaType, IDictionary<WidthType, IEnumerable<Pair<char, char>>>>()
            {
                {
                    KanaType.Katakana,
                    new Dictionary<WidthType, IEnumerable<Pair<char, char>>>()
                    {
                        {
                            WidthType.Wide,
                            Enumerable.Zip(Const.KATAKANA_HAN_SEION, Const.KATAKANA_ZEN_SEION, PAIR_MAKER_CC).ToArray()
                        },
                        {
                            WidthType.Narrow,
                            Enumerable.Zip(Const.KATAKANA_ZEN_SEION, Const.KATAKANA_HAN_SEION, PAIR_MAKER_CC).ToArray()
                        },
                    }
                },
                {
                    KanaType.Eisuu,
                    new Dictionary<WidthType, IEnumerable<Pair<char, char>>>()
                    {
                        {
                            WidthType.Wide, 
                            Enumerable.Zip(Const.EISUU_HAN, Const.EISUU_ZEN, PAIR_MAKER_CC).ToArray()
                        },
                        {
                            WidthType.Narrow, 
                            Enumerable.Zip(Const.EISUU_ZEN, Const.EISUU_HAN, PAIR_MAKER_CC).ToArray()
                        },
                    }
                },
                {
                    KanaType.Kigou,
                    new Dictionary<WidthType, IEnumerable<Pair<char, char>>>()
                    {
                        {
                            WidthType.Wide, 
                            Enumerable.Zip(Const.KIGOU_HAN, Const.KIGOU_ZEN, PAIR_MAKER_CC).ToArray()
                        },
                        {
                            WidthType.Narrow, 
                            Enumerable.Zip(Const.KIGOU_ZEN, Const.KIGOU_HAN, PAIR_MAKER_CC).ToArray()
                        },
                    }
                }
            };

        /// <summary>
        /// 大文字<->小文字変換
        /// </summary>
        public static readonly IDictionary<LetterType, IEnumerable<Pair<char, char>>> LETTER_TABLE =
            new Dictionary<LetterType, IEnumerable<Pair<char, char>>>()
            {
                {
                    LetterType.Lower,
                    Enumerable.Zip(Const.ALPHABET_OOMOJI, Const.ALPHABET_KOMOJI, PAIR_MAKER_CC).ToArray()
                },
                {
                    LetterType.Upper,
                    Enumerable.Zip(Const.ALPHABET_KOMOJI, Const.ALPHABET_OOMOJI, PAIR_MAKER_CC).ToArray()
                }
            };

        /// <summary>
        /// 濁音つき仮名->半角カタカナ＋濁音へ
        /// </summary>
        public static readonly IDictionary<KanaType, IEnumerable<Pair<string, string>>> DAKUON_SPLIT_TABLE =
            new Dictionary<KanaType, IEnumerable<Pair<string, string>>>()
            {
//                半角カタカナ→ひらがなへの直接変換パターンも考慮すると煩雑になるので、一旦半角・全角のカタカナ同士での変換のみ考慮する。
//                {
//                    KanaType.Hiragana,
//                    Enumerable.Zip(Const.HIRAGANA_DAKUON.Select(x => x.ToString()).ToArray(), KATAKANA_HAN_INCOMPLETE_DAKUON_CHUNK, PAIR_MAKER_SS).ToArray()
//                },
                {
                    KanaType.Katakana,
                    Enumerable.Zip(Const.KATAKANA_ZEN_DAKUON.Select(x => x.ToString()).ToArray(), KATAKANA_HAN_INCOMPLETE_DAKUON_CHUNK, PAIR_MAKER_SS).ToArray()
                }
            };
        
        /// <summary>
        /// 全角カナ＋全角化された濁音->通常の濁音仮名へ
        /// </summary>
        public static readonly IDictionary<KanaType, IEnumerable<Pair<string, string>>> DAKUON_CONCAT_TABLE = 
            new Dictionary<KanaType, IEnumerable<Pair<string, string>>>()
            {
//                半角カタカナ→ひらがなへの直接変換パターンも考慮すると煩雑になるので、一旦半角・全角のカタカナ同士での変換のみ考慮する。
//                {
//                    KanaType.Hiragana,
//                    Enumerable.Zip(HIRAGANA_ZEN_INCOMPLETE_DAKUON_CHUNK, Const.HIRAGANA_DAKUON.Select(x => x.ToString()).ToArray(), PAIR_MAKER_SS).ToArray()
//                },
                {
                    KanaType.Katakana,
                    Enumerable.Zip(KATAKANA_ZEN_INCOMPLETE_DAKUON_CHUNK, Const.KATAKANA_ZEN_DAKUON.Select(x => x.ToString()).ToArray(), PAIR_MAKER_SS).ToArray()
                }
            };
    }
}
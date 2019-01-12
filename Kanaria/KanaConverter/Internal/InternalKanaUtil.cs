using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using Kanaria.Common.Generic;

namespace Kanaria.KanaConverter.Internal
{
    internal static class InternalKanaUtil
    {
        /// <summary>
        /// かな・カナ変換、半角全角変換の共通部分を関数化
        /// </summary>
        /// <param name="source"></param>
        /// <param name="pair"></param>
        /// <returns></returns>
        private static string CommonTypeReplace(string source, IEnumerable<Pair<char, char>> pair)
        {
            //結果格納用
            var sb = new StringBuilder(source, source.Length);

            //対象検索用
            var search = new HashSet<char>(source);

            //置換対象を抽出し、抽出されたもののみ置換操作
            pair
                .Where(x => search.Contains(x.First))
                .ToList()
                .ForEach(x => sb.Replace(x.First, x.Second));

            return sb.ToString();
        }

        /// <summary>
        /// かな・カナ変換、半角全角変換の共通部分を関数化
        /// </summary>
        /// <param name="source"></param>
        /// <param name="pair"></param>
        /// <returns></returns>
        private static string CommonTypeReplace(string source, IEnumerable<Pair<string, string>> pair)
        {
            //結果格納用
            var sb = new StringBuilder(source, source.Length);

            //置換対象を抽出し、抽出されたもののみ置換操作
            pair
                .Where(x => source.Contains(x.First))
                .ToList()
                .ForEach(x => sb.Replace(x.First, x.Second));

            return sb.ToString();
        }

        /// <summary>
        /// かな<->カナ
        /// </summary>
        /// <param name="target"></param>
        /// <param name="requestType"></param>
        /// <returns></returns>
        public static string KanaTypeReplace(string target, KanaType requestType)
        {
            return CommonTypeReplace(target, Resources.KANA_TABLE[requestType]);
        }

        /// <summary>
        /// 半角<->全角
        /// </summary>
        /// <param name="target"></param>
        /// <param name="requestKanaType"></param>
        /// <param name="requestWidthType"></param>
        /// <param name="requestBackSlashType"></param>
        /// <returns></returns>
        public static string WidthTypeReplace(string target, KanaType requestKanaType, WidthType requestWidthType, BackSlashType requestBackSlashType = BackSlashType.BackSlash)
        {
            // フラグが立っているカナ種別の半角・全角テーブルを取得し、平坦化
            var requestTables = Resources.KANA_TYPE_LIST
                .Select(x => requestKanaType & x)
                .Where(x => x != KanaType.None)
                .Select(x => Resources.WIDTH_TABLE[x][requestWidthType])
                .SelectMany(x => x)
                .ToArray();

            // ひとまず単純置換。細かい部分は後の処理でサポート。
            var intermediate = CommonTypeReplace(target, requestTables);
            
            if ((requestKanaType & KanaType.Katakana) != KanaType.None)
            {
                // カタカナの場合、変換前・変換後で文字数が変わる部分（濁音）があり、共通関数はそこをカバーしていないので追加処理
                switch (requestWidthType)
                {
                    case WidthType.Narrow:
                        intermediate = CommonTypeReplace(intermediate, Resources.DAKUON_SPLIT_TABLE[KanaType.Katakana]);
                        break;
                    case WidthType.Wide:
                        intermediate = CommonTypeReplace(intermediate, Resources.DAKUON_CONCAT_TABLE[KanaType.Katakana]);
                        break;
                    default:
                        throw new ArgumentOutOfRangeException(nameof(requestWidthType), requestWidthType, null);
                }
            }

            if ((requestKanaType & KanaType.Kigou) != KanaType.None)
            {
                // 記号の場合、バックスラッシュと円マークは設定により変換先を分ける。
                // バックスラッシュと円マークは、半角は同じ文字として扱われるが全角では別の文字として扱われるため。
                if (requestWidthType == WidthType.Wide)
                {
                    intermediate = (requestBackSlashType == BackSlashType.Yen)
                        ? intermediate.Replace(Const.NALLOW_YEN_BACKSLASH, Const.WIDE_YEN)
                        : intermediate.Replace(Const.NALLOW_YEN_BACKSLASH, Const.WIDE_BACKSLASH);
                }
                else
                {
                    intermediate = intermediate
                        .Replace(Const.WIDE_YEN, Const.NALLOW_YEN_BACKSLASH)
                        .Replace(Const.WIDE_BACKSLASH, Const.NALLOW_YEN_BACKSLASH);
                }
            }

            return intermediate;
        }

        /// <summary>
        /// 大文字<->小文字
        /// </summary>
        /// <param name="source"></param>
        /// <param name="requestType"></param>
        /// <returns></returns>
        public static string LetterTypeReplace(string source, LetterType requestType)
        {
            return CommonTypeReplace(source, Resources.LETTER_TABLE[requestType]);
        }
    }
}
using System.Runtime.InteropServices;

namespace Kanaria.Utils
{
    public static class WidthUtils
    {
        /// <summary>
        /// 全角文字を半角に変換します。
        /// 一文字目と二文字目が結合可能だった場合、結合済みの文字列を返却します。
        /// 文字列が結合された場合、2つ目の戻り値にtrueが入っています。
        /// </summary>
        /// <param name="target">変換対象文字</param>
        /// <param name="next">次の文字。濁音・半濁音のように結合可能な文字の場合、targetとこの値を結合する。</param>
        /// <param name="is_pad">結合あり:true / 結合なし:false</param>
        /// <returns>変換後文字</returns>
        [DllImport("kanaria.dll", EntryPoint = "convert_to_wide_for_utf16",
            CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Unicode)]
        [return:MarshalAs(UnmanagedType.U2)]
        public static extern char ConvertToWide(char target, char next, out bool is_pad);

        /// <summary>
        /// 全角文字を半角に変換します。
        /// 全角カタカナの濁音文字など、戻り値が2文字分になる場合もあります。
        /// 戻り値が1文字のときは2文字目にはヌル文字相当の値が入っています。
        /// </summary>
        /// <param name="target">変換対象文字</param>
        /// <param name="second">濁音記号等で増えた文字。増えなかった場合はnull文字（\u0000）。</param>
        /// <returns>変換後文字</returns>
        [DllImport("kanaria.dll", EntryPoint = "convert_to_narrow_for_utf16",
            CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Unicode)]
        [return:MarshalAs(UnmanagedType.U2)]
        public static extern char ConvertToNarrow(char target, out char second);
    }
}
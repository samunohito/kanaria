using System;

namespace Kanaria.KanaConverter
{
    /// <summary>
    /// かな種別を表します.
    /// </summary>
    [Flags]
    public enum KanaType
    {
        /// <summary>
        /// 定義なし。
        /// ビットフラグとしても使用するため便宜上定義するが、設定しても何も起こらない。
        /// </summary>
        None = 0x0,

        /// <summary>
        /// ひらがな
        /// </summary>
        Hiragana = 0x1,

        /// <summary>
        /// カタカナ
        /// </summary>
        Katakana = 0x2,

        /// <summary>
        /// 英数
        /// </summary>
        Eisuu = 0x4,

        /// <summary>
        /// 記号
        /// </summary>
        Kigou = 0x8,
    }
}
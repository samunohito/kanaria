using System;

namespace Kanaria
{
    [Flags]
    public enum ConvertTarget
    {
        /// <summary>
        /// 半角・全角の変換対象として数値を設定する際のビットフラグです。
        /// </summary>
        Number = 0b00000001,
        /// <summary>
        /// 半角・全角の変換対象としてアルファベットを設定する際のビットフラグです。
        /// </summary>
        Alphabet = 0b00000010,
        /// <summary>
        /// 半角・全角の変換対象として記号を設定する際のビットフラグです。
        /// </summary>
        Symbol = 0b00000100,
        /// <summary>
        /// 半角・全角の変換対象としてカタカナを設定する際のビットフラグです。
        /// </summary>
        Katakana = 0b00001000,
        /// <summary>
        /// 半角・全角の変換が可能なものはすべて変換します。
        /// </summary>
        All = (Number | Alphabet| Symbol| Katakana)
    }
}
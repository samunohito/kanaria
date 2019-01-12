namespace Kanaria.KanaConverter
{
    /// <summary>
    /// バックスラッシュ及び円マークの半角・全角変換時動作を表します.
    /// </summary>
    public enum BackSlashType
    {
        /// <summary>
        /// 半角の円マーク（バックスラッシュ）は全角の円マークに変換されます
        /// </summary>
        Yen,
        /// <summary>
        /// 半角の円マーク（バックスラッシュ）は全角のバックスラッシュに変換されます
        /// </summary>
        BackSlash,
    }
}
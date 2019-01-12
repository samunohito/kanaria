namespace Kanaria.KanaConverter.Internal
{
    internal static class Const
    {
        /// <summary>
        /// LineFeed
        /// </summary>
        public const string LF = "\n";

        /// <summary>
        /// CarriageReturn
        /// </summary>
        public const string CR = "\r";

        /// <summary>
        /// CarriageReturn + LineFeed
        /// </summary>
        public const string CRLF = CR + LF;

        /// <summary>
        /// Tab
        /// </summary>
        public const string TAB = "\t";
        
        /// <summary>
        /// 半角円マーク（バックスラッシュ）
        /// </summary>
        public const char NALLOW_YEN_BACKSLASH = '\\';

        /// <summary>
        /// 全角バックスラッシュ
        /// </summary>
        public const char WIDE_BACKSLASH = '＼';
        
        /// <summary>
        /// 全角円マーク
        /// </summary>
        public const char WIDE_YEN = '￥';

        /// <summary>
        /// ひらがな清音
        /// </summary>
        public const string HIRAGANA_SEION = @"ぁあぃいぅうぇえぉおかきくけこさしすせそたちっつてとなにぬねのはひふへほまみむめもゃやゅゆょよらりるれろゎわゐゑをん";

        /// <summary>
        /// ひらがな濁音
        /// </summary>
        public const string HIRAGANA_DAKUON = @"がぎぐげござじずぜぞだぢづでどばぱびぴぶぷべぺぼぽゔ";

        /// <summary>
        /// 全角カタカナ清音
        /// </summary>
        public const string KATAKANA_ZEN_SEION = @"ァアィイゥウェエォオカキクケコサシスセソタチッツテトナニヌネノハヒフヘホマミムメモャヤュユョヨラリルレロヮワヰヱヲン";

        /// <summary>
        /// 全角カタカナ濁音
        /// </summary>
        public const string KATAKANA_ZEN_DAKUON = @"ガギグゲゴザジズゼゾダヂヅデドバパビピブプベペボポヴ";

        /// <summary>
        /// 半角カタカナ清音
        /// </summary>
        public const string KATAKANA_HAN_SEION = @"ｧｱｨｲｩｳｪｴｫｵｶｷｸｹｺｻｼｽｾｿﾀﾁｯﾂﾃﾄﾅﾆﾇﾈﾉﾊﾋﾌﾍﾎﾏﾐﾑﾒﾓｬﾔｭﾕｮﾖﾗﾘﾙﾚﾛヮﾜヰヱｦﾝ";

        /// <summary>
        /// 半角カタカナ濁音
        /// </summary>
        public const string KATAKANA_HAN_DAKUON = @"ｶﾞｷﾞｸﾞｹﾞｺﾞｻﾞｼﾞｽﾞｾﾞｿﾞﾀﾞﾁﾞﾂﾞﾃﾞﾄﾞﾊﾞﾊﾟﾋﾞﾋﾟﾌﾞﾌﾟﾍﾞﾍﾟﾎﾞﾎﾟｳﾞ";

        /// <summary>
        /// 半角カタカナ濁音<->全角カタカナ濁音変換用中間値
        /// </summary>
        public const string KATAKANA_ZEN_INCOMPLETE_DAKUON = @"カ゛キ゛ク゛ケ゛コ゛サ゛シ゛ス゛セ゛ソ゛タ゛チ゛ツ゛テ゛ト゛ハ゛ハ゜ヒ゛ヒ゜フ゛フ゜ヘ゛ヘ゜ホ゛ホ゜ウ゛";

        /// <summary>
        /// 半角カタカナ濁音<->ひらがな濁音変換用中間値
        /// </summary>
        public const string HIRAGANA_ZEN_INCOMPLETE_DAKUON = @"か゛き゛く゛け゛こ゛さ゛し゛す゛せ゛そ゛た゛ち゛つ゛て゛と゛は゛は゜ひ゛ひ゜ふ゛ふ゜へ゛へ゜ほ゛ほ゜う゛";

        /// <summary>
        /// 英数字全角
        /// </summary>
        public const string EISUU_ZEN = @"ＡＢＣＤＥＦＧＨＩＪＫＬＭＮＯＰＱＲＳＴＵＶＷＸＹＺａｂｃｄｅｆｇｈｉｊｋｌｍｎｏｐｑｒｓｔｕｖｗｘｙｚ０１２３４５６７８９";

        /// <summary>
        /// 英数字半角
        /// </summary>
        public const string EISUU_HAN = @"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

        /// <summary>
        /// アルファベット小文字（半角・全角両方）
        /// </summary>
        public const string ALPHABET_KOMOJI = @"abcdefghijklmnopqrstuvwxyzａｂｃｄｅｆｇｈｉｊｋｌｍｎｏｐｑｒｓｔｕｖｗｘｙｚ";

        /// <summary>
        /// アルファベット大文字（半角・全角両方）
        /// </summary>
        public const string ALPHABET_OOMOJI = @"ABCDEFGHIJKLMNOPQRSTUVWXYZＡＢＣＤＥＦＧＨＩＪＫＬＭＮＯＰＱＲＳＴＵＶＷＸＹＺ";

        /// <summary>
        /// 記号全角。バックスラッシュ・円マークはロジックで対応する。
        /// </summary>
        public const string KIGOU_ZEN = "・／＠．＿ー，？＆％！”’（）＃「」［］｛｝゛゜、。　";

        /// <summary>
        /// 記号半角。バックスラッシュ・円マークはロジックで対応する。
        /// </summary>
        public const string KIGOU_HAN = "･/@._ｰ,?&%!\"'()#｢｣[]{}ﾞﾟ､｡ ";
    }
}
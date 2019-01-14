namespace KanariaBenchmark.Common.Generic
{
    /// <summary>
    /// 対になるデータを表します。
    /// </summary>
    /// <typeparam name="TFirst">型</typeparam>
    /// <typeparam name="TSecond">型</typeparam>
    public class Pair<TFirst, TSecond>
    {
        /// <summary>
        /// 一方の値
        /// </summary>
        public TFirst First { get; set; }
        /// <summary>
        /// 一方の値
        /// </summary>
        public TSecond Second { get; set; }
        /// <summary>
        /// コンストラクタ
        /// </summary>
        /// <param name="f">一方の値</param>
        /// <param name="s">一方の値</param>
        public Pair(TFirst f, TSecond s)
        {
            First = f;
            Second = s;
        }
    }
}

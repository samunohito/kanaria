using System;
using System.Collections.Generic;

namespace Kanaria.Common.Linq.Internal
{
    internal class ChunkSlicer<T> : IDisposable
    {
        private readonly IEnumerator<T> _enumerator = null;
        private readonly int _chunkSize = 0;

        internal ChunkSlicer(IEnumerator<T> enumerator, int chunkSize)
        {
            if (chunkSize <= 0)
            {
                throw new ArgumentException("must be 1 or more.", nameof(chunkSize));
            }

            _enumerator = enumerator ?? throw new ArgumentNullException(nameof(enumerator));
            _chunkSize = chunkSize;
            
            // IEnumerator#Currentへの初回アクセス前に必ず呼ぶ必要があるため.
            // IEnumerator#MoveNext()前にIEnumerator#Currentを呼んだときの動作は未定義とのことで、何が起こるかわからない.
            MoveNext();
        }
        
        /// <summary>
        /// 次があるかどうかを取得する
        /// </summary>
        public bool HasNext { get; private set; }

        /// <summary>
        /// IEnumerator#MoveNext()のラッパメソッド.
        /// 同メソッドの戻り値をキャッシュし、Currentにアクセス可能かどうかを再取得可能にさせる.
        /// </summary>
        /// <returns></returns>
        private bool MoveNext()
        {
            HasNext = _enumerator.MoveNext();
            return HasNext;
        }

        /// <summary>
        /// シーケンスの現在位置から特定サイズを切り出す.
        /// 切り出しつつ列挙するため、呼び出すたびにシーケンスのポインタが進む.
        /// </summary>
        /// <returns></returns>
        public IEnumerable<T> Slice()
        {
            var cnt = 0;
            do
            {
                yield return _enumerator.Current;
                cnt++;
            } while (MoveNext() && cnt < _chunkSize);
        }

        public void Dispose()
        {
            _enumerator?.Dispose();
        }
    }
}
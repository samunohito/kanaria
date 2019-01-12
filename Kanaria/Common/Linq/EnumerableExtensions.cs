using System;
using System.Collections.Generic;
using Kanaria.Common.Linq.Internal;

namespace Kanaria.Common.Linq
{
    public static class EnumerableExtensions
    {
        /// <summary>
        /// シーケンスを指定されたサイズのチャンクに分割します.
        /// </summary>
        /// <param name="self">対象</param>
        /// <param name="chunkSize">分割数</param>
        /// <typeparam name="T">シーケンスの型</typeparam>
        /// <returns></returns>
        public static IEnumerable<IEnumerable<T>> Chunk<T>(this IEnumerable<T> self, int chunkSize)
        {
            using (var slicer = new ChunkSlicer<T>(self.GetEnumerator(), chunkSize))
            {
                while (slicer.HasNext)
                {
                    yield return slicer.Slice();
                }
            }
        }
    }
}
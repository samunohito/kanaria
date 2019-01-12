using System.Collections.Generic;
using Kanaria.KanaConverter;
using NUnit.Framework;
using TestProject1.ForTest;

namespace KanariaTest
{
    public class KanaConverterTestEisuuKigou
    {
        [Test]
        public void Hankaku_To_Zenkaku_Yen()
        {
            var list = new List<NarrowWidePair>(Const.ZEN_HAN_LIST);
            list.Add(new NarrowWidePair() {Hankaku = "\\", Zenkaku = "￥"});

            foreach (var item in list)
            {
                Assert.AreEqual(item.Zenkaku,
                    KanaConverter.ToWide(item.Hankaku, requestBackSlashType: BackSlashType.Yen));
            }
        }

        [Test]
        public void Hankaku_To_Zenkaku_BackSlash()
        {
            var list = new List<NarrowWidePair>(Const.ZEN_HAN_LIST);
            list.Add(new NarrowWidePair() {Hankaku = "\\", Zenkaku = "＼"});

            foreach (var item in list)
            {
                Assert.AreEqual(item.Zenkaku,
                    KanaConverter.ToWide(item.Hankaku, requestBackSlashType: BackSlashType.BackSlash));
            }
        }

        [Test]
        public void Zenkaku_Yen_To_Hankaku()
        {
            var list = new List<NarrowWidePair>(Const.ZEN_HAN_LIST);
            list.Add(new NarrowWidePair() {Hankaku = "\\", Zenkaku = "￥"});

            foreach (var item in list)
            {
                Assert.AreEqual(item.Hankaku, KanaConverter.ToNarrow(item.Zenkaku));
            }
        }

        [Test]
        public void Zenkaku_BackSlash_To_Hankaku()
        {
            var list = new List<NarrowWidePair>(Const.ZEN_HAN_LIST);
            list.Add(new NarrowWidePair() {Hankaku = "\\", Zenkaku = "＼"});

            foreach (var item in list)
            {
                Assert.AreEqual(item.Hankaku, KanaConverter.ToNarrow(item.Zenkaku));
            }
        }

        [Test]
        public void Oomoji_To_Komoji()
        {
            foreach (var item in Const.OOMOJI_KOMOJI_LIST)
            {
                Assert.AreEqual(item.Small, KanaConverter.ToLowerCase(item.Big));
            }
        }

        [Test]
        public void Komoji_To_Oomoji()
        {
            foreach (var item in Const.OOMOJI_KOMOJI_LIST)
            {
                Assert.AreEqual(item.Big, KanaConverter.ToUpperCase(item.Small));
            }
        }
    }
}
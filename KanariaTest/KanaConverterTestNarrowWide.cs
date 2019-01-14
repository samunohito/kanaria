using Kanaria.KanaConverter;
using KanariaTest.ForTest;
using NUnit.Framework;

namespace KanariaTest
{
    public class KanaConverterTestNarrowWide
    {
        [Test]
        public void Wide_To_Narrow()
        {
            foreach (var item in Const.NARROW_WIDE_LIST)
            {
                Assert.AreEqual(item.Narrow, KanaConverter.ToNarrow(item.Wide));
            }
        }

        [Test]
        public void Narrow_To_Wide()
        {
            foreach (var item in Const.NARROW_WIDE_LIST)
            {
                Assert.AreEqual(item.Wide, KanaConverter.ToWide(item.Narrow));
            }
        }
        
        [Test]
        public void ExampleSentence1()
        {
            var hankaku = "ﾁﾀﾀﾌﾟ ﾄﾃﾄﾃFoooo!!!11!";
            var zenkaku = "チタタプ　トテトテＦｏｏｏｏ！！！１１！";
            Assert.AreEqual(hankaku, KanaConverter.ToNarrow(zenkaku));
            Assert.AreEqual(zenkaku, KanaConverter.ToWide(hankaku));
        }
        
        [Test]
        public void ExampleSentence2()
        {
            var hankaku = "吾輩ﾊ😺猫ﾃﾞｱﾙ😺";
            var zenkaku = "吾輩ハ😺猫デアル😺";
            Assert.AreEqual(hankaku, KanaConverter.ToNarrow(zenkaku));
            Assert.AreEqual(zenkaku, KanaConverter.ToWide(hankaku));
        }
    }
}
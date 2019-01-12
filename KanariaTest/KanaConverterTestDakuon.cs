using Kanaria.KanaConverter;
using NUnit.Framework;
using TestProject1.ForTest;

namespace KanariaTest
{
    public class KanaConverterTestDakuon
    {
        [Test]
        public void Hiragana_To_Katakana()
        {
            foreach (var item in Const.DAKUON_LIST)
            {
                Assert.AreEqual(item.KatakanaZen, KanaConverter.ToKatakana(item.Hiragana));
            }
        }

        [Test]
        public void Katakana_To_Hiragana()
        {
            foreach (var item in Const.DAKUON_LIST)
            {
                Assert.AreEqual(item.Hiragana, KanaConverter.ToHiragana(item.KatakanaZen));
            }
        }
 
        [Test]
        public void KatakanaZenkaku_To_KatakanaHankaku()
        {
            foreach (var item in Const.DAKUON_LIST)
            {
                Assert.AreEqual(item.KatakanaHan, KanaConverter.ToNarrow(item.KatakanaZen));
            }
        }

        [Test]
        public void KatakanaHankaku_To_KatakanaZenkaku()
        {
            foreach (var item in Const.DAKUON_LIST)
            {
                Assert.AreEqual(item.KatakanaZen, KanaConverter.ToWide(item.KatakanaHan));
            }
        }
    }
}
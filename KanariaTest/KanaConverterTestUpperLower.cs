using Kanaria.KanaConverter;
using KanariaTest.ForTest;
using NUnit.Framework;

namespace KanariaTest
{
    public class KanaConverterTestUpperLower
    {
        [Test]
        public void Upper_To_Lower()
        {
            foreach (var item in Const.UPPER_LOWER_LIST)
            {
                Assert.AreEqual(item.Lower, KanaConverter.ToLowerCase(item.Upper));
            }
        }

        [Test]
        public void Lower_To_Upper()
        {
            foreach (var item in Const.UPPER_LOWER_LIST)
            {
                Assert.AreEqual(item.Upper, KanaConverter.ToUpperCase(item.Lower));
            }
        }
    }
}
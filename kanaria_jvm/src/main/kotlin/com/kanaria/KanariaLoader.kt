package com.kanaria

import com.kanaria.exception.NotLoadingJniException
import com.kanaria.exception.UnsupportedPlatformException
import org.dom4j.Element
import org.dom4j.Node
import org.dom4j.io.SAXReader
import java.io.File

/**
 * 実行中のプラットフォームに適合したdll/soをJVMに読み込む機能を提供します.
 */
object KanariaLoader {
    /**
     * JNIの読み込みが終了している場合trueになります.
     */
    var isLoaded: Boolean = false
        private set

    /**
     * JNIを読み込んでいるかをチェックします。
     * 読み込んでいない場合、例外がスローされます。
     *
     * @throws NotLoadingJniException [load]メソッドにより初期化処理が呼ばれていないときにスローされます。
     */
    internal fun checkOrThrow() {
        if (!isLoaded) {
            throw NotLoadingJniException("Jni is not loaded. Check if the KanariaLoader#load() function has been executed.")
        }
    }

    /**
     * JNIで使用するライブラリをjarから取り出してカレントディレクトリに配置し、JVMに読み込みます.
     */
    fun load() = load(File("."))

    /**
     * JNIで使用するライブラリをjarから取り出して[expandDirectory]で指定されたディレクトリに配置し、JVMに読み込みます.
     * @param expandDirectory ライブラリファイルの展開先
     */
    fun load(expandDirectory: File) {
        if (isLoaded) {
            return
        }

        val platformName = findCompatiblePlatforms()
        val loadPath = expandInternalLibrary(platformName, expandDirectory)

        System.load(loadPath.absolutePath.toString())
        isLoaded = true
    }

    /**
     * リソースに収められたplatform.xmlからRustのプラットフォームを検索する.
     * platform.xmlとは、RustのプラットフォームとJavaの[System.getProperty]から取得できるos.nameとos.archの値等をマッピングしたファイルのこと.
     *
     * @see System.getProperty
     */
    private fun findCompatiblePlatforms(): String {
        val inputStream = KanariaLoader.javaClass.getResourceAsStream("platform.xml")
                ?: throw RuntimeException("platform.xml is not included.")
        val documents = SAXReader().read(inputStream)
        val arch = System.getProperty("os.arch")
        val os = getFamily()

        val selectOs = documents.selectNodes("/platformMapping/os")
                .asNodeSequence()
                .filter {
                    it.attributeValue("name-prefix")
                            ?.split("|")
                            ?.any { str -> str.startsWith(os) } ?: false
                }
                .firstOrNull() ?: throw UnsupportedPlatformException(os, arch)

        val selectArch = selectOs.selectNodes("./arch")
                .asNodeSequence()
                .filter {
                    it.attributeValue("type")
                            ?.split("|")
                            ?.any { str -> str == arch } ?: false
                }
                .firstOrNull() ?: throw UnsupportedPlatformException(os, arch)

        val selectPlatforms = selectArch.selectNodes("./platform")
                .asNodeSequence()
                .toList()

        val targetPlatform = selectPlatforms
                .find { platform ->
                    platform.selectNodes("./java-system-properties")
                            .asNodeSequence()
                            .filter { it.attribute("key") != null && it.attribute("value") != null }
                            .all { System.getProperty(it.attributeValue("key")) == it.attributeValue("value") }
                }
                ?: selectPlatforms.lastOrNull()
                ?: throw UnsupportedPlatformException(os, arch)

        return targetPlatform.attributeValue("name") ?: throw RuntimeException("Impossible")
    }

    /**
     * Rustのプラットフォームと同じ名前のディレクトリに収められたdll/soをexpandPathの箇所に抽出する.
     * それらのディレクトリには、ディレクトリ名と同じプラットフォームを使用してビルドされたdll/soが収められているはず.
     *
     * @param toolchainName [findCompatiblePlatforms]から取得したRustToolchainの名前
     * @param expandPath 展開先ディレクトリ名
     * @see findCompatiblePlatforms
     */
    private fun expandInternalLibrary(toolchainName: String, expandPath: File): File {
        val jniStreamWin = KanariaLoader.javaClass.getResourceAsStream("platforms/$toolchainName/kanaria_jni")
        val jniStreamPos = KanariaLoader.javaClass.getResourceAsStream("platforms/$toolchainName/libkanaria_jni")

        val (inputStream, filename) = if (jniStreamWin != null && jniStreamPos == null) {
            Pair(jniStreamWin, "kanaria_jni.dll")
        } else if (jniStreamWin == null && jniStreamPos != null) {
            Pair(jniStreamPos, "libkanaria_jni.so")
        } else {
            // 1つのディレクトリに両方は置かない
            throw RuntimeException("Library placement incorrect.")
        }

        val outputFilePath = File(expandPath.toString(), filename)
        inputStream.use {
            outputFilePath
                    .outputStream()
                    .use { outputStream ->
                        inputStream.copyTo(outputStream)
                    }
        }

        return outputFilePath
    }

    private fun getFamily(): String {
        val os = System.getProperty("os.name")
        return when {
            os == null -> ""
            os.toLowerCase().startsWith("windows") -> "windows"
            os.toLowerCase().startsWith("mac") -> "mac"
            os.toLowerCase().startsWith("linux") -> "unix"
            else -> ""
        }
    }

    private fun List<Node>.asNodeSequence() = this.asSequence()
            .filter { it is Element }
            .map { it as Element }
}
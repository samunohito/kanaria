import java.nio.file.Files
import java.nio.file.Paths

plugins {
    id("com.osm.gradle.plugins.rustic") version "0.2.7"
}

rustic {
    projectSettings.projectLocation("$rootDir/kanaria_extern")
    defaultConfig.defaultOptions.apply {
        jobs(8)
    }

    flavors {
        create("x86") {
            environments(mapOf(
                    "AR" to "/usr/bin/i686-w64-mingw32-gcc-ar",
                    "CC" to "/usr/bin/i686-w64-mingw32-gcc"
            ))
            defaultOptions.apply {
                target("i686-pc-windows-gnu")
                cargoConfig.targetTriple.rustFlags(listOf("-C", "panic=abort"))
            }
        }

        create("x86_64") {
            environments(mapOf(
                    "AR" to "/usr/bin/x86_64-w64-mingw32-gcc-ar",
                    "CC" to "/usr/bin/x86_64-w64-mingw32-gcc"
            ))
            defaultOptions.apply {
                target("x86_64-pc-windows-gnu")
            }
        }
    }

    variants.all {
        if (enabled == false) {
            // 無効化されているvariantは処理しない
            return@all
        }

        deployReleaseOutputs(tasks, this)
    }
}

fun deployReleaseOutputs(tasks: TaskContainer, variant: com.osm.gradle.plugins.types.variants.BuildVariant) {
    val target = variant.buildOptions.target
    val flavor = variant.flavor
    val buildTask = tasks["rustBuild${variant.name}"]
    if (buildTask == null || target == null || flavor == null) {
        println("[${variant.name}] missing parameters.")
        return
    }

    if (variant.buildOptions.release == false) {
        println("[${variant.name}] release build only.")
        return
    }

    // CCに指定したものをlinkerに流用
    flavor.defaultOptions.cargoConfig.targetTriple.linker(variant.environments?.get("CC"))

    // variants#all()の実行時点ではビルドが走っておらず成果物がないため、
    // ビルドの後処理としてコピー処理を設定する
    val bit = if (target.contains("x86_64")) "64" else "32"
    val srcFilePath = Paths.get("$rootDir/kanaria_extern/target/$target/release", "kanaria.dll")
    val dstFilePath = Paths.get("$projectDir/KanariaDotNet/costura${bit}/", "kanaria.dll")
    buildTask.doLast {
        if (!Files.exists(dstFilePath.parent)) {
            Files.createDirectories(dstFilePath.parent)
        }
        if (Files.exists(dstFilePath)) {
            Files.delete(dstFilePath)
        }
        println("[${variant.name}] Copy the build output from rustBuild to src/main/resources.")
        println("[${variant.name}]   copy from : $srcFilePath")
        println("[${variant.name}]   copy to   : $dstFilePath")
        Files.copy(srcFilePath, dstFilePath)
    }
}

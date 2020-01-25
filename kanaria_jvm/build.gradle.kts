import java.nio.file.Files
import java.nio.file.Paths

plugins {
    maven
    `kotlin-dsl`
    id("org.jetbrains.dokka") version "0.10.0"
    id("com.jfrog.bintray") version "1.8.4"
    id("com.osm.gradle.plugins.rustic") version "0.2.0"
}

val GITHUB_URL = "https://github.com/sam-osamu/kanaria"
group = "com.kanaria"
version = "0.1.0"

dependencies {
    "implementation"(group = "jaxen", name = "jaxen", version = "1.1.6")
    "implementation"(group = "org.dom4j", name = "dom4j", version = "2.1.1")
    "implementation"(group = "org.jetbrains.kotlin", name = "kotlin-stdlib-jdk8")
    "testImplementation"(group = "junit", name = "junit", version = "4.12")
    "testImplementation"(group = "org.jetbrains.kotlin", name = "kotlin-test-junit", version = "1.3.21")
}

tasks {
    val dokka by getting(org.jetbrains.dokka.gradle.DokkaTask::class) {
        outputFormat = "html"
        outputDirectory = "$buildDir/javadoc"
    }
}

configure<JavaPluginConvention> {
    sourceCompatibility = JavaVersion.VERSION_11
    targetCompatibility = JavaVersion.VERSION_11

    sourceSets {
        getByName("main").java.srcDirs("src/main/kotlin")
        getByName("test").java.srcDirs("src/test/kotlin")
    }

    tasks {
        getByName("javadoc", Javadoc::class) {
            options.locale = "ja_JP"
            isFailOnError = false
            source = sourceSets["main"].allJava
        }

        val sourcesJar by creating(Jar::class) {
            dependsOn(JavaPlugin.CLASSES_TASK_NAME)
            archiveClassifier.set("sources")
            from(sourceSets["main"].allSource)
        }

        val javadocJar by creating(Jar::class) {
            dependsOn(JavaPlugin.JAVADOC_TASK_NAME)
            archiveClassifier.set("javadoc")
            from(getByName("javadoc", Javadoc::class).destinationDir)
        }

        artifacts {
            add("archives", sourcesJar)
            add("archives", javadocJar)
        }
    }
}

bintray {
    user = if (project.hasProperty("bintray_user")) project.properties["bintray_user"].toString() else ""
    key = if (project.hasProperty("bintray_apikey")) project.properties["bintray_apikey"].toString() else ""
    pkg.apply {
        repo = "maven"
        name = "${project.group}.${project.name}"
        websiteUrl = GITHUB_URL
        issueTrackerUrl = "${GITHUB_URL}/issues"
        vcsUrl = "${GITHUB_URL}.git"
        publicDownloadNumbers = true
        setVersion(project.version)
    }
    setConfigurations("archives")
}

tasks.getByName("install", Upload::class) {
    repositories.withGroovyBuilder {
        "mavenInstaller" {
            "pom" {
                "project" {
                    setProperty("groupId", project.group)
                    setProperty("artifactId", project.name)
                    setProperty("version", project.version)

                    "licenses" {
                        "license" {
                            setProperty("name", "The MIT License")
                            setProperty("url", "https://opensource.org/licenses/MIT")
                            setProperty("distribution", "repo")
                        }
                    }

                    "scm" {
                        setProperty("connection", "${GITHUB_URL}.git")
                        setProperty("url", GITHUB_URL)
                    }
                }
            }
        }
    }
}

rustic {
    projectSettings.projectLocation("$projectDir/src/main/rust")
    defaultConfig.defaultOptions.apply {
        jobs(8)
    }

    flavors {
        create("i686-pc-windows-gnu") {
            environments(mapOf(
                    "AR" to "/usr/bin/i686-w64-mingw32-gcc-ar",
                    "CC" to "/usr/bin/i686-w64-mingw32-gcc"
            ))
            defaultOptions.apply {
                target("i686-pc-windows-gnu")
                cargoConfig.targetTriple.rustFlags(listOf("-C", "panic=abort"))
            }
        }

        create("x86_64-pc-windows-gnu") {
            environments(mapOf(
                    "AR" to "/usr/bin/x86_64-w64-mingw32-gcc-ar",
                    "CC" to "/usr/bin/x86_64-w64-mingw32-gcc"
            ))
            defaultOptions.apply {
                target("x86_64-pc-windows-gnu")
            }
        }

        create("i686-apple-darwin") {
            enabled(false)
            environments(mapOf(
                    "AR" to "",
                    "CC" to ""
            ))
            defaultOptions.apply {
                target("i686-apple-darwin")
            }
        }

        create("x86_64-apple-darwin") {
            enabled(false)
            environments(mapOf(
                    "AR" to "",
                    "CC" to ""
            ))
            defaultOptions.apply {
                target("x86_64-apple-darwin")
            }
        }

        create("i686-unknown-linux-gnu") {
            environments(mapOf(
                    "AR" to "/usr/bin/i686-linux-gnu-gcc-ar",
                    "CC" to "/usr/bin/i686-linux-gnu-gcc"
            ))
            defaultOptions.apply {
                target("i686-unknown-linux-gnu")
            }
        }

        create("x86_64-unknown-linux-gnu") {
            environments(mapOf(
                    "AR" to "/usr/bin/x86_64-linux-gnu-gcc-ar",
                    "CC" to "/usr/bin/x86_64-linux-gnu-gcc"
            ))
            defaultOptions.apply {
                target("x86_64-unknown-linux-gnu")
            }
        }

        create("arm-unknown-linux-gnueabihf") {
            environments(mapOf(
                    "AR" to "/usr/bin/arm-linux-gnueabihf-gcc-ar",
                    "CC" to "/usr/bin/arm-linux-gnueabihf-gcc"
            ))
            defaultOptions.apply {
                target("arm-unknown-linux-gnueabihf")
            }
        }

        create("aarch64-unknown-linux-gnu") {
            environments(mapOf(
                    "AR" to "/usr/bin/aarch64-linux-gnu-gcc-ar",
                    "CC" to "/usr/bin/aarch64-linux-gnu-gcc"
            ))
            defaultOptions.apply {
                target("aarch64-unknown-linux-gnu")
            }
        }

        create("i686-linux-android") {
            environments(mapOf(
                    "AR" to "/usr/local/lib/android-ndk-r20b/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android-ar",
                    "CC" to "/usr/local/lib/android-ndk-r20b/toolchains/llvm/prebuilt/linux-x86_64/bin/i686-linux-android26-clang"
            ))
            defaultOptions.apply {
                target("i686-linux-android")
            }
        }

        create("x86_64-linux-android") {
            environments(mapOf(
                    "AR" to "/usr/local/lib/android-ndk-r20b/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android-ar",
                    "CC" to "/usr/local/lib/android-ndk-r20b/toolchains/llvm/prebuilt/linux-x86_64/bin/x86_64-linux-android26-clang"
            ))
            defaultOptions.apply {
                target("x86_64-linux-android")
            }
        }

        create("arm-linux-androideabi") {
            environments(mapOf(
                    "AR" to "/usr/local/lib/android-ndk-r20b/toolchains/llvm/prebuilt/linux-x86_64/bin/arm-linux-androideabi-ar",
                    "CC" to "/usr/local/lib/android-ndk-r20b/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi26-clang"
            ))
            defaultOptions.apply {
                target("arm-linux-androideabi")
            }
        }

        create("aarch64-linux-android") {
            environments(mapOf(
                    "AR" to "/usr/local/lib/android-ndk-r20b/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android-ar",
                    "CC" to "/usr/local/lib/android-ndk-r20b/toolchains/llvm/prebuilt/linux-x86_64/bin/aarch64-linux-android26-clang"
            ))
            defaultOptions.apply {
                target("aarch64-linux-android")
            }
        }
    }

    variants.all {
        if (enabled == false) {
            // 無効化されているvariantは処理しない
            return@all
        }

        deployReleaseBuildOutput(tasks, this)
    }
}

fun deployReleaseBuildOutput(tasks: TaskContainer, variant: com.osm.gradle.plugins.types.variants.BuildVariant) {
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

    // リリースビルドに限り、ビルド成果物をKotlin側ソースのリソースディレクトリに配置する
    // また、Android対策として拡張子は省いておく
    val srcFilePath = Paths.get("$projectDir/src/main/rust/target/$target/release", getOutputFileName(target))
    val dstFileName = srcFilePath.fileName.toString()
    val dstFilePath = Paths.get(
            "$projectDir/src/main/resources/com/kanaria/platforms/${target}/",
            dstFileName.substring(0, dstFileName.lastIndexOf('.'))
    )

    // variants#all()の実行時点ではビルドが走っておらず成果物がないため、
    // ビルドの後処理としてコピー処理を設定する
    buildTask.doLast {
        if (!Files.exists(dstFilePath.parent)) {
            Files.createDirectories(dstFilePath.parent)
        }

        if (Files.exists(dstFilePath)) {
            Files.delete(dstFilePath)
        }

        println("Copy the build output from rustBuild to src/main/resources.")
        println("  copy from : $srcFilePath")
        println("  copy to   : $dstFilePath")
        Files.copy(srcFilePath, dstFilePath)
    }
}

fun getOutputFileName(target: String?): String {
    return if (target != null && target.contains("windows")) "kanaria_jni.dll" else "libkanaria_jni.so"
}
allprojects {
    repositories {
        google()
        jcenter()
        mavenCentral()
    }
}

tasks {
    create("release") {
        group = "build"

        val root = this
        childProjects.values.forEach { childProject ->
            childProject.gradle.projectsEvaluated {
                childProject.tasks.findByName("rustBuildRelease")?.also {
                    root.dependsOn(it)
                }
            }
        }
    }
}
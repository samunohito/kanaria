allprojects {
    repositories {
        google()
        jcenter()
        mavenCentral()
    }
}

tasks {
    create("release") {
        group = "summary"
        bindChildProjectsTasks(this, "rustBuildRelease", childProjects.values)
    }

    create("test") {
        group = "summary"

        val exclude = listOf("kanaria_dotnet", "kanaria_extern", "kanaria_jvm")
        bindChildProjectsTasks(this, "rustTestRelease", childProjects.values.filter { pj -> !exclude.any { pj.name.contains(it) } })
        bindChildProjectsTasks(this, "testClasses", childProjects.values)
    }
}

fun bindChildProjectsTasks(rootTask: Task, findByName: String, childProjects: Iterable<Project>) {
    childProjects.forEach { childProject ->
        childProject.gradle.projectsEvaluated {
            childProject.tasks.findByName(findByName)?.also {
                rootTask.dependsOn(it)
            }
        }
    }
}
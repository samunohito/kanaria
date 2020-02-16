plugins {
    id("com.osm.gradle.plugins.rustic") version "0.2.7"
}

rustic {
    projectSettings.projectLocation(projectDir.toString())
    defaultConfig.defaultOptions.jobs(8)
}
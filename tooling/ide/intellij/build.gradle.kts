plugins {
    id("java")
    id("org.jetbrains.kotlin.jvm") version "1.9.22"
    id("org.jetbrains.intellij") version "1.17.2"
}

group = "dev.mantra"
version = "0.5.0"

repositories {
    mavenCentral()
}

intellij {
    version.set("2023.3.4")
    type.set("IC") // IntelliJ Community
    plugins.set(listOf())
}

tasks {
    patchPluginXml {
        sinceBuild.set("233")
        untilBuild.set("243.*")
    }
}


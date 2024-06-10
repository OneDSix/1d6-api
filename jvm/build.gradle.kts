plugins {
	kotlin("jvm") version "1.8.0"
	application
}

repositories {
	mavenCentral()
	google()
}

dependencies {
	implementation("com.google.code.gson:gson:2.8.9")
	implementation("com.squareup.retrofit2:retrofit:2.9.0")
	implementation("com.squareup.retrofit2:converter-gson:2.9.0")
	implementation("de.mkammerer:argon2-jvm:2.7")
	implementation("com.squareup.okhttp3:okhttp:4.9.3")
}

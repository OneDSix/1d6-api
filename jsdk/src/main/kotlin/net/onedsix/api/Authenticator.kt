package net.onedsix.api

import com.google.gson.annotations.SerializedName
import de.mkammerer.argon2.Argon2
import de.mkammerer.argon2.Argon2Factory
import retrofit2.Call
import retrofit2.Retrofit
import retrofit2.http.Body
import retrofit2.http.POST

class Authenticator(retrofit: Retrofit) {
	private val argon2: Argon2 = Argon2Factory.create()
	private val endpoint: UserApi = retrofit.create(UserApi::class.java)

	fun hashPassword(password: String): String {
		return argon2.hash(2, 65536, 1, password.toCharArray())
	}

	/** Given a plain text username+password combo, this will attempt a sign in. */
	fun login(username: String, password: String): LoginResponse? {
		val hashedPassword = hashPassword(password)
		val request = CredentialsRequest(username, hashedPassword)

		val call = endpoint.login(request)
		val response = call.execute()
		return if (response.isSuccessful) {
			response.body()
		} else {
			null
		}
	}

	data class CredentialsRequest(
		@SerializedName("username") val username: String,
		@SerializedName("password") val password: String
	)

	data class LoginResponse(
		@SerializedName("message") val message: String
	)

	interface UserApi {
		@POST("/v1/user/login")
		fun login(@Body request: CredentialsRequest): Call<LoginResponse>

		@POST("/v1/user/signup")
		fun signup(@Body request: CredentialsRequest): Call<LoginResponse>
	}
}

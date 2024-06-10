import com.google.gson.annotations.SerializedName
import de.mkammerer.argon2.Argon2
import de.mkammerer.argon2.Argon2Factory
import retrofit2.Call
import retrofit2.Retrofit
import retrofit2.http.Body
import retrofit2.http.POST

/** Data class for login request */
data class LoginRequest(
	@SerializedName("username") val username: String,
	@SerializedName("password") val password: String
)

/** Data class for login response */
data class LoginResponse(
	@SerializedName("message") val message: String // Assuming the response has a message
)

/** Retrofit API interface */
interface LoginApi {
	@POST("/api/login")
	fun login(@Body request: LoginRequest): Call<LoginResponse>
}

class Authenticator(retrofit: Retrofit) {
	private val argon2: Argon2 = Argon2Factory.create()
	private val endpoint: LoginApi = retrofit.create(LoginApi::class.java)

	fun hashPassword(password: String): String {
		return argon2.hash(2, 65536, 1, password.toCharArray())
	}

	/** Given a plain text username+password combo, this will attempt a sign in. */
	fun login(username: String, password: String): LoginResponse? {
		val hashedPassword = hashPassword(password)
		val request = LoginRequest(username, hashedPassword)

		val call = endpoint.login(request)
		val response = call.execute()
		return if (response.isSuccessful) {
			response.body()
		} else {
			null
		}
	}
}

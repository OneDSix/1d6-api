import com.google.gson.Gson
import okhttp3.Cookie
import okhttp3.CookieJar
import okhttp3.HttpUrl
import okhttp3.OkHttpClient
import retrofit2.Retrofit
import retrofit2.converter.gson.GsonConverterFactory

const val API_URL: String = "https://onedsixapi.shuttleapp.rs/"

/**
 * Creates a new instance of the API wrapper.
 * @param gsonInstance A null-safe option to give a custom `com.google.gson.Gson` instance for better formatting or to save on memory. Otherwise it uses the default `Gson()`.
 * */
class OneDSixApiWrapper(gsonInstance: Gson?) {
	private val gson = gsonInstance ?: Gson()
	private val cookieStore = CookieStore()
	private val client = OkHttpClient.Builder()
		.cookieJar(cookieStore)
		.build()
	private val retrofit = Retrofit.Builder()
		.baseUrl(API_URL)
		.addConverterFactory(GsonConverterFactory.create(gson))
		.client(client)
		.build()

	init {
	    val AUTH: Authenticator = Authenticator(retrofit)
	}

	fun getCookies(): List<Cookie> {
		return cookieStore.getCookies()
	}
}

class CookieStore : CookieJar {
	private val cookieMap = mutableMapOf<String, List<Cookie>>()

	override fun saveFromResponse(url: HttpUrl, cookies: List<Cookie>) {
		cookieMap[url.host] = cookies
	}

	override fun loadForRequest(url: HttpUrl): List<Cookie> {
		return cookieMap[url.host] ?: emptyList()
	}

	fun getCookies(): List<Cookie> {
		return cookieMap.values.flatten()
	}
}

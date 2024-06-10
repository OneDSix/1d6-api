fun testAuthenication() {
	val userLogin = UserLogin("https://your-api-base-url.com")
	val username = "yourUsername"
	val password = "yourPassword"

	val response = userLogin.login(username, password)
	if (response != null) {
		println("Login successful! Token: ${response.token}")
	} else {
		println("Login failed.")
	}
}

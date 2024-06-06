# This file exists only because cURL doesnt like to play nice with powershell and i'm too lazy to set up an ubuntu VM right now.
# This will be updated over time as it is needed.

import requests

LOGIN_URL: str = "http://127.0.0.1:8000/v1/user/login"
DASHBOARD_URL: str = "http://127.0.0.1:8000/v1/user"

login_payload = {
    "username": "myuser",
    "password": "mypass"
}

with requests.Session() as session:

    login_response = session.post(LOGIN_URL, json=login_payload)

    if login_response.status_code == 200:
        print("Logged in successfully")
        print("Login Cookie:", login_response.cookies)

        dashboard_response = session.get(DASHBOARD_URL)

        if dashboard_response.status_code == 200:
            print("Dashboard Data:", dashboard_response.text)
        else:
            print("Failed to fetch user data:", dashboard_response.json())
    else:
        print("Failed to login:", login_response.json())

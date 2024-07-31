import axios, { AxiosRequestConfig, AxiosResponse } from 'axios';
import { CookieJar } from 'tough-cookie';
import { wrapper } from 'axios-cookiejar-support';
import Authenticator from './auth';

/** Interface for request options */
export interface RequestOptions extends AxiosRequestConfig { }

/** The 1D6 API SDK */
export default class ApiSdk {
	private homeServer: string
	private jar: CookieJar
	public authenticated: boolean = false

	public AUTH: Authenticator

	constructor(homeURL: string) {
		this.homeServer = homeURL
		this.jar = new CookieJar()
		wrapper(axios)

		this.AUTH = new Authenticator()
	}

	/// GET request method
	public async get(endpoint: string, options?: RequestOptions): Promise<AxiosResponse> {
		const url = `${this.homeServer}${endpoint}`
		return axios.get(url, { ...options, jar: this.jar, withCredentials: true })
	}

	/// POST request method
	public async post(endpoint: string, data: any, options?: RequestOptions): Promise<AxiosResponse> {
		const url = `${this.homeServer}${endpoint}`
		return axios.post(url, data, { ...options, jar: this.jar, withCredentials: true })
	}
}

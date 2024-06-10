import { fetchFunction } from "./sided";

class ApiSdk {

	username: string | undefined;
	password: string | undefined;

	constructor(username: string, password: string) {
		console.log("Loaded 1D6 API SDK");
	}

	public async fetchData(url: string): Promise<any> {
		const response = await fetchFunction(url);
		if (!response.ok) {
			throw new Error(`HTTP error! status: ${response.status}`);
		}
		return response.json();
	}
}

export default ApiSdk;

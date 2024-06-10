// This file contains functions and classes

let fetchFunction: (input: RequestInfo, init?: RequestInit) => Promise<Response>;

if (typeof window === 'undefined') {
	// Node.js environment
	// Any NPM package can be used here.

	const nodeFetch = require('node-fetch');
	fetchFunction = nodeFetch;

} else {
	// Browser environment
	// Only a small portion of NPM packages can be used here.

	fetchFunction = fetch;

}

export {
	fetchFunction
}

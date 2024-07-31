import { hash } from 'argon2-browser'

export default async function hashPassword(password: string): Promise<string> {
	const result = await hash({ pass: password, salt: new Uint8Array(16) })
	return result.encoded
}


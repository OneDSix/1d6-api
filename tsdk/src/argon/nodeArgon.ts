import argon2 from 'argon2'

export default async function hashPassword(password: string): Promise<string> {
	return await argon2.hash(password)
}

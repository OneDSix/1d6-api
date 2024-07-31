let hashPassword: (password: string) => Promise<string>;

if (typeof window !== 'undefined') {
  // Browser environment
  import('./argon/browserArgon').then(module => {
    hashPassword = module.default
  })
} else {
  // Node.js environment
  import('./argon/nodeArgon').then(module => {
    hashPassword = module.default
  })
}

export default class Authenticator {

}

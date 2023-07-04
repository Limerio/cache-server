export type AuthState = {
	type: 'login'
	payload: AuthData
}

export type AuthData = {
	url: string
}

export type AuthContextValue = {
	server: AuthData | null
	login: (payload: AuthData, callback: VoidFunction) => void
}

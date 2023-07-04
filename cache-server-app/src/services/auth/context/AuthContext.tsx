import {
	PropsWithChildren,
	createContext,
	useCallback,
	useReducer,
} from 'react'
import { AuthContextValue, AuthData, AuthState } from '../utils/types'

export const AuthContext = createContext<AuthContextValue>(
	{} as AuthContextValue
)

function authReducer(state: AuthData, action: AuthState) {
	switch (action.type) {
		case 'login':
			return {
				...state,
				...action.payload,
			}
	}
}

export function AuthProvider({ children }: PropsWithChildren) {
	const [state, dispatch] = useReducer(authReducer, {} as AuthData)

	const login = useCallback((payload: AuthData, callback: VoidFunction) => {
		dispatch({ type: 'login', payload })
		callback()
	}, [])

	return (
		<AuthContext.Provider value={{ login, server: state }}>
			{children}
		</AuthContext.Provider>
	)
}

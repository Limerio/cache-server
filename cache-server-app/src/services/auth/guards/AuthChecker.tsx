import { PropsWithChildren } from 'react'
import { Navigate, useLocation } from 'react-router-dom'
import { useAuth } from '../hooks'

export function AuthChecker({ children }: PropsWithChildren) {
	const { server } = useAuth()
	const location = useLocation()

	if (!server) {
		return <Navigate to="/login" state={{ from: location }} replace />
	}

	return children
}

import { useAuth } from '@/services/auth/hooks'
import { useForm } from '@mantine/form'

export function LoginPage() {
	const { login } = useAuth()

	const form = useForm({})
}

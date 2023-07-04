import { AppShell } from '@mantine/core'
import { Outlet } from 'react-router-dom'
import { Header, Navbar } from '../components/layouts'
import { AuthChecker } from '../services/auth/guards'

export function MainLayout() {
	return (
		<AuthChecker>
			<AppShell padding="md" navbar={<Navbar />} header={<Header />}>
				<Outlet />
			</AppShell>
		</AuthChecker>
	)
}

import { MainLayout } from '@/layouts'
import { LoginPage } from '@/pages/auth'
import { ServerPage } from '@/pages/server'
import { Route, Routes } from 'react-router-dom'

function App() {
	return (
		<Routes>
			<Route path="/login" element={<LoginPage />} />
			<Route path="/server" element={<MainLayout />}>
				<Route index element={<ServerPage />} />
			</Route>
		</Routes>
	)
}

export default App

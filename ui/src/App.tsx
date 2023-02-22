import { isMobile } from 'react-device-detect'
import { Route, Routes } from 'react-router-dom'
import { ProtectedRoute } from './components/ProtectedRoute'
import { ChatForPc } from './pages/chat-for-pc'
import { ChatForMobile } from './pages/chat-for-mobile'
import { Login } from './pages/login'
import { Signup } from './pages/signup'

function App() {
  return (
    <Routes>
      <Route
        path="/"
        element={
          <ProtectedRoute>
            {isMobile ? <ChatForMobile /> : <ChatForPc />}
          </ProtectedRoute>
        }
      />
      <Route path="/signup" element={<Signup />} />
      <Route path="/login" element={<Login />} />
    </Routes>
  )
}

export default App

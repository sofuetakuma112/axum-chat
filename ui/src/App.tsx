import { Route, Routes } from "react-router-dom"
import { ProtectedRoute } from "./components/ProtectedRoute"
import { Chat } from "./pages/chat"
import { Login } from "./pages/login"
import { Signup } from "./pages/signup"

function App() {

  return <Routes>
    <Route
      path="/"
      element={
        <ProtectedRoute>
          <Chat />
        </ProtectedRoute>
      }
    />
    <Route path="/signup" element={<Signup />} />
    <Route path="/login" element={<Login />} />
  </Routes>
}

export default App

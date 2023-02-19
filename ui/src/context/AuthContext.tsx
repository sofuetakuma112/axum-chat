import React, { createContext, useContext, useEffect } from 'react'
import useAuth, { LoginPayload, User } from '../hooks/useAuth'

type AuthContextType = { user: User | null | undefined, login: (data: LoginPayload) => Promise<void>, signup: (data: LoginPayload) => Promise<void>, logout: () => void }

const AuthContext = createContext<AuthContextType>({
  user: undefined,
  login: function (data: LoginPayload): Promise<void> {
    throw new Error('Function not implemented.')
  },
  signup: function (data: LoginPayload): Promise<void> {
    throw new Error('Function not implemented.')
  },
  logout: function (): Promise<void> {
    throw new Error('Function not implemented.')
  }
})

export function useAuthContext() {
  return useContext(AuthContext)
}

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const { user, login, signup, getUser, logout } = useAuth()

  useEffect(() => {
    getUser()
  }, [getUser])

  const value = {
    user,
    login,
    signup,
    logout
  }

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>
}

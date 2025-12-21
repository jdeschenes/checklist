import {
    createContext,
    useContext,
    useEffect,
    useState,
    ReactNode,
} from 'react'
import { setAuthToken, setAuthErrorHandler } from '@/api/authenticated-client'

interface User {
    user_id: string
    email: string
}

interface AuthContextType {
    user: User | null
    token: string | null
    isAuthenticated: boolean
    isLoading: boolean
    login: (token: string, user: User) => void
    logout: () => void
    redirectToLogin: () => void
}

const AuthContext = createContext<AuthContextType | undefined>(undefined)

export const useAuth = () => {
    const context = useContext(AuthContext)
    if (context === undefined) {
        throw new Error('useAuth must be used within an AuthProvider')
    }
    return context
}

interface AuthProviderProps {
    children: ReactNode
}

const TOKEN_STORAGE_KEY = 'checklist_auth_token'
const USER_STORAGE_KEY = 'checklist_auth_user'

export const AuthProvider = ({ children }: AuthProviderProps) => {
    const [user, setUser] = useState<User | null>(null)
    const [token, setToken] = useState<string | null>(null)
    const [isLoading, setIsLoading] = useState(true)

    useEffect(() => {
        // Set up auth error handler
        setAuthErrorHandler(() => {
            logout()
            redirectToLogin()
        })

        // Add a small delay to prevent flash of loading screen
        const initAuth = async () => {
            // Load auth state from localStorage on mount
            const storedToken = localStorage.getItem(TOKEN_STORAGE_KEY)
            const storedUser = localStorage.getItem(USER_STORAGE_KEY)

            if (storedToken && storedUser) {
                try {
                    const parsedUser = JSON.parse(storedUser) as User
                    setToken(storedToken)
                    setUser(parsedUser)
                    setAuthToken(storedToken)
                } catch {
                    // Clear invalid stored data
                    localStorage.removeItem(TOKEN_STORAGE_KEY)
                    localStorage.removeItem(USER_STORAGE_KEY)
                    setAuthToken(null)
                }
            }

            // Small delay to prevent flashing
            await new Promise((resolve) => setTimeout(resolve, 100))
            setIsLoading(false)
        }

        void initAuth()
    }, [])

    const login = (newToken: string, newUser: User) => {
        setToken(newToken)
        setUser(newUser)
        setAuthToken(newToken)
        localStorage.setItem(TOKEN_STORAGE_KEY, newToken)
        localStorage.setItem(USER_STORAGE_KEY, JSON.stringify(newUser))
    }

    const logout = () => {
        setToken(null)
        setUser(null)
        setAuthToken(null)
        localStorage.removeItem(TOKEN_STORAGE_KEY)
        localStorage.removeItem(USER_STORAGE_KEY)
    }

    const redirectToLogin = () => {
        // Store current location for redirect after login
        const pathOnly = window.location.pathname
        const currentPath = pathOnly + window.location.search
        if (
            pathOnly !== '/login' &&
            !pathOnly.startsWith('/login/') &&
            !pathOnly.startsWith('/auth/callback')
        ) {
            localStorage.setItem('checklist_redirect_after_login', currentPath)
        }

        // Redirect to frontend login page with explicit origin
        window.location.href = `${window.location.origin}/login`
    }

    const value: AuthContextType = {
        user,
        token,
        isAuthenticated: !!token && !!user,
        isLoading,
        login,
        logout,
        redirectToLogin,
    }

    return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>
}

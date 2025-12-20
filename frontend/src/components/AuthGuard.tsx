import { useAuth } from '@/contexts/AuthContext'
import { useNavigate } from '@tanstack/react-router'
import { useEffect } from 'react'

interface AuthGuardProps {
  children: React.ReactNode
}

export function AuthGuard({ children }: AuthGuardProps) {
  const { isAuthenticated, isLoading } = useAuth()
  const navigate = useNavigate()

  useEffect(() => {
    if (!isLoading && !isAuthenticated) {
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
      
      // Navigate to login page using React Router
      void navigate({ to: '/login' })
    }
  }, [isAuthenticated, isLoading, navigate])

  // Show loading screen while determining auth state
  if (isLoading) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-gray-50">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto"></div>
          <h2 className="mt-4 text-lg font-medium text-gray-900">Loading...</h2>
          <p className="mt-2 text-sm text-gray-600">Please wait while we check your authentication status</p>
        </div>
      </div>
    )
  }

  // Show loading screen while navigating to login (prevents flash of error)
  if (!isAuthenticated) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-gray-50">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600 mx-auto"></div>
          <h2 className="mt-4 text-lg font-medium text-gray-900">Redirecting...</h2>
          <p className="mt-2 text-sm text-gray-600">Taking you to the sign in page</p>
        </div>
      </div>
    )
  }

  return <>{children}</>
}

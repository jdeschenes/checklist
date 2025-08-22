import { createFileRoute } from '@tanstack/react-router'
import { useAuth } from '@/contexts/AuthContext'
import { useEffect, useState } from 'react'

export const Route = createFileRoute('/auth/callback')({
  component: AuthCallback,
})

function AuthCallback() {
  const { login } = useAuth()
  const [error, setError] = useState<string | null>(null)
  const [isLoading, setIsLoading] = useState(true)

  useEffect(() => {
    const handleCallback = async () => {
      try {
        const urlParams = new URLSearchParams(window.location.search)
        const token = urlParams.get('token')
        const user_id = urlParams.get('user_id')
        const email = urlParams.get('email')
        const error = urlParams.get('error')

        if (error) {
          setError(`Authentication failed: ${error}`)
          setIsLoading(false)
          return
        }

        if (!token || !user_id || !email) {
          setError('Missing authentication data')
          setIsLoading(false)
          return
        }

        // Process the token and user data received from backend redirect
        const user = { user_id, email }
        login(token, user)
        
        // Redirect to intended destination or home
        const redirectPath = localStorage.getItem('checklist_redirect_after_login') || '/'
        localStorage.removeItem('checklist_redirect_after_login')
        window.location.href = redirectPath
      } catch (err) {
        console.error('Authentication error:', err)
        setError(err instanceof Error ? err.message : 'Authentication failed')
        setIsLoading(false)
      }
    }

    void handleCallback()
  }, [login])

  if (isLoading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900 mx-auto"></div>
          <p className="mt-4 text-gray-600">Completing authentication...</p>
        </div>
      </div>
    )
  }

  if (error) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="text-center">
          <h2 className="text-2xl font-bold text-red-600 mb-4">Authentication Error</h2>
          <p className="text-gray-600 mb-6">{error}</p>
          <a
            href="/login"
            className="text-blue-600 hover:text-blue-800 underline"
          >
            Try again
          </a>
        </div>
      </div>
    )
  }

  return null
}
import { QueryClient } from '@tanstack/react-query'
import { Outlet, createRootRouteWithContext, useLocation, useNavigate } from '@tanstack/react-router'
import { TanStackRouterDevtools } from '@tanstack/router-devtools'
import { AppBreadcrumb } from '@/components/AppBreadcrumb'
import { useAuth } from '@/contexts/AuthContext'
import { Button } from '@/components/ui/button'
import { AuthGuard } from '@/components/AuthGuard'

type RouterContext = {
    queryClient: QueryClient
}

function RootComponent() {
    const { isAuthenticated, logout } = useAuth()
    const location = useLocation()
    const navigate = useNavigate()

    const handleLogout = () => {
        logout()
        navigate({ to: '/login' })
    }

    // Routes that don't require authentication
    const publicRoutes = ['/login', '/auth/callback']
    const isPublicRoute = publicRoutes.includes(location.pathname)

    const content = (
        <>
            {isAuthenticated && (
                <header className="bg-white border-b border-gray-200 px-4 py-2 flex items-center justify-between">
                    <div className="flex items-center">
                        <AppBreadcrumb />
                    </div>
                    <Button
                        onClick={handleLogout}
                        variant="ghost"
                        size="sm"
                        className="p-2"
                        title="Logout"
                    >
                        <svg 
                            className="w-4 h-4" 
                            fill="none" 
                            stroke="currentColor" 
                            viewBox="0 0 24 24"
                        >
                            <path 
                                strokeLinecap="round" 
                                strokeLinejoin="round" 
                                strokeWidth={2} 
                                d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" 
                            />
                        </svg>
                    </Button>
                </header>
            )}
            <Outlet />
            <TanStackRouterDevtools />
        </>
    )

    // If it's a public route, render without auth guard
    if (isPublicRoute) {
        return content
    }

    // Otherwise, wrap with auth guard
    return <AuthGuard>{content}</AuthGuard>
}

export const Route = createRootRouteWithContext<RouterContext>()({
    component: RootComponent,
})

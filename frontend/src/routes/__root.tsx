import { QueryClient } from '@tanstack/react-query'
import {
    Outlet,
    createRootRouteWithContext,
} from '@tanstack/react-router'
import { TanStackRouterDevtools } from '@tanstack/router-devtools'
import { AppBreadcrumb } from '@/components/AppBreadcrumb'

type RouterContext = {
    queryClient: QueryClient
}

export const Route = createRootRouteWithContext<RouterContext>()({
    component: () => (
        <>
            <div className="p-2">
                <AppBreadcrumb />
            </div>
            <hr />
            <Outlet />
            <TanStackRouterDevtools />
        </>
    ),
})

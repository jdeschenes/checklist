import React from 'react'
import ReactDOM from 'react-dom/client'
import { RouterProvider, createRouter } from '@tanstack/react-router'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'

import { routeTree } from './routeTree.gen'
import { TanStackRouterDevtools } from '@tanstack/router-devtools'

const queryClient = new QueryClient()

const router = createRouter({ routeTree, context:{queryClient} })

declare module '@tanstack/react-router' {
    interface Register {
        router: typeof router
    }
}

ReactDOM.createRoot(document.getElementById('root')!).render(
    <React.StrictMode>
        <QueryClientProvider client={queryClient}>
            <RouterProvider router={router} context={{ queryClient }} />
        </QueryClientProvider>
        <TanStackRouterDevtools router={router} />
    </React.StrictMode>
)

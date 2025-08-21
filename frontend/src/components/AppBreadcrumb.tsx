import { Link, useLocation } from '@tanstack/react-router'
import { useSuspenseQuery } from '@tanstack/react-query'
import { getTodoQueryOptions } from '@/api/todoQueryOptions'
import {
    Breadcrumb,
    BreadcrumbList,
    BreadcrumbItem,
    BreadcrumbLink,
    BreadcrumbPage,
    BreadcrumbSeparator,
} from '@/components/ui/breadcrumb'

export function AppBreadcrumb() {
    const location = useLocation()
    const pathname = location.pathname

    // Parse route segments
    const segments = pathname.split('/').filter(Boolean)
    
    // For todo routes, get the todo name if we have a todoId
    const todoId = segments[1]
    const todoQuery = todoId && segments[0] === 'todo' 
        ? useSuspenseQuery(getTodoQueryOptions(todoId))
        : null

    const breadcrumbItems: Array<{
        label: string
        href: string
        isPage: boolean
    }> = []

    // Always start with Home
    if (pathname !== '/') {
        breadcrumbItems.push({
            label: 'Home',
            href: '/',
            isPage: false,
        })
    }

    // Handle different route patterns
    if (segments[0] === 'todo') {
        if (segments.length === 2 && segments[1] === 'new') {
            // /todo/new
            breadcrumbItems.push({
                label: 'New Todo',
                href: '/todo/new',
                isPage: true,
            })
        } else if (segments.length >= 2 && segments[1] !== 'new') {
            // /todo/$todoId or /todo/$todoId/new
            const todoName = todoQuery?.data?.name || todoId
            
            if (segments.length === 2) {
                // /todo/$todoId
                breadcrumbItems.push({
                    label: todoName,
                    href: `/todo/${todoId}`,
                    isPage: true,
                })
            } else if (segments.length === 3 && segments[2] === 'new') {
                // /todo/$todoId/new
                breadcrumbItems.push({
                    label: todoName,
                    href: `/todo/${todoId}`,
                    isPage: false,
                })
                breadcrumbItems.push({
                    label: 'New Item',
                    href: `/todo/${todoId}/new`,
                    isPage: true,
                })
            }
        }
    }

    // Don't render breadcrumb if we're on home page with no additional context
    if (breadcrumbItems.length === 0) {
        return null
    }

    return (
        <Breadcrumb>
            <BreadcrumbList>
                {breadcrumbItems.map((item, index) => (
                    <div key={item.href} className="flex items-center gap-1.5">
                        {index > 0 && <BreadcrumbSeparator />}
                        <BreadcrumbItem>
                            {item.isPage ? (
                                <BreadcrumbPage>{item.label}</BreadcrumbPage>
                            ) : (
                                <BreadcrumbLink asChild>
                                    <Link to={item.href}>{item.label}</Link>
                                </BreadcrumbLink>
                            )}
                        </BreadcrumbItem>
                    </div>
                ))}
            </BreadcrumbList>
        </Breadcrumb>
    )
}
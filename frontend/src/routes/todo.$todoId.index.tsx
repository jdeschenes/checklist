import { createFileRoute, Link } from '@tanstack/react-router'
import { useSuspenseQuery } from '@tanstack/react-query'
import { Plus } from 'lucide-react'
import { buttonVariants } from '@/components/ui/button'
import { getTodoQueryOptions } from '@/api/todoQueryOptions'

export const Route = createFileRoute('/todo/$todoId/')({
    component: RouteComponent,
})

function RouteComponent() {
    const todoId = Route.useParams().todoId
    const getTodoQuery = useSuspenseQuery(getTodoQueryOptions(todoId))
    return (
        <div className="bg-white rounded-lg shadow-sm p-6 mb-4">
            <div className="flex items-center justify-between">
                <div className="space-y-1">
                    <h1 className="text-2xl font-semibold text-gray-900">{getTodoQuery.data.name}</h1>
                    <p className="text-sm text-gray-500">Manage your tasks and stay organized</p>
                </div>
                <Link
                    className={buttonVariants({ 
                        variant: 'default',
                        className: 'gap-2 shadow-sm hover:shadow-md transition-shadow'
                    })}
                    to="/todo/$todoId/new"
                    params={{ todoId }}
                >
                    <Plus className="h-4 w-4" /> New Task
                </Link>
            </div>
        </div>
    )
}

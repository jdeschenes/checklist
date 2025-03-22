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
        <div className="flex flex-auto">
            <h1 className="text-xl font-bold grow">{getTodoQuery.data.name}</h1>
            <Link
                className={buttonVariants({ variant: 'default' })}
                to="/todo/$todoId/new"
                params={{ todoId }}
            >
                <Plus /> New
            </Link>
        </div>
    )
}

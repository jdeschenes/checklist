import { createFileRoute, Link, Outlet } from '@tanstack/react-router'
import { getTodoQueryOptions } from '@/api/todoQueryOptions'
import { listTodoItemsQueryOptions } from '@/api/todoItemQueryOptions'
import { useSuspenseQuery } from '@tanstack/react-query'
export const Route = createFileRoute('/todo/$todoId')({
    component: RouteComponent,
    loader: async ({ context: { queryClient }, params: { todoId } }) => {
        await Promise.allSettled([
            queryClient.ensureQueryData(getTodoQueryOptions(todoId)),
            queryClient.ensureQueryData(listTodoItemsQueryOptions(todoId)),
        ])
    },
})

function RouteComponent() {
    const todoId = Route.useParams().todoId
    const listTodoItemQuery = useSuspenseQuery(
        listTodoItemsQueryOptions(todoId)
    )
    return (
        <div className="p-2 flex flex-col gap-1">
            <div>
                <Outlet />
            </div>
            {listTodoItemQuery.data.items.length === 0 ? (
                <div className="text-3xl py-10 text-center">Empty</div>
            ) : (
                <ul>
                    {listTodoItemQuery.data.items.map((i) => (
                        <li key={i.title}>{i.title}</li>
                    ))}
                </ul>
            )}
        </div>
    )
}

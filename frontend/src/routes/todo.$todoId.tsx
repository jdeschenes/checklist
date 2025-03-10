import { createFileRoute } from '@tanstack/react-router'
import { getTodoQueryOptions } from '@/api/todoQueryOptions'
import { listTodoItemsQueryOptions } from '@/api/todoItemQueryOptions'
import { useSuspenseQuery } from '@tanstack/react-query'

export const Route = createFileRoute('/todo/$todoId')({
    component: RouteComponent,
    loader: ({ context: { queryClient }, params: { todoId } }) => {
        queryClient.ensureQueryData(getTodoQueryOptions(todoId))
        queryClient.ensureQueryData(listTodoItemsQueryOptions(todoId))
    },
})

function RouteComponent() {
    const todoId = Route.useParams().todoId
    const getTodoQuery = useSuspenseQuery(getTodoQueryOptions(todoId))
    const listTodoItemQuery = useSuspenseQuery(
        listTodoItemsQueryOptions(todoId)
    )
    return (
        <div>
            <h1>{getTodoQuery.data.name}</h1>
            <ul>
                {listTodoItemQuery.data.items.map((i) => (
                    <li>{i.title}</li>
                ))}
            </ul>
        </div>
    )
}

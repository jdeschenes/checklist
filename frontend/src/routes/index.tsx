import { createFileRoute, Link } from '@tanstack/react-router'
import { useSuspenseQuery } from '@tanstack/react-query'
import { listTodoQueryOptions } from '@/api/todoQueryOptions'
import useCreateTodo from '@/api/useCreateTodo'

export const Route = createFileRoute('/')({
    component: Index,
    loader: ({ context: { queryClient } }) =>
        queryClient.ensureQueryData(listTodoQueryOptions),
})

function Index() {
    const listTodoQuery = useSuspenseQuery(listTodoQueryOptions)
    const todos = listTodoQuery.data
    const createTodoMutation = useCreateTodo()
    return (
        <div>
            <h1 className="text-3xl font-bold underline">TODOS</h1>
            {todos.items.length === 0 ? (
                'EMPTY TODO'
            ) : (
                <ul>
                    {todos.items.map((i) => (
                        <li>
                            <Link
                                key={i.name}
                                to="/todo/$todoId"
                                params={{ todoId: i.name }}
                            >
                                {i.name}
                            </Link>
                        </li>
                    ))}
                </ul>
            )}
            <Link to="/todo/new">Create todo</Link>
        </div>
    )
}

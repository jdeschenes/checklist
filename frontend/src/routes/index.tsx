import { createFileRoute, Link } from '@tanstack/react-router'
import { useSuspenseQuery } from '@tanstack/react-query'
import { listTodoQueryOptions } from '@/api/todoQueryOptions'
import useCreateTodo from '@/api/useCreateTodo'
import { buttonVariants } from '@/components/ui/button'
import { Plus } from 'lucide-react'

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
        <div className="p-2 flex flex-col gap-1">
            <div className="flex flex-auto my-2">
                <h1 className="text-xl font-bold text-gray-800 mb-6 grow">
                    TODOS
                </h1>
                <Link
                    className={buttonVariants({ variant: 'default' })}
                    to="/todo/new"
                >
                    <Plus /> New
                </Link>
            </div>
            {todos.items.length === 0 ? (
                <div className="text-3xl py-10 text-center">EMPTY TODO</div>
            ) : (
                <ul className="space-y-4">
                    {todos.items.map((i) => (
                        <li
                            key={i.name}
                            className="flex justify-between items-center bg-gray-50 rounded-lg shadow-sm hover:bg-gray:100 transition"
                        >
                            <Link
                                className="w-full h-full p-3"
                                key={i.name}
                                to="/todo/$todoId"
                                params={{ todoId: i.name }}
                            >
                                <span className="text-gray-700 h-100">
                                    {i.name}
                                </span>
                            </Link>
                        </li>
                    ))}
                </ul>
            )}
        </div>
    )
}

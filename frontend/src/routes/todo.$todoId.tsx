import { createFileRoute, Link, Outlet } from '@tanstack/react-router'
import { getTodoQueryOptions } from '@/api/todoQueryOptions'
import { listTodoItemsQueryOptions } from '@/api/todoItemQueryOptions'
import { useSuspenseQuery } from '@tanstack/react-query'
import { CheckCircle2, Circle, Calendar } from 'lucide-react'

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
        <div className="flex flex-col gap-4">
            <div>
                <Outlet />
            </div>
            {listTodoItemQuery.data.items.length === 0 ? (
                <div className="flex flex-col items-center justify-center py-16 px-4 bg-gray-50 rounded-lg border border-dashed border-gray-200">
                    <div className="text-2xl font-medium text-gray-500 mb-2">No tasks yet</div>
                    <p className="text-gray-400 text-sm">Add a new task to get started</p>
                </div>
            ) : (
                <ul className="space-y-2">
                    {listTodoItemQuery.data.items.map((item) => (
                        <li 
                            key={item.todo_item_id}
                            className="group flex items-center gap-3 p-4 bg-white rounded-lg border border-gray-100 hover:border-gray-200 transition-all hover:shadow-sm"
                        >
                            <div className="flex-shrink-0">
                                {item.is_complete ? (
                                    <CheckCircle2 className="h-5 w-5 text-green-500" />
                                ) : (
                                    <Circle className="h-5 w-5 text-gray-300 group-hover:text-gray-400" />
                                )}
                            </div>
                            <div className="flex-grow min-w-0">
                                <div className={`text-sm font-medium ${item.is_complete ? 'text-gray-400 line-through' : 'text-gray-900'}`}>
                                    {item.title}
                                </div>
                                {item.due_date && (
                                    <div className="flex items-center gap-1 text-xs text-gray-500 mt-1">
                                        <Calendar className="h-3 w-3" />
                                        <span>Due {new Date(item.due_date).toLocaleDateString()}</span>
                                    </div>
                                )}
                            </div>
                        </li>
                    ))}
                </ul>
            )}
        </div>
    )
}

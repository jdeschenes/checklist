import { createFileRoute, Outlet, useLocation } from '@tanstack/react-router'
import { getTodoQueryOptions } from '@/api/todoQueryOptions'
import { listTodoItemsQueryOptions } from '@/api/todoItemQueryOptions'
import { useSuspenseQuery } from '@tanstack/react-query'
import { CheckCircle2, Circle, Calendar } from 'lucide-react'
import useCompleteTodoItem from '@/api/useCompleteTodoItem'
import * as React from 'react'

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
    const location = useLocation()
    const listTodoItemQuery = useSuspenseQuery(
        listTodoItemsQueryOptions(todoId)
    )
    
    // Check if we're on a subpage (templates, template edit, new item)
    const isOnSubPage = location.pathname !== `/todo/${todoId}`
    const completeTodoItemMutation = useCompleteTodoItem(todoId)
    const [pendingCompletions, setPendingCompletions] = React.useState<
        Set<string>
    >(new Set())
    const timeoutRefs = React.useRef<
        Map<string, ReturnType<typeof setTimeout>>
    >(new Map())

    const handleItemClick = React.useCallback(
        (itemId: string, isComplete: boolean) => {
            // Don't handle click if already completed
            if (isComplete) return

            // Check if item is currently pending
            const isPending = pendingCompletions.has(itemId)

            if (isPending) {
                // Cancel the pending completion
                const existingTimeout = timeoutRefs.current.get(itemId)
                if (existingTimeout) {
                    clearTimeout(existingTimeout)
                    timeoutRefs.current.delete(itemId)
                }

                // Remove from pending completions
                setPendingCompletions((prev) => {
                    const newSet = new Set(prev)
                    newSet.delete(itemId)
                    return newSet
                })
            } else {
                // Start pending completion
                setPendingCompletions((prev) => new Set(prev).add(itemId))

                // Set timeout for 1 second debounce
                const timeout = setTimeout(() => {
                    completeTodoItemMutation.mutate(itemId, {
                        onSettled: () => {
                            // Remove from pending completions when API call completes
                            setPendingCompletions((prev) => {
                                const newSet = new Set(prev)
                                newSet.delete(itemId)
                                return newSet
                            })
                            timeoutRefs.current.delete(itemId)
                        },
                    })
                }, 1000)

                timeoutRefs.current.set(itemId, timeout)
            }
        },
        [completeTodoItemMutation, pendingCompletions]
    )

    // Cleanup timeouts on unmount
    React.useEffect(() => {
        return () => {
            timeoutRefs.current.forEach((timeout) => clearTimeout(timeout))
        }
    }, [])
    return (
        <div className="flex flex-col gap-4 p-4 sm:p-6">
            <div>
                <Outlet />
            </div>
            {!isOnSubPage && (listTodoItemQuery.data.items.length === 0 ? (
                <div className="flex flex-col items-center justify-center py-12 sm:py-16 px-4 bg-gray-50 rounded-lg border border-dashed border-gray-200">
                    <div className="text-lg sm:text-2xl font-medium text-gray-500 mb-2">
                        No tasks yet
                    </div>
                    <p className="text-gray-400 text-sm text-center">
                        Add a new task to get started
                    </p>
                </div>
            ) : (
                <ul className="space-y-2">
                    {listTodoItemQuery.data.items.map((item) => {
                        const isPending = pendingCompletions.has(
                            item.todo_item_id
                        )
                        const isClickable = !item.is_complete

                        return (
                            <li
                                key={item.todo_item_id}
                                onClick={() =>
                                    handleItemClick(
                                        item.todo_item_id,
                                        item.is_complete
                                    )
                                }
                                className={`group flex items-start sm:items-center gap-3 p-3 sm:p-4 bg-white rounded-lg border transition-all ${
                                    isClickable
                                        ? 'cursor-pointer hover:shadow-sm'
                                        : 'border-gray-100'
                                } ${
                                    isPending
                                        ? 'border-green-200 bg-green-50 hover:border-green-300'
                                        : isClickable
                                          ? 'border-gray-100 hover:border-gray-200'
                                          : 'border-gray-100'
                                }`}
                            >
                                <div className="flex-shrink-0 mt-0.5 sm:mt-0">
                                    {item.is_complete ? (
                                        <CheckCircle2 className="h-5 w-5 text-green-500" />
                                    ) : isPending ? (
                                        <CheckCircle2 className="h-5 w-5 text-green-500 animate-pulse" />
                                    ) : (
                                        <Circle className="h-5 w-5 text-gray-300 group-hover:text-gray-400" />
                                    )}
                                </div>
                                <div className="flex-grow min-w-0">
                                    <div
                                        className={`text-sm sm:text-base font-medium leading-relaxed ${item.is_complete ? 'text-gray-400 line-through' : 'text-gray-900'}`}
                                    >
                                        {item.title}
                                    </div>
                                    {item.due_date && (
                                        <div className="flex items-center gap-1 text-xs text-gray-500 mt-1 w-full">
                                            <Calendar className="h-3 w-3" />
                                            <span className="w-full">
                                                Due {item.due_date}
                                            </span>
                                        </div>
                                    )}
                                </div>
                            </li>
                        )
                    })}
                </ul>
            ))}
        </div>
    )
}

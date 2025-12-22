import { createFileRoute, Outlet, useLocation } from '@tanstack/react-router'
import { getTodoQueryOptions } from '@/api/todoQueryOptions'
import { listTodoItemsQueryOptions } from '@/api/todoItemQueryOptions'
import { useSuspenseQuery } from '@tanstack/react-query'
import { CheckCircle2, Circle, Calendar } from 'lucide-react'
import useCompleteTodoItem from '@/api/useCompleteTodoItem'
import useCreateTodoItem from '@/api/useCreateTodoItem'
import { Input } from '@/components/ui/input'
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
    const createTodoItemMutation = useCreateTodoItem(todoId)
    const [pendingCompletions, setPendingCompletions] = React.useState<
        Set<string>
    >(new Set())
    const [quickTitle, setQuickTitle] = React.useState('')
    const [quickError, setQuickError] = React.useState<string | null>(null)
    const completionTimeoutRef = React.useRef<ReturnType<
        typeof setTimeout
    > | null>(null)
    const pendingCompletionsRef = React.useRef(pendingCompletions)
    const inFlightCompletionsRef = React.useRef<Set<string>>(new Set())

    React.useEffect(() => {
        pendingCompletionsRef.current = pendingCompletions
    }, [pendingCompletions])

    const clearCompletionTimeout = React.useCallback(() => {
        if (completionTimeoutRef.current) {
            clearTimeout(completionTimeoutRef.current)
            completionTimeoutRef.current = null
        }
    }, [])

    const resetCompletionWindow = React.useCallback(() => {
        clearCompletionTimeout()

        const pending = Array.from(pendingCompletionsRef.current).filter(
            (itemId) => !inFlightCompletionsRef.current.has(itemId)
        )

        if (pending.length === 0) {
            return
        }

        completionTimeoutRef.current = setTimeout(() => {
            const itemsToComplete = Array.from(
                pendingCompletionsRef.current
            ).filter((itemId) => !inFlightCompletionsRef.current.has(itemId))

            if (itemsToComplete.length === 0) {
                return
            }

            itemsToComplete.forEach((itemId) => {
                inFlightCompletionsRef.current.add(itemId)
                completeTodoItemMutation.mutate(itemId, {
                    onSettled: () => {
                        inFlightCompletionsRef.current.delete(itemId)
                        setPendingCompletions((prev) => {
                            if (!prev.has(itemId)) {
                                return prev
                            }
                            const newSet = new Set(prev)
                            newSet.delete(itemId)
                            return newSet
                        })
                    },
                })
            })
        }, 1000)
    }, [clearCompletionTimeout, completeTodoItemMutation])

    React.useEffect(() => {
        resetCompletionWindow()
    }, [pendingCompletions, resetCompletionWindow])

    const handleItemClick = React.useCallback(
        (itemId: string, isComplete: boolean) => {
            if (isComplete) return

            setPendingCompletions((prev) => {
                const newSet = new Set(prev)

                if (newSet.has(itemId)) {
                    newSet.delete(itemId)
                } else {
                    newSet.add(itemId)
                }

                return newSet
            })
        },
        []
    )

    // Cleanup debounce timeout on unmount
    React.useEffect(() => {
        return () => {
            clearCompletionTimeout()
        }
    }, [clearCompletionTimeout])

    const handleQuickAddSubmit = React.useCallback(
        (event: React.FormEvent<HTMLFormElement>) => {
            event.preventDefault()

            const trimmedTitle = quickTitle.trim()
            if (!trimmedTitle) return

            setQuickError(null)
            const submittedTitle = trimmedTitle
            setQuickTitle('')
            resetCompletionWindow()
            createTodoItemMutation.mutate(
                {
                    todoId,
                    data: {
                        title: trimmedTitle,
                    },
                },
                {
                    onError: (error) => {
                        setQuickError(
                            error instanceof Error
                                ? error.message
                                : 'Failed to add task'
                        )
                        setQuickTitle((current) =>
                            current ? current : submittedTitle
                        )
                    },
                }
            )
        },
        [createTodoItemMutation, quickTitle, resetCompletionWindow, todoId]
    )

    return (
        <div className="flex flex-col gap-1 p-4 sm:p-6">
            <div>
                <Outlet />
            </div>
            {!isOnSubPage && (
                <div className="space-y-3">
                    <form onSubmit={handleQuickAddSubmit}>
                        <label htmlFor="quick-add" className="sr-only">
                            Add a task
                        </label>
                        <Input
                            id="quick-add"
                            name="quick-add"
                            type="text"
                            value={quickTitle}
                            onChange={(event) => {
                                setQuickTitle(event.target.value)
                                if (quickError) {
                                    setQuickError(null)
                                }
                            }}
                            placeholder="Add a task and press Enter"
                            autoComplete="off"
                        />
                        {quickError && (
                            <p className="text-sm text-red-600">{quickError}</p>
                        )}
                    </form>
                    {listTodoItemQuery.data.items.length === 0 ? (
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
                    )}
                </div>
            )}
        </div>
    )
}

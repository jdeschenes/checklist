import { createFileRoute, Link, useNavigate } from '@tanstack/react-router'
import { useSuspenseQuery } from '@tanstack/react-query'
import { Plus, Trash2, Clock } from 'lucide-react'
import { Button, buttonVariants } from '@/components/ui/button'
import { getTodoQueryOptions } from '@/api/todoQueryOptions'
import useDeleteTodo from '@/api/useDeleteTodo'
import * as React from 'react'

export const Route = createFileRoute('/todo/$todoId/')({
    component: RouteComponent,
})

function RouteComponent() {
    const todoId = Route.useParams().todoId
    const navigate = useNavigate()
    const getTodoQuery = useSuspenseQuery(getTodoQueryOptions(todoId))
    const deleteTodoMutation = useDeleteTodo()
    const [showDeleteConfirm, setShowDeleteConfirm] = React.useState(false)

    const handleDeleteClick = React.useCallback(() => {
        setShowDeleteConfirm(true)
    }, [])

    const handleDeleteConfirm = React.useCallback(() => {
        deleteTodoMutation.mutate(todoId, {
            onSuccess: () => {
                void navigate({ to: '/' })
            },
        })
    }, [deleteTodoMutation, todoId, navigate])

    const handleDeleteCancel = React.useCallback(() => {
        setShowDeleteConfirm(false)
    }, [])

    return (
        <div className="bg-white rounded-lg shadow-sm p-4 sm:p-6 mb-4">
            <div className="flex flex-col gap-4">
                <div className="flex items-center justify-between">
                    <h1 className="text-xl sm:text-2xl font-semibold text-gray-900">
                        {getTodoQuery.data.name}
                    </h1>
                    <Button
                        variant="ghost"
                        size="sm"
                        onClick={handleDeleteClick}
                        className="h-8 w-8 p-0 text-red-500 hover:text-red-600 hover:bg-red-50"
                    >
                        <Trash2 className="h-4 w-4" />
                    </Button>
                </div>
                {showDeleteConfirm ? (
                    <div className="flex flex-col items-end gap-2">
                        <span className="text-sm text-red-600 text-right">
                            Delete this todo? This cannot be undone.
                        </span>
                        <div className="flex gap-2">
                            <Button
                                variant="outline"
                                size="sm"
                                onClick={handleDeleteCancel}
                                disabled={deleteTodoMutation.isPending}
                            >
                                Cancel
                            </Button>
                            <Button
                                variant="destructive"
                                size="sm"
                                onClick={handleDeleteConfirm}
                                disabled={deleteTodoMutation.isPending}
                            >
                                {deleteTodoMutation.isPending
                                    ? 'Deleting...'
                                    : 'Delete'}
                            </Button>
                        </div>
                    </div>
                ) : (
                    <div>
                        <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 mb-4">
                            <p className="text-sm text-gray-500">
                                Manage your tasks and stay organized
                            </p>
                            <div className="flex flex-wrap gap-2">
                                <Link
                                    className={buttonVariants({
                                        variant: 'outline',
                                        size: 'sm',
                                        className: 'gap-2',
                                    })}
                                    to="/todo/$todoId/templates"
                                    params={{ todoId }}
                                >
                                    <Clock className="h-4 w-4" /> Templates
                                </Link>
                                <Link
                                    className={buttonVariants({
                                        variant: 'default',
                                        size: 'sm',
                                        className:
                                            'gap-2 shadow-sm hover:shadow-md transition-shadow',
                                    })}
                                    to="/todo/$todoId/new"
                                    params={{ todoId }}
                                    search={{ recurring: false }}
                                >
                                    <Plus className="h-4 w-4" /> New Task
                                </Link>
                            </div>
                        </div>
                    </div>
                )}
            </div>
        </div>
    )
}

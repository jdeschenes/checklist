import { createFileRoute, Link } from '@tanstack/react-router'
import { Plus, Edit, Trash2, Clock, Play, Pause } from 'lucide-react'
import { Button, buttonVariants } from '@/components/ui/button'
import {
    useListRecurringTemplates,
    useDeleteRecurringTemplate,
} from '@/api/useRecurringTemplateOperations'
import { formatDistanceToNow } from 'date-fns'
import * as React from 'react'

export const Route = createFileRoute('/todo/$todoId/templates')({
    component: RouteComponent,
})

function formatRecurrenceInterval(interval: {
    months?: number
    days?: number
    microseconds?: number
}) {
    const parts: string[] = []

    if (interval.months && interval.months > 0) {
        parts.push(`${interval.months} month${interval.months > 1 ? 's' : ''}`)
    }
    if (interval.days && interval.days > 0) {
        parts.push(`${interval.days} day${interval.days > 1 ? 's' : ''}`)
    }

    return parts.join(', ') || 'Never'
}

function RouteComponent() {
    const todoId = Route.useParams().todoId
    const templatesQuery = useListRecurringTemplates(todoId)
    const deleteTemplateMutation = useDeleteRecurringTemplate()

    const [deleteConfirmId, setDeleteConfirmId] = React.useState<string | null>(
        null
    )

    const handleDeleteClick = React.useCallback((templateId: string) => {
        setDeleteConfirmId(templateId)
    }, [])

    const handleDeleteConfirm = React.useCallback(
        (templateId: string) => {
            deleteTemplateMutation.mutate(
                { todo_name: todoId, template_id: templateId },
                {
                    onSuccess: () => {
                        setDeleteConfirmId(null)
                    },
                }
            )
        },
        [deleteTemplateMutation, todoId]
    )

    const handleDeleteCancel = React.useCallback(() => {
        setDeleteConfirmId(null)
    }, [])

    return (
        <div className="bg-white rounded-lg shadow-sm p-4 sm:p-6 mb-4">
            <div className="flex flex-col gap-6">
                {templatesQuery.data?.templates.length !== 0 ? (
                    <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
                        <div className="flex gap-2">
                            <Link
                                className={buttonVariants({
                                    variant: 'default',
                                    size: 'sm',
                                    className:
                                        'gap-2 shadow-sm hover:shadow-md transition-shadow',
                                })}
                                to="/todo/$todoId/new"
                                params={{ todoId }}
                                search={{ recurring: true }}
                            >
                                <Plus className="h-4 w-4" /> New Template
                            </Link>
                        </div>
                    </div>
                ) : null}

                {templatesQuery.data?.templates.length === 0 ? (
                    <div className="text-center py-12">
                        <Clock className="mx-auto h-12 w-12 text-gray-400" />
                        <h3 className="mt-2 text-sm font-medium text-gray-900">
                            No templates
                        </h3>
                        <p className="mt-1 text-sm text-gray-500">
                            Get started by creating a recurring template
                        </p>
                        <div className="mt-6">
                            <Link
                                className={buttonVariants({
                                    variant: 'default',
                                    size: 'sm',
                                    className: 'gap-2',
                                })}
                                to="/todo/$todoId/new"
                                params={{ todoId }}
                                search={{ recurring: true }}
                            >
                                <Plus className="h-4 w-4" /> New Template
                            </Link>
                        </div>
                    </div>
                ) : (
                    <div className="space-y-4">
                        {templatesQuery.data?.templates.map((template) => (
                            <div
                                key={template.template_id}
                                className="border border-gray-200 rounded-lg p-4 hover:border-gray-300 transition-colors"
                            >
                                <div className="flex items-start justify-between">
                                    <div className="flex-1 min-w-0">
                                        <div className="flex items-center gap-2 mb-2">
                                            <h3 className="text-lg font-medium text-gray-900 truncate">
                                                {template.title}
                                            </h3>
                                            <div className="flex items-center gap-1">
                                                {template.is_active ? (
                                                    <span className="inline-flex items-center gap-1 px-2 py-1 text-xs font-medium text-green-700 bg-green-50 border border-green-200 rounded-full">
                                                        <Play className="h-3 w-3" />
                                                        Active
                                                    </span>
                                                ) : (
                                                    <span className="inline-flex items-center gap-1 px-2 py-1 text-xs font-medium text-gray-700 bg-gray-50 border border-gray-200 rounded-full">
                                                        <Pause className="h-3 w-3" />
                                                        Inactive
                                                    </span>
                                                )}
                                            </div>
                                        </div>
                                        <div className="space-y-1 text-sm text-gray-500">
                                            <p>
                                                <span className="font-medium">
                                                    Repeats:
                                                </span>{' '}
                                                Every{' '}
                                                {formatRecurrenceInterval(
                                                    template.recurrence_interval
                                                )}
                                            </p>
                                            <p>
                                                <span className="font-medium">
                                                    Start Date:
                                                </span>{' '}
                                                {new Date(
                                                    template.start_date
                                                ).toLocaleDateString()}
                                            </p>
                                            {template.end_date && (
                                                <p>
                                                    <span className="font-medium">
                                                        End Date:
                                                    </span>{' '}
                                                    {new Date(
                                                        template.end_date
                                                    ).toLocaleDateString()}
                                                </p>
                                            )}
                                            {template.last_generated_date && (
                                                <p>
                                                    <span className="font-medium">
                                                        Last Generated:
                                                    </span>{' '}
                                                    {formatDistanceToNow(
                                                        new Date(
                                                            template.last_generated_date
                                                        ),
                                                        { addSuffix: true }
                                                    )}
                                                </p>
                                            )}
                                        </div>
                                    </div>
                                    <div className="flex items-center gap-2 ml-4">
                                        {deleteConfirmId ===
                                        template.template_id ? (
                                            <div className="flex items-center gap-2">
                                                <Button
                                                    variant="outline"
                                                    size="sm"
                                                    onClick={handleDeleteCancel}
                                                    disabled={
                                                        deleteTemplateMutation.isPending
                                                    }
                                                >
                                                    Cancel
                                                </Button>
                                                <Button
                                                    variant="destructive"
                                                    size="sm"
                                                    onClick={() =>
                                                        handleDeleteConfirm(
                                                            template.template_id
                                                        )
                                                    }
                                                    disabled={
                                                        deleteTemplateMutation.isPending
                                                    }
                                                >
                                                    {deleteTemplateMutation.isPending
                                                        ? 'Deleting...'
                                                        : 'Delete'}
                                                </Button>
                                            </div>
                                        ) : (
                                            <>
                                                <Link
                                                    to="/todo/$todoId/template/$templateId/edit"
                                                    params={{
                                                        todoId,
                                                        templateId:
                                                            template.template_id,
                                                    }}
                                                    className="inline-flex items-center justify-center h-8 w-8 p-0 text-gray-400 hover:text-blue-600 hover:bg-blue-50 rounded-md transition-colors"
                                                >
                                                    <Edit className="h-4 w-4" />
                                                </Link>
                                                <Button
                                                    variant="ghost"
                                                    size="sm"
                                                    onClick={() =>
                                                        handleDeleteClick(
                                                            template.template_id
                                                        )
                                                    }
                                                    className="h-8 w-8 p-0 text-gray-400 hover:text-red-600 hover:bg-red-50"
                                                >
                                                    <Trash2 className="h-4 w-4" />
                                                </Button>
                                            </>
                                        )}
                                    </div>
                                </div>
                            </div>
                        ))}
                    </div>
                )}
            </div>
        </div>
    )
}

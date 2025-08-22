import * as React from 'react'
import { format } from 'date-fns'
import { Input } from '@/components/ui/input'
import { Button } from '@/components/ui/button'
import { createFileRoute, useNavigate, Link } from '@tanstack/react-router'
import useCreateTodoItem from '@/api/useCreateTodoItem'
import { DatePicker } from '@/components/ui/datepicker'
import { IntervalPicker } from '@/components/ui/interval-picker'
import { ArrowLeft } from 'lucide-react'
import { RecurrenceInterval } from '@/api'
import { useCreateRecurringTemplate } from '@/api/useRecurringTemplateOperations'

export const Route = createFileRoute('/todo/$todoId/new')({
    component: RouteComponent,
    validateSearch: (search: Record<string, unknown>) => ({
        recurring: search.recurring === true || search.recurring === 'true',
    }),
})

function RouteComponent() {
    const todoId = Route.useParams().todoId
    const search = Route.useSearch()
    const navigate = useNavigate()
    const [selectedDate, setSelectedDate] = React.useState<Date | undefined>()
    const [title, setTitle] = React.useState('')
    const [isRecurring, setIsRecurring] = React.useState(
        search.recurring || false
    )
    const [recurrenceInterval, setRecurrenceInterval] =
        React.useState<RecurrenceInterval>({ days: 1 })
    const [endDate, setEndDate] = React.useState<Date | undefined>()
    const createTodoItemMutation = useCreateTodoItem(todoId)
    const createRecurringTemplateMutation = useCreateRecurringTemplate()

    const handleFormSubmit = React.useCallback(
        (event: React.FormEvent) => {
            event.preventDefault()
            const formData = new FormData(event.target as HTMLFormElement)
            const titleValue = formData.get('title') as string

            if (!titleValue.trim()) return

            if (isRecurring) {
                // Create recurring template
                createRecurringTemplateMutation.mutate(
                    {
                        todo_name: todoId,
                        template: {
                            title: titleValue.trim(),
                            recurrence_interval: recurrenceInterval,
                            start_date: selectedDate
                                ? format(selectedDate, 'yyyy-MM-dd')
                                : undefined,
                            end_date: endDate
                                ? format(endDate, 'yyyy-MM-dd')
                                : undefined,
                        },
                    },
                    {
                        onSuccess: () => {
                            navigate({
                                to: '/todo/$todoId',
                                params: {
                                    todoId,
                                },
                            })
                        },
                    }
                )
            } else {
                // Create regular todo item
                createTodoItemMutation.mutate(
                    {
                        todoId: todoId,
                        data: {
                            title: titleValue.trim(),
                            due_date: selectedDate
                                ? format(selectedDate, 'yyyy-MM-dd')
                                : undefined,
                        },
                    },
                    {
                        onSuccess: () => {
                            navigate({
                                to: '/todo/$todoId',
                                params: {
                                    todoId,
                                },
                            })
                        },
                    }
                )
            }
        },
        [
            createTodoItemMutation,
            createRecurringTemplateMutation,
            selectedDate,
            todoId,
            navigate,
            isRecurring,
            recurrenceInterval,
            endDate,
        ]
    )

    return (
        <form onSubmit={handleFormSubmit}>
            <div className="bg-white rounded-lg shadow-sm p-4 sm:p-6 mb-4">
                <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 mb-6">
                    <div className="space-y-1">
                        <h1 className="text-xl sm:text-2xl font-semibold text-gray-900">
                            New Task
                        </h1>
                        <p className="text-sm text-gray-500">
                            Add a new item to your todo list
                        </p>
                    </div>
                    <div className="flex gap-2">
                        <Link
                            to="/todo/$todoId"
                            params={{ todoId }}
                            className="inline-flex items-center justify-center h-9 w-9 p-0 text-gray-400 hover:text-gray-600 hover:bg-gray-100 rounded-md transition-colors"
                        >
                            <ArrowLeft className="h-4 w-4" />
                        </Link>
                        <Button
                            type="submit"
                            disabled={
                                !title.trim() ||
                                createTodoItemMutation.isPending ||
                                createRecurringTemplateMutation.isPending
                            }
                            className="ml-auto"
                        >
                            {createTodoItemMutation.isPending ||
                            createRecurringTemplateMutation.isPending
                                ? 'Creating...'
                                : isRecurring
                                  ? 'Create Template'
                                  : 'Create Task'}
                        </Button>
                    </div>
                </div>
                <div className="space-y-4">
                    <div>
                        <label
                            htmlFor="title"
                            className="block text-sm font-medium text-gray-700 mb-1"
                        >
                            Task Title
                        </label>
                        <Input
                            id="title"
                            name="title"
                            type="text"
                            value={title}
                            onChange={(e) => setTitle(e.target.value)}
                            placeholder="Enter todo item..."
                            autoFocus
                            className="w-full"
                            required
                        />
                    </div>
                    <div>
                        <label
                            htmlFor="date"
                            className="block text-sm font-medium text-gray-700 mb-1"
                        >
                            {isRecurring
                                ? 'Start Date (Optional)'
                                : 'Due Date (Optional)'}
                        </label>
                        <DatePicker
                            date={selectedDate}
                            onDateChange={setSelectedDate}
                        />
                    </div>

                    <div className="flex items-center space-x-2">
                        <input
                            id="recurring"
                            type="checkbox"
                            checked={isRecurring}
                            onChange={(e) => setIsRecurring(e.target.checked)}
                            className="rounded border-input"
                        />
                        <label
                            htmlFor="recurring"
                            className="text-sm font-medium text-gray-700"
                        >
                            Make this a recurring template
                        </label>
                    </div>

                    {isRecurring && (
                        <div className="space-y-4 p-4 bg-gray-50 rounded-lg border">
                            <div>
                                <label className="block text-sm font-medium text-gray-700 mb-2">
                                    Recurrence Settings
                                </label>
                                <IntervalPicker
                                    value={recurrenceInterval}
                                    onChange={setRecurrenceInterval}
                                />
                            </div>

                            <div>
                                <label className="block text-sm font-medium text-gray-700 mb-1">
                                    End Date (Optional)
                                </label>
                                <DatePicker
                                    date={endDate}
                                    onDateChange={setEndDate}
                                />
                                {endDate && (
                                    <Button
                                        type="button"
                                        variant="ghost"
                                        size="sm"
                                        onClick={() => setEndDate(undefined)}
                                        className="mt-1"
                                    >
                                        Clear
                                    </Button>
                                )}
                            </div>
                        </div>
                    )}
                </div>
            </div>
        </form>
    )
}

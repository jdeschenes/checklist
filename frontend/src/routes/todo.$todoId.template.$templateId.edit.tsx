import * as React from 'react'
import { format } from 'date-fns'
import { Input } from '@/components/ui/input'
import { Button } from '@/components/ui/button'
import { createFileRoute, useNavigate } from '@tanstack/react-router'
import { DatePicker } from '@/components/ui/datepicker'
import { IntervalPicker } from '@/components/ui/interval-picker'
import { Save } from 'lucide-react'
import { RecurrenceInterval, UpdateRecurringTemplateRequest } from '@/api'
import { 
    useGetRecurringTemplate, 
    useUpdateRecurringTemplate 
} from '@/api/useRecurringTemplateOperations'

export const Route = createFileRoute('/todo/$todoId/template/$templateId/edit')({
    component: RouteComponent,
})

function RouteComponent() {
    const { todoId, templateId } = Route.useParams()
    const navigate = useNavigate()
    const templateQuery = useGetRecurringTemplate(todoId, templateId)
    const updateTemplateMutation = useUpdateRecurringTemplate()

    const [title, setTitle] = React.useState('')
    const [startDate, setStartDate] = React.useState<Date | undefined>()
    const [endDate, setEndDate] = React.useState<Date | undefined>()
    const [recurrenceInterval, setRecurrenceInterval] = React.useState<RecurrenceInterval>({ days: 1 })
    const [isActive, setIsActive] = React.useState(true)

    // Initialize form with existing template data
    React.useEffect(() => {
        if (templateQuery.data) {
            setTitle(templateQuery.data.title)
            setStartDate(new Date(templateQuery.data.start_date))
            setEndDate(templateQuery.data.end_date ? new Date(templateQuery.data.end_date) : undefined)
            setRecurrenceInterval(templateQuery.data.recurrence_interval)
            setIsActive(templateQuery.data.is_active)
        }
    }, [templateQuery.data])

    const handleFormSubmit = React.useCallback(
        (event: React.FormEvent) => {
            event.preventDefault()
            const formData = new FormData(event.target as HTMLFormElement)
            const titleValue = formData.get('title') as string

            if (!titleValue.trim()) return

            const updateRequest: UpdateRecurringTemplateRequest = {
                title: titleValue.trim(),
                recurrence_interval: recurrenceInterval,
                start_date: startDate ? format(startDate, 'yyyy-MM-dd') : undefined,
                end_date: endDate ? format(endDate, 'yyyy-MM-dd') : undefined,
                is_active: isActive,
            }

            updateTemplateMutation.mutate(
                {
                    todo_name: todoId,
                    template_id: templateId,
                    template: updateRequest,
                },
                {
                    onSuccess: () => {
                        navigate({
                            to: '/todo/$todoId/templates',
                            params: { todoId },
                        })
                    },
                }
            )
        },
        [
            updateTemplateMutation,
            todoId,
            templateId,
            startDate,
            endDate,
            recurrenceInterval,
            isActive,
            navigate,
        ]
    )

    if (templateQuery.isLoading) {
        return (
            <div className="bg-white rounded-lg shadow-sm p-4 sm:p-6 mb-4">
                <div className="animate-pulse space-y-4">
                    <div className="h-8 bg-gray-200 rounded w-1/3"></div>
                    <div className="space-y-2">
                        <div className="h-4 bg-gray-200 rounded w-1/4"></div>
                        <div className="h-10 bg-gray-200 rounded"></div>
                    </div>
                </div>
            </div>
        )
    }

    if (templateQuery.isError) {
        return (
            <div className="bg-white rounded-lg shadow-sm p-4 sm:p-6 mb-4">
                <div className="text-center py-12">
                    <h3 className="mt-2 text-sm font-medium text-gray-900">Template not found</h3>
                    <p className="mt-1 text-sm text-gray-500">
                        The template you're looking for doesn't exist or has been deleted.
                    </p>
                </div>
            </div>
        )
    }

    return (
        <form onSubmit={handleFormSubmit}>
            <div className="bg-white rounded-lg shadow-sm p-4 sm:p-6 mb-4">
                <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 mb-6">
                    <div className="space-y-1">
                        <h1 className="text-xl sm:text-2xl font-semibold text-gray-900">
                            Edit Template
                        </h1>
                        <p className="text-sm text-gray-500">
                            Modify the recurring template settings
                        </p>
                    </div>
                    <div className="flex gap-2">
                        <Button
                            type="submit"
                            disabled={!title.trim() || updateTemplateMutation.isPending}
                            className="gap-2"
                        >
                            <Save className="h-4 w-4" />
                            {updateTemplateMutation.isPending ? 'Saving...' : 'Save Template'}
                        </Button>
                    </div>
                </div>

                <div className="space-y-6">
                    <div>
                        <label
                            htmlFor="title"
                            className="block text-sm font-medium text-gray-700 mb-1"
                        >
                            Template Title
                        </label>
                        <Input
                            id="title"
                            name="title"
                            type="text"
                            value={title}
                            onChange={(e) => setTitle(e.target.value)}
                            placeholder="Enter template title..."
                            autoFocus
                            className="w-full"
                            required
                        />
                    </div>

                    <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div>
                            <label
                                htmlFor="start-date"
                                className="block text-sm font-medium text-gray-700 mb-1"
                            >
                                Start Date
                            </label>
                            <DatePicker
                                date={startDate}
                                onDateChange={setStartDate}
                            />
                        </div>

                        <div>
                            <label
                                htmlFor="end-date"
                                className="block text-sm font-medium text-gray-700 mb-1"
                            >
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
                    </div>

                    <div className="flex items-center space-x-2">
                        <input
                            id="is-active"
                            type="checkbox"
                            checked={isActive}
                            onChange={(e) => setIsActive(e.target.checked)}
                            className="rounded border-input"
                        />
                        <label
                            htmlFor="is-active"
                            className="text-sm font-medium text-gray-700"
                        >
                            Template is active
                        </label>
                        <span className="text-xs text-gray-500">
                            (Inactive templates won't generate new items)
                        </span>
                    </div>

                    {templateQuery.data?.last_generated_date && (
                        <div className="text-sm text-gray-500 p-3 bg-blue-50 border border-blue-200 rounded-md">
                            <strong>Last Generated:</strong>{' '}
                            {new Date(templateQuery.data.last_generated_date).toLocaleDateString()}
                        </div>
                    )}

                </div>
            </div>
        </form>
    )
}
import * as React from 'react'
import { format } from 'date-fns'
import { Input } from '@/components/ui/input'
import { Button } from '@/components/ui/button'
import { createFileRoute, useNavigate, Link } from '@tanstack/react-router'
import useCreateTodoItem from '@/api/useCreateTodoItem'
import { DatePicker } from '@/components/ui/datepicker'
import { ArrowLeft } from 'lucide-react'

export const Route = createFileRoute('/todo/$todoId/new')({
    component: RouteComponent,
})

function RouteComponent() {
    const todoId = Route.useParams().todoId
    const navigate = useNavigate()
    const [selectedDate, setSelectedDate] = React.useState<Date | undefined>()
    const [title, setTitle] = React.useState('')
    const createTodoItemMutation = useCreateTodoItem(todoId)

    const handleFormSubmit = React.useCallback((event: React.FormEvent) => {
        event.preventDefault()
        const formData = new FormData(event.target as HTMLFormElement)
        const title = formData.get('title') as string
        
        if (!title.trim()) return
        
        createTodoItemMutation.mutate(
            {
                todoId: todoId,
                data: {
                    title: title.trim(),
                    due_date: selectedDate ? format(selectedDate, 'yyyy-MM-dd') : undefined,
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
    }, [createTodoItemMutation, selectedDate, todoId, navigate])

    return (
        <form onSubmit={handleFormSubmit}>
            <div className="bg-white rounded-lg shadow-sm p-4 sm:p-6 mb-4">
                <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4 mb-6">
                    <div className="space-y-1">
                        <h1 className="text-xl sm:text-2xl font-semibold text-gray-900">New Task</h1>
                        <p className="text-sm text-gray-500">Add a new item to your todo list</p>
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
                            disabled={!title.trim() || createTodoItemMutation.isPending}
                            className="ml-auto"
                        >
                            {createTodoItemMutation.isPending ? 'Creating...' : 'Create Task'}
                        </Button>
                    </div>
                </div>
                <div className="space-y-4">
                    <div>
                        <label htmlFor="title" className="block text-sm font-medium text-gray-700 mb-1">
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
                        <label htmlFor="due-date" className="block text-sm font-medium text-gray-700 mb-1">
                            Due Date (Optional)
                        </label>
                        <DatePicker 
                            date={selectedDate} 
                            onDateChange={setSelectedDate}
                            className="w-full"
                        />
                    </div>
                </div>
            </div>
        </form>
    )
}

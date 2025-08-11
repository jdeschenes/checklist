import * as React from 'react'
import { format } from 'date-fns'
import { Input } from '@/components/ui/input'
import { Button } from '@/components/ui/button'
import { createFileRoute, useNavigate } from '@tanstack/react-router'
import useCreateTodoItem from '@/api/useCreateTodoItem'
import { DatePicker } from '@/components/ui/datepicker'

export const Route = createFileRoute('/todo/$todoId/new')({
    component: RouteComponent,
})

function RouteComponent() {
    const todoId = Route.useParams().todoId
    const navigate = useNavigate()
    const [selectedDate, setSelectedDate] = React.useState<Date | undefined>()
    const createTodoItemMutation = useCreateTodoItem(todoId)
    const submitCallback = React.useCallback(
        (event: React.SyntheticEvent) => {
            event.preventDefault()
            const target = event.target as typeof event.target & {
                title: { value: string }
            }
            createTodoItemMutation.mutate(
                {
                    todoId: todoId,
                    data: {
                        title: target.title.value,
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
        },
        [createTodoItemMutation, selectedDate]
    )
    return (
        <form className="flex flex-col gap-4" onSubmit={submitCallback}>
            <Input type="text" name="title" placeholder="Enter todo item..." autoFocus className="w-full" />
            <div className="flex gap-2 items-center">
                <DatePicker date={selectedDate} onDateChange={setSelectedDate} />
                <Button type="submit">Create</Button>
            </div>
        </form>
    )
}

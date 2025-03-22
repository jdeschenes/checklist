import * as React from 'react'
import { Input } from '@/components/ui/input'
import { Button } from '@/components/ui/button'
import { createFileRoute } from '@tanstack/react-router'
import useCreateTodoItem from '@/api/useCreateTodoItem'

export const Route = createFileRoute('/todo/$todoId/new')({
    component: RouteComponent,
})

function RouteComponent() {
    const todoId = Route.useParams().todoId
    const createTodoItemMutation = useCreateTodoItem(todoId)
    const submitCallback = React.useCallback(
        (event: React.SyntheticEvent) => {
            event.preventDefault()
            const target = event.target as typeof event.target & {
                name: { value: string }
            }
            createTodoItemMutation.mutate({
                todo_name: todoId,
            })
        },
        [createTodoItemMutation]
    )
    return (
        <form className="flex flex-auto" onSubmit={submitCallback}>
            <Input type="text" name="title" placeholder="todo" autoFocus />
            <Button type="submit">Create</Button>
        </form>
    )
}

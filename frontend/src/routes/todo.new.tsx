import * as React from 'react'
import { createFileRoute, useNavigate } from '@tanstack/react-router'
import useCreateTodo from '@/api/useCreateTodo'

export const Route = createFileRoute('/todo/new')({
    component: RouteComponent,
})

function RouteComponent() {
    const createTodoMutation = useCreateTodo()
    const navigate = useNavigate()
    const submitCallback = React.useCallback(
        (event: React.SyntheticEvent) => {
            event.preventDefault()
            const target = event.target as typeof event.target & {
                name: { value: string }
            }
            createTodoMutation.mutate(
                {
                    name: target.name.value,
                },
                {
                    onSuccess: () => {
                        navigate({
                            to: '/',
                        })
                    },
                    onError: (e) => {
                        console.log('ERROR', e)
                    },
                }
            )
        },
        [createTodoMutation]
    )
    return (
        <div>
            <h1>Create New Todo</h1>
            <form onSubmit={submitCallback}>
                <input type="text" name="name" />
                <input type="submit" value="Create" />
            </form>
        </div>
    )
}

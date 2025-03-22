import * as React from 'react'
import { createFileRoute, useNavigate } from '@tanstack/react-router'
import useCreateTodo from '@/api/useCreateTodo'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'

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
                            to: '/todo/$todoId',
                            params: {
                                todoId: target.name.value,
                            },
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
        <div className="p-2 flex flex-col gap-1">
            <h1 className="text-xl">Create New Todo</h1>
            <form className="flex flex-col gap-2" onSubmit={submitCallback}>
                <Input type="text" name="name" placeholder="Todo" autoFocus />
                <Button variant="default" type="submit">
                    Create
                </Button>
            </form>
        </div>
    )
}

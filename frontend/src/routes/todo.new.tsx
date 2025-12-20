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
    const [error, setError] = React.useState<string | null>(null)
    
    const submitCallback = React.useCallback(
        (event: React.FormEvent<HTMLFormElement>) => {
            event.preventDefault()
            event.stopPropagation()
            
            setError(null)
            
            const formData = new FormData(event.currentTarget)
            const todoName = formData.get('name') as string
            
            if (!todoName || todoName.trim() === '') {
                setError('Todo name is required')
                return
            }
            
            createTodoMutation.mutate(
                {
                    name: todoName.trim(),
                },
                {
                    onSuccess: () => {
                        const trimmedName = todoName.trim()
                        setTimeout(() => {
                            void navigate({
                                to: '/todo/$todoId',
                                params: {
                                    todoId: trimmedName,
                                },
                                replace: true,
                            })
                        }, 100)
                    },
                    onError: (error) => {
                        setError(error instanceof Error ? error.message : 'Failed to create todo')
                    },
                }
            )
        },
        [createTodoMutation, navigate]
    )
    return (
        <div className="p-2 flex flex-col gap-1">
            <h1 className="text-xl">Create New Todo</h1>
            {error && (
                <div className="bg-red-50 border border-red-200 text-red-600 px-3 py-2 rounded text-sm">
                    {error}
                </div>
            )}
            <form className="flex flex-col gap-2" onSubmit={submitCallback}>
                <Input 
                    type="text" 
                    name="name" 
                    placeholder="Todo" 
                    autoFocus 
                    disabled={createTodoMutation.isPending}
                />
                <Button 
                    variant="default" 
                    type="submit" 
                    disabled={createTodoMutation.isPending}
                >
                    {createTodoMutation.isPending ? 'Creating...' : 'Create'}
                </Button>
            </form>
        </div>
    )
}

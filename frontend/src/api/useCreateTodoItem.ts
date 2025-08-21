import { useMutation, useQueryClient } from '@tanstack/react-query'
import { CreateTodoItemRequest, FinalTodoItemAPI } from '.'

type HookInput = {
    todoId: string
    data: CreateTodoItemRequest
}

const useCreateTodoItem = (todoId: string) => {
    const queryClient = useQueryClient()
    const mutation = useMutation({
        mutationFn: (x: HookInput) =>
            FinalTodoItemAPI.CreateTodoItem(x.todoId, x.data),
        onSuccess: async () => {
            // Invalidate both the todo details and the todo items list
            await queryClient.invalidateQueries({ queryKey: ['todo', todoId] })
            await queryClient.invalidateQueries({
                queryKey: ['todo', todoId, 'item'],
            })
        },
    })
    return mutation
}

export default useCreateTodoItem

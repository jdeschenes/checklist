import { useMutation, useQueryClient } from '@tanstack/react-query'
import { FinalTodoAPI } from '.'

export default function useDeleteTodo() {
    const queryClient = useQueryClient()
    
    return useMutation({
        mutationFn: (todoId: string) => FinalTodoAPI.DeleteTodo(todoId),
        onSuccess: () => {
            // Invalidate todo list to refresh after deletion
            queryClient.invalidateQueries({
                queryKey: ['todos'],
            })
        },
    })
}
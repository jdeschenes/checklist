import { useMutation, useQueryClient } from '@tanstack/react-query'
import { FinalTodoItemAPI } from '.'
import { listTodoItemsQueryOptions } from './todoItemQueryOptions'

export default function useCompleteTodoItem(todoId: string) {
    const queryClient = useQueryClient()

    return useMutation({
        mutationFn: (itemId: string) =>
            FinalTodoItemAPI.CompleteTodoItem(todoId, itemId),
        onSuccess: () => {
            // Invalidate and refetch todo items to update the list
            queryClient.invalidateQueries({
                queryKey: listTodoItemsQueryOptions(todoId).queryKey,
            })
        },
    })
}

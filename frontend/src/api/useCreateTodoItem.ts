import { useMutation, useQueryClient } from '@tanstack/react-query'
import { FinalTodoItemAPI } from '.'
const useCreateTodoItem = (todoId: string) => {
    const queryClient = useQueryClient()
    const mutation = useMutation({
        mutationFn: FinalTodoItemAPI.CreateTodoItem,
        onSuccess: async () => {
            console.log('DONe')
            await queryClient.invalidateQueries({ queryKey: ['todo', todoId] })
        },
    })
    return mutation
}

export default useCreateTodoItem

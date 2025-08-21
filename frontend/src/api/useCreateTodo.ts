import { useMutation, useQueryClient } from '@tanstack/react-query'
import { CreateTodoRequest, FinalTodoAPI } from '.'
export default function useCreateTodo() {
    const queryClient = useQueryClient()
    const mutation = useMutation({
        mutationFn: (r: CreateTodoRequest) => FinalTodoAPI.CreateTodo(r),
        onSuccess: async () => {
            await queryClient.invalidateQueries({ queryKey: ['todo'] })
        },
    })
    return mutation
}

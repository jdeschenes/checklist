import { useMutation, useQueryClient } from '@tanstack/react-query'
import { FinalTodoAPI } from '.'
const useCreateTodo = () => {
    const queryClient = useQueryClient()
    const mutation = useMutation({
        mutationFn: FinalTodoAPI.CreateTodo,
        onSuccess: () => {
            console.log('DONe')
            queryClient.invalidateQueries({ queryKey: ['todo'] })
        },
    })
    return mutation
}

export default useCreateTodo

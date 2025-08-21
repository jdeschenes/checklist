import { useMutation, useQueryClient } from '@tanstack/react-query'
import { FinalRecurringTemplateAPI, CreateRecurringTemplateRequest } from '.'

export const useCreateRecurringTemplate = () => {
    const queryClient = useQueryClient()

    return useMutation({
        mutationFn: ({
            todo_name,
            template,
        }: {
            todo_name: string
            template: CreateRecurringTemplateRequest
        }) =>
            FinalRecurringTemplateAPI.CreateRecurringTemplate(
                todo_name,
                template
            ),
        onSuccess: (data, variables) => {
            queryClient.invalidateQueries({
                queryKey: ['recurring-templates', variables.todo_name],
            })
        },
    })
}

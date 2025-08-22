import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'
import {
    FinalRecurringTemplateAPI,
    FinalTodoItemAPI,
    CreateRecurringTemplateRequest,
    UpdateRecurringTemplateRequest,
} from '.'

export const useListRecurringTemplates = (todoName: string) => {
    return useQuery({
        queryKey: ['recurring-templates', todoName],
        queryFn: () =>
            FinalRecurringTemplateAPI.ListRecurringTemplates(todoName),
    })
}

export const useGetRecurringTemplate = (
    todoName: string,
    templateId: string
) => {
    return useQuery({
        queryKey: ['recurring-template', todoName, templateId],
        queryFn: () =>
            FinalRecurringTemplateAPI.GetRecurringTemplate(
                todoName,
                templateId
            ),
        enabled: !!todoName && !!templateId,
    })
}

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
            // Invalidate todo items since creating a template can generate advance todo items
            queryClient.invalidateQueries({
                queryKey: ['todo', variables.todo_name, 'item'],
            })
        },
    })
}

export const useUpdateRecurringTemplate = () => {
    const queryClient = useQueryClient()

    return useMutation({
        mutationFn: ({
            todo_name,
            template_id,
            template,
        }: {
            todo_name: string
            template_id: string
            template: UpdateRecurringTemplateRequest
        }) =>
            FinalRecurringTemplateAPI.UpdateRecurringTemplate(
                todo_name,
                template_id,
                template
            ),
        onSuccess: (data, variables) => {
            queryClient.invalidateQueries({
                queryKey: ['recurring-templates', variables.todo_name],
            })
            queryClient.invalidateQueries({
                queryKey: [
                    'recurring-template',
                    variables.todo_name,
                    variables.template_id,
                ],
            })
            // Invalidate todo items since updating a template can generate new todo items
            queryClient.invalidateQueries({
                queryKey: ['todo', variables.todo_name, 'item'],
            })
        },
    })
}

export const useDeleteRecurringTemplate = () => {
    const queryClient = useQueryClient()

    return useMutation({
        mutationFn: ({
            todo_name,
            template_id,
        }: {
            todo_name: string
            template_id: string
        }) =>
            FinalRecurringTemplateAPI.DeleteRecurringTemplate(
                todo_name,
                template_id
            ),
        onSuccess: (data, variables) => {
            queryClient.invalidateQueries({
                queryKey: ['recurring-templates', variables.todo_name],
            })
            queryClient.removeQueries({
                queryKey: [
                    'recurring-template',
                    variables.todo_name,
                    variables.template_id,
                ],
            })
        },
    })
}

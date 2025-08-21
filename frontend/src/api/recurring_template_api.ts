import {
    CreateRecurringTemplateRequest,
    UpdateRecurringTemplateRequest,
    RecurringTemplateResponse,
    ListRecurringTemplatesResponse,
    BASE_URL,
} from '.'

export const BackendRecurringTemplateAPI = {
    CreateRecurringTemplate: async (
        todo_name: string,
        r: CreateRecurringTemplateRequest
    ): Promise<RecurringTemplateResponse> => {
        const url = `${BASE_URL}/todo/${todo_name}/recurring`
        const options: RequestInit = {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(r),
        }
        return await fetch(url, options).then((r) => r.json())
    },

    ListRecurringTemplates: async (
        todo_name: string
    ): Promise<ListRecurringTemplatesResponse> => {
        const url = `${BASE_URL}/todo/${todo_name}/recurring`
        const options: RequestInit = {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
            },
        }
        return await fetch(url, options).then((r) => r.json())
    },

    GetRecurringTemplate: async (
        todo_name: string,
        template_id: string
    ): Promise<RecurringTemplateResponse> => {
        const url = `${BASE_URL}/todo/${todo_name}/recurring/${template_id}`
        const options: RequestInit = {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
            },
        }
        return await fetch(url, options).then((r) => r.json())
    },

    UpdateRecurringTemplate: async (
        todo_name: string,
        template_id: string,
        r: UpdateRecurringTemplateRequest
    ): Promise<RecurringTemplateResponse> => {
        const url = `${BASE_URL}/todo/${todo_name}/recurring/${template_id}`
        const options: RequestInit = {
            method: 'PUT',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(r),
        }
        return await fetch(url, options).then((r) => r.json())
    },

    DeleteRecurringTemplate: async (
        todo_name: string,
        template_id: string
    ): Promise<void> => {
        const url = `${BASE_URL}/todo/${todo_name}/recurring/${template_id}`
        const options: RequestInit = {
            method: 'DELETE',
            headers: {
                'Content-Type': 'application/json',
            },
        }
        await fetch(url, options)
    },
}

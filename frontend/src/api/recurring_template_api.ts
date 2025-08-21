import {
    CreateRecurringTemplateRequest,
    RecurringTemplateResponse,
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
}

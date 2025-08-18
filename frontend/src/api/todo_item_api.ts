import {
    ListTodoItemResponse,
    BASE_URL,
    CreateTodoItemRequest,
    CreateTodoItemResponse,
    GetTodoItemResponse,
} from '.'

export const BackendTodoItemAPI = {
    CreateTodoItem: async (
        todo_name: string,
        r: CreateTodoItemRequest
    ): Promise<CreateTodoItemResponse> => {
        const url = `${BASE_URL}/todo/${todo_name}/item`
        const options: RequestInit = {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(r),
        }
        return await fetch(url, options).then((r) => r.json())
    },
    ListTodoItem: async (todo_name: string): Promise<ListTodoItemResponse> => {
        const url = `${BASE_URL}/todo/${todo_name}/item`
        const options: RequestInit = {
            method: 'GET',
        }
        // TODO Handle failure
        return await fetch(url, options).then((r) => r.json())
    },
    CompleteTodoItem: async (
        todo_name: string,
        item_id: string
    ): Promise<GetTodoItemResponse> => {
        const url = `${BASE_URL}/todo/${todo_name}/item/${item_id}/complete`
        const options: RequestInit = {
            method: 'POST',
        }
        return await fetch(url, options).then((r) => r.json())
    },
}

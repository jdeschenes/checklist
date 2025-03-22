import {
    CreateTodoRequest,
    CreateTodoResponse,
    GetTodoResponse,
    DeleteTodoResponse,
    UpdateTodoRequest,
    UpdateTodoResponse,
    ListTodoResponse,
    BASE_URL,
} from '.'

export const BackendTodoAPI = {
    CreateTodo: async (r: CreateTodoRequest): Promise<CreateTodoResponse> => {
        const url = `${BASE_URL}/todo`
        const options: RequestInit = {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(r),
        }
        // TODO Handle failure
        await fetch(url, options)
    },
    GetTodo: async (todo_name: string): Promise<GetTodoResponse> => {
        const url = `${BASE_URL}/todo/${todo_name}`
        const options: RequestInit = {
            method: 'GET',
        }
        return await fetch(url, options).then((r) => r.json())
    },
    DeleteTodo: async (todo_name: string): Promise<DeleteTodoResponse> => {
        const url = `${BASE_URL}/todo/${todo_name}`
        const options: RequestInit = {
            method: 'DELETE',
        }
        // TODO Handle failure
        return await fetch(url, options).then((r) => r.json())
    },
    UpdateTodo: async (
        todo_name: string,
        r: UpdateTodoRequest
    ): Promise<UpdateTodoResponse> => {
        const url = `${BASE_URL}/todo/${todo_name}`
        const options: RequestInit = {
            method: 'PUT',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify(r),
        }
        // TODO Handle failure
        await fetch(url, options).then((r) => r.json())
    },
    ListTodo: async (): Promise<ListTodoResponse> => {
        const url = `${BASE_URL}/todo`
        const options: RequestInit = {
            method: 'GET',
        }
        // TODO Handle failure
        return await fetch(url, options).then((r) => r.json())
    },
}

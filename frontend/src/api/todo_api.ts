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
        let url = `${BASE_URL}/todo`
        let options: RequestInit = {
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
        return {
            name: todo_name,
            create_time: '123',
            update_time: '456',
        }
    },
    DeleteTodo: async (todo_name: string): Promise<DeleteTodoResponse> => {
        let url = `${BASE_URL}/todo/${todo_name}`
        let options: RequestInit = {
            method: 'DELETE',
        }
        // TODO Handle failure
        return await fetch(url, options).then((r) => r.json())
    },
    UpdateTodo: async (
        todo_name: string,
        r: UpdateTodoRequest
    ): Promise<UpdateTodoResponse> => {
        let url = `${BASE_URL}/todo/${todo_name}`
        let options: RequestInit = {
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
        let url = `${BASE_URL}/todo`
        let options: RequestInit = {
            method: 'GET',
        }
        // TODO Handle failure
        return await fetch(url, options).then((r) => r.json())
    },
}

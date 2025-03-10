import { ListTodoItemResponse, BASE_URL } from '.'

export const BackendTodoItemAPI = {
    ListTodoItem: async (todo_name: string): Promise<ListTodoItemResponse> => {
        let url = `${BASE_URL}/todo/${todo_name}/item`
        let options: RequestInit = {
            method: 'GET',
        }
        // TODO Handle failure
        return await fetch(url, options).then((r) => r.json())
    },
}

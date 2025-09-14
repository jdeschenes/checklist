import {
    ListTodoItemResponse,
    CreateTodoItemRequest,
    CreateTodoItemResponse,
    GetTodoItemResponse,
} from '.'
import { 
    authenticatedPost, 
    authenticatedGet 
} from './authenticated-client'

export const BackendTodoItemAPI = {
    CreateTodoItem: async (
        todo_name: string,
        r: CreateTodoItemRequest
    ): Promise<CreateTodoItemResponse> => {
        return await authenticatedPost<CreateTodoItemResponse>(`/todo/${todo_name}/item`, r)
    },
    ListTodoItem: async (todo_name: string): Promise<ListTodoItemResponse> => {
        return await authenticatedGet<ListTodoItemResponse>(`/todo/${todo_name}/item`)
    },
    CompleteTodoItem: async (
        todo_name: string,
        item_id: string
    ): Promise<GetTodoItemResponse> => {
        return await authenticatedPost<GetTodoItemResponse>(`/todo/${todo_name}/item/${item_id}/complete`)
    },
}

import { BackendTodoAPI } from './todo_api'
import { BackendTodoItemAPI } from './todo_item_api'

export const BASE_URL = 'http://localhost:3000'
export type CreateTodoRequest = {
    name: string
}

export type CreateTodoResponse = void

export type ListTodoSingleItem = {
    name: string
    create_time: string
    update_time: string
}

export type ListTodoResponse = {
    items: ListTodoSingleItem[]
}

export type GetTodoResponse = {
    name: string
    create_time: string
    update_time: string
}

export type UpdateTodoRequest = {
    name: string
}

export type UpdateTodoResponse = void
export type DeleteTodoResponse = void

interface TodoAPI {
    CreateTodo(r: CreateTodoRequest): Promise<CreateTodoResponse>
    GetTodo(todo_name: string): Promise<GetTodoResponse>
    DeleteTodo(todo_name: string): Promise<DeleteTodoResponse>
    UpdateTodo(
        todo_name: string,
        r: UpdateTodoRequest
    ): Promise<UpdateTodoResponse>
    ListTodo(): Promise<ListTodoResponse>
}

export type CreateTodoItemRequest = {
    title: string
}

export type CreateTodoItemResponse = {
    todo_item_id: string
    title: string
    is_complete: string
}

type ListTodoItemSingle = {
    todo_item_id: string
    title: string
    due_date: string
    is_complete: boolean
    complete_time: string
    create_time: string
    update_time: string
}

export type ListTodoItemResponse = {
    items: ListTodoItemSingle[]
}

interface TodoItemAPI {
    CreateTodoItem(
        todo_name: string,
        r: CreateTodoItemRequest
    ): Promise<CreateTodoItemResponse>
    ListTodoItem(todo_name: string): Promise<ListTodoItemResponse>
}

export const FinalTodoAPI: TodoAPI = BackendTodoAPI
export const FinalTodoItemAPI: TodoItemAPI = BackendTodoItemAPI

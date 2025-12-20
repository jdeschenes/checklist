import { BackendTodoAPI } from './todo_api'
import { BackendTodoItemAPI } from './todo_item_api'
import { BackendRecurringTemplateAPI } from './recurring_template_api'

const DEFAULT_BASE_URL = 'http://localhost:3000'
export const BASE_URL = import.meta.env.VITE_API_BASE_URL ?? DEFAULT_BASE_URL
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
    due_date?: string
}

export type CreateTodoItemResponse = {
    todo_item_id: string
    title: string
    is_complete: string
}

export type GetTodoItemResponse = {
    todo_item_id: string
    title: string
    due_date: string
    is_complete: boolean
    complete_time: string | null
    create_time: string
    update_time: string
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
    CompleteTodoItem(
        todo_name: string,
        item_id: string
    ): Promise<GetTodoItemResponse>
}

export type RecurrenceInterval = {
    months?: number
    days?: number
    microseconds?: number
}

export type CreateRecurringTemplateRequest = {
    title: string
    recurrence_interval: RecurrenceInterval
    start_date?: string
    end_date?: string
}

export type UpdateRecurringTemplateRequest = {
    title: string
    recurrence_interval: RecurrenceInterval
    start_date?: string
    end_date?: string
    is_active: boolean
}

export type RecurringTemplateResponse = {
    todo_name: string
    template_id: string
    title: string
    recurrence_interval: RecurrenceInterval
    start_date: string
    end_date: string | null
    last_generated_date: string | null
    is_active: boolean
    create_time: string
    update_time: string
}

export type ListRecurringTemplatesResponse = {
    templates: RecurringTemplateResponse[]
}

interface RecurringTemplateAPI {
    CreateRecurringTemplate(
        todo_name: string,
        r: CreateRecurringTemplateRequest
    ): Promise<RecurringTemplateResponse>
    ListRecurringTemplates(
        todo_name: string
    ): Promise<ListRecurringTemplatesResponse>
    GetRecurringTemplate(
        todo_name: string,
        template_id: string
    ): Promise<RecurringTemplateResponse>
    UpdateRecurringTemplate(
        todo_name: string,
        template_id: string,
        r: UpdateRecurringTemplateRequest
    ): Promise<RecurringTemplateResponse>
    DeleteRecurringTemplate(
        todo_name: string,
        template_id: string
    ): Promise<void>
}

export const FinalTodoAPI: TodoAPI = BackendTodoAPI
export const FinalTodoItemAPI: TodoItemAPI = BackendTodoItemAPI
export const FinalRecurringTemplateAPI: RecurringTemplateAPI =
    BackendRecurringTemplateAPI

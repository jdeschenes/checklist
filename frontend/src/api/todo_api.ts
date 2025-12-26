import {
  CreateTodoRequest,
  CreateTodoResponse,
  GetTodoResponse,
  DeleteTodoResponse,
  UpdateTodoRequest,
  UpdateTodoResponse,
  ListTodoResponse,
} from ".";
import {
  authenticatedGet,
  authenticatedPut,
  authenticatedDelete,
  authenticatedFetch,
} from "./authenticated-client";

export const BackendTodoAPI = {
  CreateTodo: async (r: CreateTodoRequest): Promise<CreateTodoResponse> => {
    console.log("CreateTodo API call - Request:", r);
    console.log("Making POST request to /todo");

    const response = await authenticatedFetch("/todo", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(r),
    });

    console.log("CreateTodo API response status:", response.status);
    console.log("CreateTodo API response:", response);

    if (!response.ok) {
      const errorText = await response.text();
      console.error("CreateTodo API error response:", errorText);
      throw new Error(
        `HTTP error! status: ${response.status}, body: ${errorText}`
      );
    }

    // Backend returns empty response for create todo
    return undefined as CreateTodoResponse;
  },
  GetTodo: async (todo_name: string): Promise<GetTodoResponse> => {
    return await authenticatedGet<GetTodoResponse>(`/todo/${todo_name}`);
  },
  DeleteTodo: async (todo_name: string): Promise<DeleteTodoResponse> => {
    await authenticatedDelete(`/todo/${todo_name}`);
  },
  UpdateTodo: async (
    todo_name: string,
    r: UpdateTodoRequest
  ): Promise<UpdateTodoResponse> => {
    await authenticatedPut(`/todo/${todo_name}`, r);
  },
  ListTodo: async (): Promise<ListTodoResponse> => {
    return await authenticatedGet<ListTodoResponse>("/todo");
  },
};

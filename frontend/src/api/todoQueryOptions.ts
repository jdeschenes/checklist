import { queryOptions } from "@tanstack/react-query";
import { FinalTodoAPI } from ".";

export const listTodoQueryOptions = queryOptions({
  queryKey: ["todo"],
  queryFn: () => FinalTodoAPI.ListTodo(),
});

export const getTodoQueryOptions = (todo_name: string) =>
  queryOptions({
    queryKey: ["todo", todo_name],
    queryFn: () => FinalTodoAPI.GetTodo(todo_name),
  });

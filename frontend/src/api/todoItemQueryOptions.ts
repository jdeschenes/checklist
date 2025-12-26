import { queryOptions } from "@tanstack/react-query";
import { FinalTodoItemAPI } from ".";

export const listTodoItemsQueryOptions = (todo_name: string) => {
  return queryOptions({
    queryKey: ["todo", todo_name, "item"],
    queryFn: () => FinalTodoItemAPI.ListTodoItem(todo_name),
  });
};

import { useMutation, useQueryClient } from "@tanstack/react-query";
import { FinalTodoItemAPI } from ".";
import {
  todayItemsQueryOptions,
  listTodoItemsQueryOptions,
} from "./todoItemQueryOptions";

export default function useCompleteTodayItem() {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: ({ todoName, itemId }: { todoName: string; itemId: string }) =>
      FinalTodoItemAPI.CompleteTodoItem(todoName, itemId),
    onSuccess: (_data, { todoName }) => {
      void queryClient.invalidateQueries({
        queryKey: todayItemsQueryOptions.queryKey,
      });
      void queryClient.invalidateQueries({
        queryKey: listTodoItemsQueryOptions(todoName).queryKey,
      });
    },
  });
}

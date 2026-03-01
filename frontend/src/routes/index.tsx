import { createFileRoute } from "@tanstack/react-router";
import { useSuspenseQuery } from "@tanstack/react-query";
import { todayItemsQueryOptions } from "@/api/todoItemQueryOptions";
import useCompleteTodayItem from "@/api/useCompleteTodayItem";
import { CheckCircle2, Circle } from "lucide-react";
import * as React from "react";

const COMPLETION_DELAY_MS = 3000;

export const Route = createFileRoute("/")({
  component: Today,
  loader: ({ context: { queryClient } }) =>
    queryClient.ensureQueryData(todayItemsQueryOptions),
});

function Today() {
  const todayItemsQuery = useSuspenseQuery(todayItemsQueryOptions);
  const items = todayItemsQuery.data.items;

  const completeTodayItemMutation = useCompleteTodayItem();
  const [pendingCompletions, setPendingCompletions] = React.useState<
    Set<string>
  >(new Set());
  const completionTimeoutRef = React.useRef<ReturnType<
    typeof setTimeout
  > | null>(null);
  const pendingCompletionsRef = React.useRef(pendingCompletions);
  const pendingTodoNamesRef = React.useRef<Map<string, string>>(new Map());
  const inFlightCompletionsRef = React.useRef<Set<string>>(new Set());

  React.useEffect(() => {
    pendingCompletionsRef.current = pendingCompletions;
  }, [pendingCompletions]);

  const clearCompletionTimeout = React.useCallback(() => {
    if (completionTimeoutRef.current) {
      clearTimeout(completionTimeoutRef.current);
      completionTimeoutRef.current = null;
    }
  }, []);

  const resetCompletionWindow = React.useCallback(() => {
    clearCompletionTimeout();

    const pending = Array.from(pendingCompletionsRef.current).filter(
      (itemId) => !inFlightCompletionsRef.current.has(itemId)
    );

    if (pending.length === 0) {
      return;
    }

    completionTimeoutRef.current = setTimeout(() => {
      const itemsToComplete = Array.from(pendingCompletionsRef.current).filter(
        (itemId) => !inFlightCompletionsRef.current.has(itemId)
      );

      if (itemsToComplete.length === 0) {
        return;
      }

      itemsToComplete.forEach((itemId) => {
        inFlightCompletionsRef.current.add(itemId);
        completeTodayItemMutation.mutate(
          { todoName: pendingTodoNamesRef.current.get(itemId)!, itemId },
          {
            onSettled: () => {
              inFlightCompletionsRef.current.delete(itemId);
              setPendingCompletions((prev) => {
                if (!prev.has(itemId)) {
                  return prev;
                }
                const newSet = new Set(prev);
                newSet.delete(itemId);
                return newSet;
              });
            },
          }
        );
      });
    }, COMPLETION_DELAY_MS);
  }, [clearCompletionTimeout, completeTodayItemMutation]);

  React.useEffect(() => {
    resetCompletionWindow();
  }, [pendingCompletions, resetCompletionWindow]);

  const handleItemClick = React.useCallback(
    (itemId: string, todoName: string, isComplete: boolean) => {
      if (isComplete) return;

      setPendingCompletions((prev) => {
        const newSet = new Set(prev);

        if (newSet.has(itemId)) {
          newSet.delete(itemId);
          pendingTodoNamesRef.current.delete(itemId);
        } else {
          newSet.add(itemId);
          pendingTodoNamesRef.current.set(itemId, todoName);
        }

        return newSet;
      });
    },
    []
  );

  React.useEffect(() => {
    return () => {
      clearCompletionTimeout();
    };
  }, [clearCompletionTimeout]);

  return (
    <div className="p-4 sm:p-6 pb-24">
      <div className="mx-auto flex w-full max-w-3xl flex-col gap-6">
        {items.length === 0 ? (
          <div className="flex flex-col items-center justify-center py-12 sm:py-16 px-4 bg-gray-50 rounded-lg border border-dashed border-gray-200">
            <div className="text-lg sm:text-2xl font-medium text-gray-500 mb-2">
              All done
            </div>
            <p className="text-gray-400 text-sm text-center">
              Nothing due today
            </p>
          </div>
        ) : (
          <ul className="space-y-2">
            {items.map((item) => {
              const isPending = pendingCompletions.has(item.todo_item_id);
              const isClickable = !item.is_complete;

              return (
                <li
                  key={item.todo_item_id}
                  onClick={() =>
                    handleItemClick(
                      item.todo_item_id,
                      item.todo_name,
                      item.is_complete
                    )
                  }
                  className={`group flex items-start sm:items-center gap-3 p-3 sm:p-4 bg-white rounded-lg border transition-all ${
                    isClickable
                      ? "cursor-pointer hover:shadow-sm"
                      : "border-gray-100"
                  } ${
                    isPending
                      ? "border-green-200 bg-green-50 hover:border-green-300"
                      : isClickable
                        ? "border-gray-100 hover:border-gray-200"
                        : "border-gray-100"
                  }`}
                >
                  <div className="flex-shrink-0 mt-0.5 sm:mt-0">
                    {item.is_complete ? (
                      <CheckCircle2 className="h-5 w-5 text-green-500" />
                    ) : isPending ? (
                      <CheckCircle2 className="h-5 w-5 text-green-500 animate-pulse" />
                    ) : (
                      <Circle className="h-5 w-5 text-gray-300 group-hover:text-gray-400" />
                    )}
                  </div>
                  <div className="flex-grow min-w-0">
                    <div className="flex items-baseline justify-between gap-2">
                      <div
                        className={`text-sm sm:text-base font-medium leading-relaxed ${item.is_complete ? "text-gray-400 line-through" : "text-gray-900"}`}
                      >
                        {item.title}
                      </div>
                      <span className="text-sm text-gray-500 font-medium shrink-0">
                        {item.todo_name}
                      </span>
                    </div>
                  </div>
                </li>
              );
            })}
          </ul>
        )}
      </div>
    </div>
  );
}

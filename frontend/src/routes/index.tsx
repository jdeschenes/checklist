import { createFileRoute, Link } from "@tanstack/react-router";
import { useSuspenseQuery } from "@tanstack/react-query";
import { listTodoQueryOptions } from "@/api/todoQueryOptions";
import { buttonVariants } from "@/components/ui/button";
import { ChevronRight, Plus } from "lucide-react";

export const Route = createFileRoute("/")({
  component: Index,
  loader: ({ context: { queryClient } }) =>
    queryClient.ensureQueryData(listTodoQueryOptions),
});

function Index() {
  const listTodoQuery = useSuspenseQuery(listTodoQueryOptions);
  const todos = listTodoQuery.data;
  return (
    <div className="p-4 sm:p-6">
      <div className="mx-auto flex w-full max-w-3xl flex-col gap-6">
        <div className="space-y-1">
          <div className="flex flex-wrap items-center justify-between gap-3">
            <h1 className="text-2xl font-semibold text-gray-900">Todo Lists</h1>
            <Link
              className={buttonVariants({ variant: "default" })}
              to="/todo/new"
            >
              <Plus className="h-4 w-4" /> New
            </Link>
          </div>
        </div>
        {todos.items.length === 0 ? (
          <div className="rounded-lg border border-dashed border-gray-200 bg-gray-50 px-6 py-12 text-center">
            <div className="text-xl font-semibold text-gray-700">
              No lists yet
            </div>
            <p className="mt-2 text-sm text-gray-500">
              Create your first todo list to get started.
            </p>
          </div>
        ) : (
          <ul className="space-y-2">
            {todos.items.map((i) => {
              const isPrivate = i.visibility === "private";

              return (
                <li key={i.name}>
                  <Link
                    className="group flex items-center justify-between gap-4 rounded-lg border border-gray-200 bg-white px-4 py-3 shadow-sm transition hover:border-gray-300 hover:shadow-md"
                    to="/todo/$todoId"
                    params={{ todoId: i.name }}
                  >
                    <div className="min-w-0">
                      <div className="flex items-center gap-2">
                        <span className="truncate text-sm font-semibold text-gray-900">
                          {i.name}
                        </span>
                        <span
                          className={`rounded-full px-2 py-0.5 text-[11px] font-medium ${
                            isPrivate
                              ? "bg-gray-900 text-white"
                              : "bg-gray-100 text-gray-700"
                          }`}
                        >
                          {isPrivate ? "Private" : "Public"}
                        </span>
                      </div>
                    </div>
                    <ChevronRight className="h-4 w-4 text-gray-400 transition group-hover:text-gray-500" />
                  </Link>
                </li>
              );
            })}
          </ul>
        )}
      </div>
    </div>
  );
}

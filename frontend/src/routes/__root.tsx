import { QueryClient } from "@tanstack/react-query";
import {
  Outlet,
  createRootRouteWithContext,
  useLocation,
} from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/router-devtools";
import { AppBreadcrumb } from "@/components/AppBreadcrumb";
import { useAuth } from "@/contexts/AuthContext";
import { AuthGuard } from "@/components/AuthGuard";
import { BottomNav } from "@/components/BottomNav";

type RouterContext = {
  queryClient: QueryClient;
};

function RootComponent() {
  const { isAuthenticated } = useAuth();
  const location = useLocation();

  // Routes that don't require authentication
  const isPublicRoute =
    location.pathname === "/login" ||
    location.pathname.startsWith("/login/") ||
    location.pathname.startsWith("/auth/callback");

  const content = (
    <>
      {isAuthenticated && (
        <header className="bg-white border-b border-gray-200 px-4 py-2 flex items-center">
          <AppBreadcrumb />
        </header>
      )}
      <main className="pb-16">
        <Outlet />
      </main>
      {isAuthenticated && <BottomNav />}
      {import.meta.env.DEV ? (
        <TanStackRouterDevtools position="top-right" />
      ) : null}
    </>
  );

  // If it's a public route, render without auth guard
  if (isPublicRoute) {
    return content;
  }

  // Otherwise, wrap with auth guard
  return <AuthGuard>{content}</AuthGuard>;
}

export const Route = createRootRouteWithContext<RouterContext>()({
  component: RootComponent,
});

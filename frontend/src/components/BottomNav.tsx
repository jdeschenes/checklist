import { Link, useRouterState } from "@tanstack/react-router";
import { Home, ListTodo, LogOut } from "lucide-react";
import { useAuth } from "@/contexts/AuthContext";
import { useNavigate } from "@tanstack/react-router";
import { cn } from "@/lib/utils";

export function BottomNav() {
  const { logout } = useAuth();
  const navigate = useNavigate();
  const pathname = useRouterState({ select: (s) => s.location.pathname });

  const handleLogout = () => {
    logout();
    void navigate({ to: "/login" });
  };

  const isHome = pathname === "/";
  const isTodos = pathname.startsWith("/todos") || pathname.startsWith("/todo");

  const itemClass = (active: boolean) =>
    cn(
      "flex flex-col items-center justify-center h-full w-full transition-colors",
      active ? "text-gray-900" : "text-gray-400 hover:text-gray-600"
    );

  return (
    <nav className="fixed bottom-0 left-0 right-0 z-50 border-t border-gray-200 bg-white">
      <div className="grid h-14 grid-cols-3">
        <Link to="/" className={itemClass(isHome)}>
          <Home className={cn("h-5 w-5", isHome && "stroke-[2.5]")} />
        </Link>
        <Link to="/todos" className={itemClass(isTodos)}>
          <ListTodo className={cn("h-5 w-5", isTodos && "stroke-[2.5]")} />
        </Link>
        <button onClick={handleLogout} className={itemClass(false)}>
          <LogOut className="h-5 w-5" />
        </button>
      </div>
    </nav>
  );
}

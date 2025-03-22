import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/todo/$todoId/new')({
    component: RouteComponent,
})

function RouteComponent() {
    return <div>Hello /todo/$todoId/new!</div>
}

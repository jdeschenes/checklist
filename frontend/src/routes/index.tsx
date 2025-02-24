import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/')({
    component: Index,
})

function Index() {
    return (
        <div className="text-3xl font-bold underline">
            <h1>TODO NAME</h1>
        </div>
    )
}

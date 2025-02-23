import React from 'react'
import ReactDOM from 'react-dom/client'

function Foo() {
    const [x, setX] = React.useState(0)
    React.useEffect(() => {
        console.log('Potato', x)
    })
    return <div>{x}</div>
}

ReactDOM.createRoot(document.getElementById('root')).render(
    <React.StrictMode>
        <Foo />
    </React.StrictMode>
)

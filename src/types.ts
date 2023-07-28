export type TodoItem = {
    id: number,
    name: string,
    completed: boolean
}

export type ProjectType = {
    id: number,
    name: string,
    todos: TodoItem[]
}
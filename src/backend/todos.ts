
export async function getTodos(id: number) {
    const response = await fetch(`http://localhost:8000/todos/${id}`);
    return await response.json();
}
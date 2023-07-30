// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
export async function load({ fetch, params }) {
    const response = await fetch(`http://localhost:8000/todos/${params.id}`);
    const todos = await response.json();
    console.log(todos);
    return { todos };
}
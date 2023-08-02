<script>
    import {getTodos} from "../../../backend/todos";
    import {onMount} from "svelte";
    import {page} from '$app/stores';

    let todos = [];

    onMount(async () => {

        const id = parseInt($page.url.pathname.replace("/project/", ""));

        todos = await getTodos(id);
    });
</script>

<div>
    {#each todos as todo}
        <div class="flex flex-col border-2 border-gray-700 m-3 p-3">
            <div>id: {todo.id}</div>
            <div>description: {todo.description}</div>
            <div>done: {todo.done}</div>
        </div>
    {/each}
</div>



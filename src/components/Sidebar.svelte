<script lang="ts">

    let myProjects = [
        {
            id: 1,
            name: "My First Project",
            todos: [
                {
                    id: 1,
                    name: "My First Todo",
                    completed: true
                },
                {
                    id: 2,
                    name: "My Second Todo",
                    completed: false
                }
            ]
        },
        {
            id: 2,
            name: "My Second Project",
            todos: [
                {
                    id: 1,
                    name: "My Third Todo",
                    completed: true
                },
                {
                    id: 2,
                    name: "My Fourth Todo",
                    completed: false
                }
            ]
        }
    ];

    function openPage(page) {
        window.location.href = page;
    }

</script>

<div class="flex flex-col justify-between bg-gray-800 w-72 h-[100%] border-t-4 border-green-500">
    <div class="flex-row">
        <button class="p-5 border-b-2 hover:bg-white hover:bg-opacity-10 w-full text-start text-white font-bold" on:click={() => openPage("/")}>
            <i class="fa-solid fa-home mr-3"></i>Home
        </button>
        {#each myProjects as project}
            <button class="p-3 pl-5 pr-5 hover:bg-white hover:bg-opacity-10 w-full text-start text-white font-bold" on:click={() => openPage("/project/" + project.id)}>
                <i class="fa-solid fa-list mr-3"></i>{project.name}
            </button>
        {/each}
        <div class="pl-5 pt-5 pr-5 ">
            <button class="border-2 p-3 pl-5 rounded-md hover:bg-white hover:bg-opacity-10 w-full text-start text-white font-bold" on:click={() => openPage("/project/new")}>
                <i class="fa-solid fa-plus mr-3"></i>New Project
            </button>
        </div>
    </div>
    <button class="p-5 border-t-2 hover:bg-white hover:bg-opacity-10 w-full text-start text-white font-bold" on:click={() => openPage("/settings")}>
        <i class="fa-solid fa-gear mr-3"></i>Settings
    </button>
</div>
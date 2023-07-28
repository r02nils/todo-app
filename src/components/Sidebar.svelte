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
</script>

<div class="flex flex-col justify-between bg-gray-800 w-96 h-[100%] border-t-4 border-green-500">
    <div class="flex-row">
        <div class="p-5 border-b-2">
            <a class="text-white font-bold" href="/"><i class="fa-solid fa-house mr-3"></i>Home</a>
        </div>
        {#each myProjects as project}
            <div class="pl-5 pt-5 pr-5">
                <a class="text-white font-bold" href="/project/{project.id}"><i class="fa-solid fa-list mr-3"></i>{project.name}</a>
            </div>
        {/each}
    </div>
    <div class="p-5 border-t-2">
        <a class="text-white font-bold" href="/settings"><i class="fa-solid fa-gear mr-3"></i>Settings</a>
    </div>
</div>
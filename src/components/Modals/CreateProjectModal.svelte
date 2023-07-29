<script lang="ts">

    let name = '';

    function createNewProject() {

        fetch("http://localhost:8000/create", {
            method: "POST",
            headers: {'Content-Type': 'application/x-www-form-urlencoded;charset=UTF-8'},
            body: "description=" + name
        }).then((response) => {
            console.log(response);
            window.location.reload();
        })
    }

</script>

<div class="bg-gray-800 p-5 rounded-xl text-white w-96 border-2">
    <h1 class="font-bold text-xl">Create Project</h1>
    <form class="flex flex-col pt-5 gap-3">
        <input class="pl-1 rounded-md border-2 bg-transparent hover:bg-white hover:bg-opacity-10 " type="text"
               placeholder="Name" bind:value={name}>
        <button class="ml-auto pl-5 pr-5 text-white rounded-md font-bold border-2 hover:bg-white hover:bg-opacity-10"
                on:click={createNewProject}>Submit
        </button>
    </form>
</div>
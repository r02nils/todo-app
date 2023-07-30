<script lang="ts">

    import { modalStore } from '@skeletonlabs/skeleton';

    function removeProject() {

        fetch("http://localhost:8000/deleteProject/" + $modalStore[0].value, {
            method: "POST",
            headers: {'Content-Type': 'application/x-www-form-urlencoded;charset=UTF-8'}
        }).then((response) => {
            console.log(response);
            window.location.reload();
        })
    }

</script>

<div class="bg-gray-800 p-5 rounded-xl text-white w-96 border-2">
    <h1 class="font-bold text-xl">Remove Project</h1>
    <form class="flex flex-col pt-5 gap-3">
        <button class="ml-auto pl-5 pr-5 text-white rounded-md font-bold border-2 border-red-500 hover:bg-red-500 hover:bg-opacity-10"
                on:click={removeProject}>Remove
        </button>
    </form>
</div>
<script lang="ts">

    import {onMount} from 'svelte';
    import type {ModalComponent, ModalSettings} from "@skeletonlabs/skeleton";
    import {Modal, modalStore} from "@skeletonlabs/skeleton";
    import CreateProjectModal from "./CreateProjectModal.svelte";

    let projects = [];

    onMount(async () => {
        const response = await fetch('http://localhost:8000/');
        console.log(response);
        projects = await response.json();
    });

    function openPage(page) {
        window.location.href = page;
    }

    const modalComponentRegistry: Record<string, ModalComponent> = {

        // Custom Modal 1
        modalCreateProject: {
            // Pass a reference to your custom component
            ref: CreateProjectModal,
            // Add the component properties as key/value pairs
            props: { background: 'bg-red-500' },
            // Provide a template literal for the default component slot
            slot: '<p>Skeleton</p>'
        },
    };

    const modal: ModalSettings = {
        type: 'component',
        component: 'modalCreateProject',
    };

    function openCreateProjectModal() {
        console.log("test");
        modalStore.trigger(modal);
    }

</script>

<div class="flex flex-col justify-between bg-gray-800 w-72 h-[100%] border-t-4 border-green-500">
    <div class="flex-row">
        <button class="p-5 border-b-2 hover:bg-white hover:bg-opacity-10 w-full text-start text-white font-bold"
                on:click={() => openPage("/")}>
            <i class="fa-solid fa-home mr-3"></i>Home
        </button>
        {#each projects as project}
            <button class="p-3 pl-5 pr-5 hover:bg-white hover:bg-opacity-10 w-full text-start text-white font-bold"
                    on:click={() => openPage("/project/" + project.id)}>
                <i class="fa-solid fa-list mr-3"></i>{project.description}
            </button>
        {/each}
        <div class="pl-5 pt-5 pr-5 ">
            <button class="border-2 p-3 pl-5 rounded-md hover:bg-white hover:bg-opacity-10 w-full text-start text-white font-bold"
                    on:click={() => openCreateProjectModal()}>
                <i class="fa-solid fa-plus mr-3"></i>New Project
            </button>
        </div>
    </div>
    <button class="p-5 border-t-2 hover:bg-white hover:bg-opacity-10 w-full text-start text-white font-bold"
            on:click={() => openPage("/settings")}>
        <i class="fa-solid fa-gear mr-3"></i>Settings
    </button>
</div>

<Modal components={modalComponentRegistry} />
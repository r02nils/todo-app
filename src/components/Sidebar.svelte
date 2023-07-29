<script lang="ts">

    import {onMount} from 'svelte';
    import type {ModalComponent, ModalSettings} from "@skeletonlabs/skeleton";
    import {Modal, modalStore} from "@skeletonlabs/skeleton";
    import CreateProjectModal from "./Modals/CreateProjectModal.svelte";
    import RemoveProjectModal from "./Modals/RemoveProjectModal.svelte";

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

        modalCreateProject: {
            ref: CreateProjectModal,
        },

        modalRemoveProject: {
            ref: RemoveProjectModal,
        },
    };

    const createProjectModal: ModalSettings = {
        type: 'component',
        component: 'modalCreateProject',
    };

    const removeProjectModal: ModalSettings = {
        type: 'component',
        component: 'modalRemoveProject',
        value: 0
    };

    function openCreateProjectModal() {
        modalStore.trigger(createProjectModal);
    }

    function openRemoveProjectModal(id) {
        removeProjectModal.value = id;
        modalStore.trigger(removeProjectModal);
    }

</script>

<div class="flex flex-col justify-between bg-gray-800 w-72 h-[100%] border-t-4 border-green-500">
    <div class="flex-row">
        <button class="p-5 border-b-2 hover:bg-white hover:bg-opacity-10 w-full text-start text-white font-bold"
                on:click={() => openPage("/")}>
            <i class="fa-solid fa-home mr-3"></i>Home
        </button>
        {#if projects.length < 1}
            {#each {length: 3} as _, i}
                <div class="grid grid-cols-1 gap-2 p-3 pl-5 pr-5">
                    <div class="placeholder animate-pulse bg-gray-700 rounded-xl h-6"></div>
                </div>
            {/each}
        {:else}
            {#each projects as project}
                <div class="flex flex-row">
                    <button class="p-3 pl-5 pr-5 hover:bg-white hover:bg-opacity-10 w-full text-start text-white font-bold"
                            on:click={() => openPage("/project/" + project.id)}>
                        <i class="fa-solid fa-list mr-3"></i><span class="placeholder">{project.description}</span>
                    </button>
                    <button class="w-1/5 hover:bg-red-500 hover:bg-opacity-10"
                            on:click={() => openRemoveProjectModal(project.id)}>
                        <i class="fa-solid fa-xmark text-red-500"></i>
                    </button>
                </div>
            {/each}
        {/if}
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

<Modal components={modalComponentRegistry}/>
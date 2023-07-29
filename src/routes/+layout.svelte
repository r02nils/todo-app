<script>
    import '../app.css';
    import '@skeletonlabs/skeleton/styles/skeleton.css';

    import Sidebar from "../components/Sidebar.svelte";
    import Header from "../components/Header.svelte";

    import { AppShell } from '@skeletonlabs/skeleton';
</script>

<AppShell>
    <svelte:fragment slot="header">
        <Header />
    </svelte:fragment>
    <svelte:fragment slot="sidebarLeft">
        <Sidebar />
    </svelte:fragment>
    <slot />
</AppShell>
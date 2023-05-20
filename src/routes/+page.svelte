<script>
    import { invoke } from "@tauri-apps/api/tauri"
    import { onMount } from "svelte";

    import Navbar from "../components/Navbar.svelte";
    import HostsTable from "../components/HostsTable.svelte";
    import AppUsage from "../components/AppUsage.svelte";
    import ProtocolUsage from "../components/ProtocolUsage.svelte";
    import TotalUsage from "../components/TotalUsage.svelte";

    import "chart.js/auto";

    onMount(() => {
        invoke("init_socket_handler");
    });

</script>

<div class="container">
    <Navbar />
    <div class="tile is-ancestor is-vertical">
        <div class="container pl-6 py-2">
            <h3 class="title is-4">
                Usage graphs
            </h3>
        </div>
        <div class="tile is-parent">
            <div class="tile is-parent box notification is-3 is-flex">
                <TotalUsage />
            </div>

            <div class="tile is-parent box is-5 ml-4 mx-6">
                <AppUsage />
            </div>
            <div class="tile is-parent box is-3" style="height: 252px">
                <ProtocolUsage />
            </div>
        </div>

        <div class="container pl-6 py-2">
            <h3 class="title is-4">
                Active Connections 
            </h3>
        </div>
        <div class="tile box is-child">
            <HostsTable />
        </div>
    </div>
</div>

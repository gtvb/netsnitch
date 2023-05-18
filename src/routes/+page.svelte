<script>
    import { invoke } from "@tauri-apps/api/tauri"
    import { onMount } from "svelte";
    import { hostsDataStore } from "../utils/store";

    import HostsTable from "../components/HostsTable.svelte";
    import AppUsage from "../components/AppUsage.svelte";
    import ProtocolUsage from "../components/ProtocolUsage.svelte";

    import "chart.js/auto";

    onMount(() => {
        invoke("init_socket_handler");
    });
</script>

<div class="section">
    <div class="tile is-vertical is-ancestor">
        <h2 class="title">
            Network Usage
        </h2>
        <div class="tile is-parent">
            <div class="tile is-child is-4">
            </div>

            <div class="tile is-child is-4">
                <AppUsage />
            </div>

            <div class="tile is-child is-4">
                <ProtocolUsage />
            </div>
        </div>

        <h2 class="title">
            Connections
        </h2>
        <div class="tile is-child">
            <HostsTable />
        </div>
    </div>
</div>

<script>
    import { invoke } from "@tauri-apps/api/tauri"
    import { onMount } from "svelte";

    import HostsTable from "../components/HostsTable.svelte";
    import AppUsage from "../components/AppUsage.svelte";
    import ProtocolUsage from "../components/ProtocolUsage.svelte";

    import "chart.js/auto";

    onMount(() => {
        invoke("init_socket_handler");
    });
</script>

<section class="section">
    <div class="tile is-ancestor is-vertical">
      <div class="tile">
        <div class="tile">
            <AppUsage />
        </div>
        <div class="tile">
            <ProtocolUsage />
        </div>
      </div>
      <div class="tile is-parent">
        <article class="tile is-child">
            <HostsTable />
        </article>
      </div>
    </div>
</section>

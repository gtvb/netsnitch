<script>
    import { listen } from "@tauri-apps/api/event"
    import { onMount } from "svelte";
    import { convertToBytes } from "../utils/units";
    import { settingsStore } from "../utils/settingsStore";
    import { convertJsonToArray } from "../utils/json";

    let fetchedData;
    let hostsData;

    onMount(async () => {
        async function fetchHostsData() {
            const unlisten = await listen('ip-event', (data) => {
                fetchedData = JSON.parse(data.payload);
            })

            return unlisten;
        }

        const cleanup = await fetchHostsData();
        const interval = setInterval(() => {
            const data = convertJsonToArray(fetchedData);
            data.sort((a, b) => {
                const aTotal = convertToBytes(a.total);
                const bTotal = convertToBytes(b.total);

                return bTotal - aTotal;
            });

            hostsData = data;
        }, $settingsStore.delay);

        return () => {
            cleanup();
            clearInterval(interval);
        }
    });
</script>

<div style="overflow-y: scroll; height: 230px">
{#if hostsData}
    <table class="table is-bordered is-striped is-narrow is-hoverable is-fullwidth">
        <thead>
            <tr>
                <th scope="col">Host IP</th>
                <th scope="col">Download</th>
                <th scope="col">Upload</th>
                <th scope="col">Total</th>
            </tr>
        </thead>
        <tbody>
            {#each hostsData as { host, download, upload, total }}
                <tr>
                    <td>{host}</td>
                    <td>{download}</td>
                    <td>{upload}</td>
                    <td>{total}</td>
                </tr>
            {/each}
        </tbody>
    </table>
{:else}
    <div>
        Loading...
    </div>
{/if}
</div>

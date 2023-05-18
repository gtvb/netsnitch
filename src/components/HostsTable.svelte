<script>
    import { listen } from "@tauri-apps/api/event"
    import { onMount } from "svelte";
    import { convertToBytes, formatByteValue } from "../utils/units";

    let fetchedData;
    let hostsData;

    onMount(async () => {
        async function fetchAppUsageData() {
            const unlisten = await listen('ip-event', (data) => {
                fetchedData = JSON.parse(data.payload);
            })

            return unlisten;
        }

        const cleanup = await fetchAppUsageData();
        const interval = setInterval(() => {
            const jsonArray = Object.entries(fetchedData).map(([key, val]) => {
                return {
                    ...val,
                    key
                }
            });

            jsonArray.sort((a, b) => {
                const aTotal = convertToBytes(a.total);
                const bTotal = convertToBytes(b.total);

                return bTotal - aTotal;
            });

            hostsData = jsonArray;
        }, 3000);

        return () => {
            cleanup();
            clearInterval(interval);
        }
    });
</script>

<div style="overflow-y: scroll; height: 300px">
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

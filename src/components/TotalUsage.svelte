<script>
    import { onMount } from "svelte";
    import { hostsDataStore } from "../utils/hostsStore";
    import { settingsStore } from "../utils/settingsStore";
    import { convertToBytes, formatByteValue } from "../utils/units";
    import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/api/notification';

    let totalNetworkUsage;

    $: {
        let usage = 0;
        for(const host of $hostsDataStore) {
            usage += convertToBytes(host.total); 
        }

        totalNetworkUsage = formatByteValue(usage);
    }

    $: totalUsageBytes = (parseFloat(totalNetworkUsage.value).toFixed(1))
    $: totalUsageLimitBytes = (parseFloat($settingsStore.usage).toFixed(1))
    $: hasMoreUsage = (totalUsageBytes > totalUsageLimitBytes);

    onMount(async () => {
        let permissionGranted = await isPermissionGranted();

        if (!permissionGranted) {
            const permission = await requestPermission();
            permissionGranted = permission === 'granted';
        }

        if (permissionGranted && hasMoreUsage) {
            sendNotification({ title: 'Warning!', body: 'You went above your usage limit!' });
        }
    });
</script>

<div class="level">
    <div class="level-item has-text-centered">
        <div class="mr-6 px-3">
            <h3 class="title is-centered is-bold">Total Usage</h3>
            {#if totalNetworkUsage && $settingsStore.usage && $settingsStore.unit}
                <h1 class="title is-4 is-centered">
                    {parseFloat(totalNetworkUsage.value).toFixed(2)} {totalNetworkUsage.unit}B
                    / {parseFloat($settingsStore.usage).toFixed(2)} {$settingsStore.unit}
                </h1>
            {:else}
                <h1 class="is-centered">
                    {parseFloat(totalNetworkUsage.value).toFixed(2)} {totalNetworkUsage.unit}B
                    / No usage limit
                </h1>
            {/if}
        </div>
    </div>
</div>

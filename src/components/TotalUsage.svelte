<script>
    import { onMount, afterUpdate } from "svelte";
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

    let totalUsageLimitBytes;
    $: totalUsageBytes = (totalNetworkUsage.value+totalNetworkUsage.unit+"B")
    $: if($settingsStore.usage) {
        totalUsageLimitBytes = ($settingsStore.usage+$settingsStore.unit)
    }

    let permissionGranted;
    onMount(async () => {
        permissionGranted = await isPermissionGranted();

        if (!permissionGranted) {
            const permission = await requestPermission();
            permissionGranted = permission === 'granted';
        }
    });

    let lastPercentage;
    afterUpdate(() => {
        if(!permissionGranted || !totalUsageLimitBytes) {
            return;
        }

        let usedBytes = convertToBytes(totalUsageBytes);
        let limitBytes = convertToBytes(totalUsageLimitBytes);
        let usagePercentage = Math.ceil(parseInt((usedBytes * 100 / limitBytes).toFixed(0)));

        if(lastPercentage == usagePercentage) {
            return;
        } else {
            lastPercentage = usagePercentage;
        }

        let percentage;
        if (usagePercentage >= 48 && usagePercentage <= 52) {
            percentage = "50";
        } else if (usagePercentage >= 72 && usagePercentage <= 77) {
            percentage = "75";
        } else if (usagePercentage >= 88 && usagePercentage <= 92) {
            percentage = "90";
        } else if (usagePercentage > 100) {
            percentage = "(100+)";
        } else {
            return;
        }

        sendNotification({ title: 'Network Usage Alerts!', body: "You already used " + percentage + "% of your network resources!" });
    });
</script>

<div class="level">
    <div class="level-item has-text-centered">
        <div class="mr-6 px-5">
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

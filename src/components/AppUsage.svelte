<script>
    import { onMount } from "svelte";
    import { Chart } from "chart.js";

    import { convertToBytes, formatByteValue } from "../utils/units";
    import { settingsStore } from "../utils/settingsStore";
    import { barChartConfig } from "../utils/chartOptions";
    import { listen } from "@tauri-apps/api/event";
    import { convertJsonToArray } from "../utils/json";

    let applicationData;
    let fetchedData;

    let chartCtx;
    let chartElement;

    onMount(() => {
        chartCtx = chartElement.getContext("2d");
        chartElement = new Chart(chartCtx, barChartConfig);
    });

    onMount(async () => {
        async function fetchAppUsageData() {
            const unlisten = await listen('application-event', (data) => {
                fetchedData = JSON.parse(data.payload);
            })

            return unlisten;
        }

        const cleanup = await fetchAppUsageData();
        const interval = setInterval(() => {
            applicationData = convertJsonToArray(fetchedData);
        }, $settingsStore.delay * 1000);

        return () => {
            cleanup();
            clearInterval(interval);
        }
    });

    $: if(applicationData && chartElement) {
        chartElement.data.labels = applicationData.map(value => value.name);
        chartElement.data.datasets[0].data = applicationData.map(value => {
            const formatted = formatByteValue(convertToBytes(value.download) + convertToBytes(value.upload));
            return formatted.value;
        });

        chartElement.update();
    }
</script>

<canvas bind:this={chartElement}></canvas>

<script>
    import { listen } from "@tauri-apps/api/event"

    import { onMount } from "svelte";
    import { Chart } from "chart.js";

    import { hostsDataStore } from "../utils/hostsStore";
    import { settingsStore } from "../utils/settingsStore";
    import { convertToBytes, formatByteValue } from "../utils/units";
    import { doughnutChartConfig } from "../utils/chartOptions";
    import { convertJsonToArray } from "../utils/json";

    let fetchedData;
    let protocolData;

    let chartCtx;
    let chartElement;

    onMount(() => {
        chartCtx = chartElement.getContext("2d");
        chartElement = new Chart(chartCtx, doughnutChartConfig);
    });

    onMount(async () => {
        async function fetchProtocolData() {
            const unlisten = await listen('protocol-event', (data) => {
                fetchedData = JSON.parse(data.payload);
            })

            return unlisten;
        }

        const cleanup = await fetchProtocolData();
        const interval = setInterval(() => {
            protocolData = convertJsonToArray(fetchedData);
            hostsDataStore.set(protocolData);
        }, $settingsStore.delay * 1000);

        return () => {
            cleanup();
            clearInterval(interval);
        }
    });

    $: if(protocolData && chartElement) {
        chartElement.data.labels = protocolData.map(value => value.key);
        chartElement.data.datasets[0].data = protocolData.map(value => {
            return formatByteValue(convertToBytes(value.download) + convertToBytes(value.upload));
        });
        chartElement.update();
    }

</script>

<canvas bind:this={chartElement}></canvas>

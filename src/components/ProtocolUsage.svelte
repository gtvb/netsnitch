<script>
    import { listen } from "@tauri-apps/api/event"

    import { onMount } from "svelte";
    import { Chart } from "chart.js";

    import { hostsDataStore } from "../utils/store";
    import { convertToBytes, formatByteValue } from "../utils/units";
    import { generateRandomHexColor } from "../utils/color";

    let fetchedData;
    let protocolData;

    let chartCtx;
    let chartElement;

    onMount(() => {
        const backgrounds = [];
        for(let i = 0; i < 50; i++) {
            backgrounds.push(generateRandomHexColor());
        }

        chartCtx = chartElement.getContext("2d");
        chartElement = new Chart(chartCtx, {
            type: "doughnut",
            data: {
                labels: [],
                datasets: [{
                    data: [],
                    backgroundColor: backgrounds
                }],
            },
            options: {
                responsive: true,
                plugins: {
                    legend: {
                        position: 'right',
                        display: true,
                    },
                    title: {
                        display: true,
                        text: 'By Protocol'
                    }
                },
            }
        });
    });

    onMount(async () => {
        async function fetchAppUsageData() {
            const unlisten = await listen('protocol-event', (data) => {
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
            protocolData = jsonArray;
            hostsDataStore.set(protocolData);
        }, 3000);

        return () => {
            cleanup();
            clearInterval(interval);
        }
    });

    $: if(protocolData && chartElement) {
        chartElement.data.labels = protocolData.map(value => {
            const sum = formatByteValue(convertToBytes(value.download) + convertToBytes(value.upload));
            return value.key + "("+sum.unit+"B)";
        });
        chartElement.data.datasets[0].data = protocolData.map(value => {
            return formatByteValue(convertToBytes(value.download) + convertToBytes(value.upload));
        });
        chartElement.update();
    }

</script>

<div style="max-height: 250px" class="block">
    <canvas bind:this={chartElement}></canvas>
</div>

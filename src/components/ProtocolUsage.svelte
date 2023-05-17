<script>
    import { listen } from "@tauri-apps/api/event"
    import { onMount } from "svelte";
    import { convertToMegabytes } from "../utils/units";
    import { Chart } from "chart.js";
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
                labels: ["None"],
                datasets: [{
                    data: [],
                    backgroundColor: backgrounds
                }],
            },
            options: {
                responsive: true,
                plugins: {
                    legend: {
                        position: 'top',
                        display: false,
                    },
                    title: {
                        display: true,
                        text: 'Protocol Usage'
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
        }, 3000);

        return () => {
            cleanup();
            clearInterval(interval);
        }
    });

    $: if(protocolData && chartElement) {
        chartElement.data.labels = protocolData.map(value => value.name);
        chartElement.data.datasets[0].data = protocolData.map(value => convertToMegabytes(value.total));
        chartElement.data.datasets[0].labels = protocolData.map(_ => "Traffic (kB)");

        chartElement.update();
    }

</script>

<canvas bind:this={chartElement}></canvas>

<script>
    import { listen } from "@tauri-apps/api/event"

    import { onMount } from "svelte";
    import { Chart } from "chart.js";

    import { convertToBytes, formatByteValue } from "../utils/units";

    let fetchedData; 
    let applicationData;

    let chartCtx;
    let chartElement;

    onMount(() => {
        chartCtx = chartElement.getContext("2d");
        chartElement = new Chart(chartCtx, {
            type: "bar",
            data: {
                datasets: [{
                    data: [],
                    backgroundColor: [
                        "orange",
                        "green",
                        "red",
                        "blue",
                        "magenta",
                        "yellow",
                        "brown",
                    ]
                }]
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
                        text: 'By Application/Process'
                    }
                },
                scales: {
                    y: {
                        beginAtZero: true,
                    }
                }
            }
        });
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
            const jsonArray = Object.entries(fetchedData).map(([key, val]) => {
                return {
                    ...val,
                    key
                }
            });
            applicationData = jsonArray;
        }, 3000);

        return () => {
            cleanup();
            clearInterval(interval);
        }
    });

    $: if(applicationData && chartElement) {
        chartElement.data.labels = applicationData.map(value => {
            return value.name;
        });
        chartElement.data.datasets[0].data = applicationData.map(value => {
            const formatted = formatByteValue(convertToBytes(value.download) + convertToBytes(value.upload));
            return formatted.value;
        });

        chartElement.update();
    }
</script>

<canvas bind:this={chartElement}></canvas>

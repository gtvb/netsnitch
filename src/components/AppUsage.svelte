<script>
    import { listen } from "@tauri-apps/api/event"
    import { onMount } from "svelte";
    import { convertToMegabytes } from "../utils/units";
    import { Chart } from "chart.js";
    import { generateRandomHexColor } from "../utils/color";

    let fetchedData;
    let applicationData;

    let chartCtx;
    let chartElement;

    onMount(() => {
        chartCtx = chartElement.getContext("2d");
        chartElement = new Chart(chartCtx, {
            type: "bar",
            data: {
                labels: [],
                datasets: [{
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
                        text: 'Application Usage'
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
        chartElement.data.labels = applicationData.map(value => value.name);
        chartElement.data.datasets[0].data = applicationData.map(value => {
            return convertToKilobytes(value.download) + convertToKilobytes(value.upload);
        });

        chartElement.update();
    }

</script>

<canvas bind:this={chartElement}></canvas>

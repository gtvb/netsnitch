import { generateRandomHexColor } from "../utils/color";

const backgrounds = [];
for(let i = 0; i < 50; i++) {
    backgrounds.push(generateRandomHexColor());
}

const barChartConfig = {
    type: "bar",
    data: {
        datasets: [{
            data: [],
            backgroundColor: backgrounds
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
                text: 'By Application/Process (MegaBytes)'
            }
        },
        scales: {
            y: {
                beginAtZero: true,
            }
        }
    }
}


const doughnutChartConfig = {
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
        layout: {
            padding: {
                right: 10
            }
        },
        plugins: {
            legend: {
                position: 'right',
                display: true,
            },
            title: {
                display: true,
                text: 'By Protocol (MegaBytes)'
            }
        },
    }
}

export {
    barChartConfig,
    doughnutChartConfig
}

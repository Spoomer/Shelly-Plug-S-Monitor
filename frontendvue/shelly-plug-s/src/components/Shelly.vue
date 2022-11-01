<script setup lang="ts">
import { reactive } from 'vue'

const dataArr : any[] = [];
const state = reactive({
    currentJson: JSON.parse("{}"),
    series: [
        {
            name: "elect. Energy",
            data: dataArr,
        },
    ],
    energy: 0,
    date:new Date()
})

const options = {
    chart: {
        id: "energylinechart",
    },
    xaxis: {
        type: "datetime",
    },
    yaxis: {
        title: { text: "Ws" },
    },
    tooltip: {
        x: { format: "dd.MM.yyyy HH.mm.ss" },
    },
}
async function fetchApi() {
    return await fetch("http://127.0.0.1:8080/api/shelly").then((res) => {
        if (res.ok) {
            return res.json();
        } else return "";
    });
}
function round(number: number, decimals: number): number {
    const decimalhelper = 10 ** decimals;
    return Math.round((number + Number.EPSILON) * decimalhelper) / decimalhelper;
}
const interval = setInterval(() => {
    fetchApi().then((j) => state.currentJson = j);
    if (state.currentJson.timestamp) {
        state.series[0].data.push({
            x: state.currentJson.timestamp * 1000,
            y: state.currentJson.power,
        });
        state.energy += state.currentJson.power;
    }
    const date = new Date(state.currentJson.timestamp * 1000);

    //ApexCharts.exec("energylinechart", "updateSeries", series);
}, 1000);
</script>

<template>
    <div id="shelly">
        <p>Current Power: {{ state.currentJson.power }} W</p>
        <p>
            Last measured Energy (1min): {{ state.currentJson.counters[0] + " Wm or " }}
            {{ round(state.currentJson.counters[0] / 60000, 6) }}
            {{ " kWh" }}
        </p>
        <p>
            Total Energy since plug in: {{ state.currentJson.total }}
            {{ " Wm or " }}
            {{ round(state.currentJson.total / 60000, 3) }}
            {{ " kWh" }}
        </p>
        <p>
            Total Energy since refreshing the page: {{ round(state.energy, 3) + " Ws" }}
        </p>
        <p>Timestamp: {{ state.date.toUTCString() }} </p>
    </div>
    <apexchart :options="options" :series="state.series" type="line" height="300" />
</template>

<style scoped>
#shelly {
    flex: auto;
}
</style>
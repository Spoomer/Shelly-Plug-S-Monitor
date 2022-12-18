<script setup lang="ts">
import { reactive } from 'vue'

const dataArr: any[] = [];
const pricePerKwhCache: number = Number.parseFloat(localStorage.getItem("pricePerKwh") ?? "0");
function changepricePerKwh(event: Event) {
    pricePerKwh = Number.parseFloat((event.target as HTMLInputElement)?.value ?? 0);
    localStorage.setItem("pricePerKwh", pricePerKwh.toString())
}
let pricePerKwh: number = pricePerKwhCache;

function getTimeStampInSeconds(date: Date) {
    return (date.getTime()) / 1000;
}
const state = reactive({
    currentJson: JSON.parse("{}"),
    series: [
        {
            name: "elect. Energy",
            data: dataArr,
        },
    ],
    energy: 0,
    date: new Date(),
    from: new Date(0),
    to: new Date(),
})

const preferDarkmode: boolean = window.matchMedia("(prefers-color-scheme:dark)").matches;
const options = {
    chart: {
        id: "energylinechart",
    },
    xaxis: {
        type: "datetime",
    },
    yaxis: {
        title: { text: "Wm" },
    },
    tooltip: {
        x: { format: "dd.MM.yyyy HH.mm.ss" },
    },
    theme: {
        mode: preferDarkmode ? 'dark' : 'light',
    }
}
async function fetchApi(from: number, to: number) {
    return await fetch(`http://127.0.0.1:8080/api/archive?from=${from}&to=${to}`).then((res) => {
        if (res.ok) {
            return res.json();
        } else return "";
    });
}
function round(number: number, decimals: number): number {
    const decimalhelper = 10 ** decimals;
    return Math.round((number + Number.EPSILON) * decimalhelper) / decimalhelper;
}
function updateChart() {
    let fromEpoch = round((state.from.getTime()) / 1000, 0);
    let toEpoch = round((state.to.getTime()) / 1000, 0);
    fetchApi(fromEpoch, toEpoch).then((json) => {

        if (json[0]) {
            state.series[0].data = [];
            json.forEach(function (ele: any) {
                state.series[0].data.push({
                    x: ele.timestamp * 1000,
                    y: ele.energy,
                });
            });
        }
    });
}
updateChart();
</script>


<template>
    <div class="archive">
        <div class="form-floating mb-3">
            <label for="fromInput" color="black">from: </label>
            <input type="datetime" class="form-control" id="fromInput" v-model="state.from" style="width:50%">
            
        </div>
        <div class="form-floating mb-3">
            <label for="toInput">to: </label>
            <input type="datetime" class="form-control" id="toInput" v-model="state.to" style="width:50%">
        </div>
        <button @click="updateChart">Refresh</button>
        <apexchart :options="options" :series="state.series" type="line" height="300" />
    </div>
</template>
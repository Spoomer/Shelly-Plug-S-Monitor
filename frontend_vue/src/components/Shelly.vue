<script setup lang="ts">
import { reactive } from 'vue'
import { watch } from 'vue';

const dataArr: any[] = [];
const pricePerKwhCache = Number.parseFloat(localStorage.getItem("pricePerKwh") ?? "0");

function displayLastMeasuredDate(): string {
    let dateStr: string = new Date(state.date.setSeconds(0)).toLocaleString();
    if (dateStr === 'Invalid Date') {
        return 'No Measurement / No Connection';
    }
    return dateStr;
}
let pricePerKwh = pricePerKwhCache;
interface ShellyState {
    currentJson: any,
    series: {
        name: string;
        data: any[];
    }[],
    energy: number,
    date: Date,
}
const state: ShellyState = reactive({
    currentJson: JSON.parse("{}"),
    series: [
        {
            name: "elect. Energy",
            data: dataArr,
        },
    ],
    energy: 0,
    date: new Date(),
})

const preferDarkmode = window.matchMedia("(prefers-color-scheme:dark)").matches;
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
    theme: {
        mode: preferDarkmode ? 'dark' : 'light',
    }
}
async function fetchApi() {
    return await fetch("http://" + window.location.host + "/api/shelly").then((res) => {
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
    state.date = new Date((state.currentJson.timestamp  - state.currentJson.utcOffset) * 1000);

    //ApexCharts.exec("energylinechart", "updateSeries", series);
}, 1000);

watch(() => pricePerKwh, (newValue) => {
  localStorage.setItem("pricePerKwh", newValue.toString());
});
</script>

<template>
  <div id="shelly" v-if="state.currentJson.counters">
    <p><span class="fw-bold">Current Power: </span>{{ state.currentJson.power }} W</p>
    <p>
      <span class="fw-bold">Last measured Energy ({{ displayLastMeasuredDate() }}):
      </span>{{ state.currentJson.counters[0] + " Wm or " }}
      {{ round(state.currentJson.counters[0] / 60000, 6) }}
      {{ " kWh" }}
    </p>
    <p>
      <span class="fw-bold">Total Energy since plug in or restart: </span>{{ state.currentJson.total }}
      {{ " Wm or " }}
      {{ round(state.currentJson.total / 60000, 3) }}
      {{ " kWh" }}
    </p>
    <p>
      <span class="fw-bold">Total Energy since refreshing the page: </span>{{ round(state.energy, 3) + " Ws" }}
    </p>
    <p><input type="number" step="0.01" id="inputPricePerKwh" v-model.number="pricePerKwh"> <span class="fw-bold">Money
        per kWh</span></p>
    <p><span class="fw-bold">Cost since Reload: </span>{{ round(pricePerKwh * state.energy / 3600000, 6) }}</p>
    <p><span class="fw-bold">Cost since plug in: </span>{{ round(pricePerKwh * state.currentJson.total / 60000, 2) }}
    </p>
  </div>
  <apexchart :options="options" :series="state.series" type="line" height="300"/>
</template>

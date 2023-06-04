<script setup lang="ts">
import { reactive } from 'vue'
const getDateString = (date?: Date) => { const newDate = date ? new Date(date) : new Date(); return new Date(newDate.getTime() - newDate.getTimezoneOffset() * 60000).toISOString().slice(0, -1); };
const dataArr: any[] = [];
const pricePerKwhCache: number = Number.parseFloat(localStorage.getItem("pricePerKwh") ?? "0");
function changepricePerKwh(event: Event) {
    pricePerKwh = Number.parseFloat((event.target as HTMLInputElement)?.value ?? 0);
    localStorage.setItem("pricePerKwh", pricePerKwh.toString())
}
function handleFromInput(event: Event) {
    let input = Date.parse((event.target as HTMLInputElement)?.value);
    if (!Number.isNaN(input)) {
        state.from = new Date(input);
    }
}
function handleToInput(event: Event) {
    let input = Date.parse((event.target as HTMLInputElement)?.value);
    if (!Number.isNaN(input)) {
        state.to = new Date(input);
    }
}
let pricePerKwh: number = pricePerKwhCache;

function getTimeStampInSeconds(date: Date) {
    return (date.getTime()) / 1000;
}

interface ArchiveState {
    currentJson: any,
    series: {
        name: string;
        data: any[];
    }[],
    energy: number,
    date: Date,
    from: Date,
    to: Date
}

const state: ArchiveState = reactive({
    currentJson: JSON.parse("{}"),
    series: [
        {
            name: "elect. Energy",
            data: dataArr,
        },
    ],
    energy: 0,
    date: new Date(),
    from: new Date(1577836800000), //01.01.2020
    to: new Date(),
})

const preferDarkmode: boolean = window.matchMedia("(prefers-color-scheme:dark)").matches;
const options = {
    chart: {
        id: "energylinechart",
    },
    xaxis: {
        type: "datetime",
        labels : {
            datetimeUTC: false,
        }
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
    return await fetch("http://" + window.location.host + `/api/archive?from=${from}&to=${to}`).then((res) => {
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
        state.series[0].data = [];
        if (json[0]) {
            json.forEach(function (ele: any) {
                state.series[0].data.push({
                    x: ele.timestamp * 1000,
                    y: ele.energy / 1000,
                });
            });
        }
    });
}
updateChart();

async function deleteArchive() {
    await fetch("http://" + window.location.host + `/api/archive/delete?plugId=1`).then((res) => {
        if (res.ok) {
            updateChart();
        }
    });
}
</script>


<template>
    <div class="archive">
        <div class="form-floating mb-3">
            <label for="fromInput" color="black">from: </label>
            <input type="datetime-local" step="1" class="form-control" id="fromInput" :value="getDateString(state.from)"
                @input="handleFromInput" style="width:50%">

        </div>
        <div class="form-floating mb-3">
            <label for="toInput">to: </label>
            <input type="datetime-local" step="1" class="form-control" id="toInput" :value="getDateString(state.to)"
                @input="handleToInput" style="width:50%">
        </div>
        <button @click="updateChart">Refresh</button>
        <button @click="deleteArchive">Delete archive</button>
        <apexchart :options="options" :series="state.series" type="line" height="300" />
    </div>
</template>
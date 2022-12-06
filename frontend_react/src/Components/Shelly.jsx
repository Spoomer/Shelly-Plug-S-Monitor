import React from "react";
import { useEffect, useState } from "react";
import Chart from "react-apexcharts";
import { Translator } from "../translator.js";
function Shelly() {
  async function fetchApi() {
    return await fetch("http://localhost:8080/api/shelly").then((res) => {
      if (res.ok) {
        return res.json();
      } else return "";
    });
  }
  //const translator = new Translator("en");
  const timestamp = new Date().getTime();
  //json
  const [currentJson, setJson] = useState(JSON.parse("{}"));
  //chart-options
  const [options, setOptions] = useState({
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
  });
  //chart-series
  const [series, setSeries] = useState([
    {
      name: "elect. Energy",
      data: [],
    },
  ]);
  // Total Energy
  const [energy, setEnergy] = useState(0.0);
  useEffect(() => {
    const interval = setInterval(() => {
      fetchApi().then((j) => setJson(j));
      if (currentJson.timestamp) {
        series[0].data.push({
          x: currentJson.timestamp * 1000,
          y: currentJson.power,
        });
        setEnergy(energy + currentJson.power);
      }
      setSeries(series);
      ApexCharts.exec("energylinechart", "updateSeries", series);
    }, 1000);

    return () => clearInterval(interval);
  });
  if (currentJson.timestamp) {
    const date = new Date(currentJson.timestamp * 1000);

    return (
      <div>
        <p>Current Power: {currentJson.power} W</p>
        <p>
          Last measured Energy (1min): {currentJson.counters[0] + " Wm or "}
          {round(currentJson.counters[0] / 60000, 6)}
          {" kWh"}
        </p>
        <p>
          Total Energy since plug in: {currentJson.total}
          {" Wm or "}
          {round(currentJson.total / 60000, 3)}
          {" kWh"}
        </p>
        <p>
          Total Energy since refreshing the page: {round(energy, 3) + " Ws"}
        </p>
        <p>Timestamp: {date.toUTCString()} </p>
        <Chart options={options} series={series} type="line" height="300" />
      </div>
    );
  }
  return <div>No Connection to Shelly!</div>;
}

function round(number, decimals) {
  const decimalhelper = 10 ** decimals;
  return Math.round((number + Number.EPSILON) * decimalhelper) / decimalhelper;
}

export default Shelly;

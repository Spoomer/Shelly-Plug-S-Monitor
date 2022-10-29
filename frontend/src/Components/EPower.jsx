import React from "react";
import { useEffect, useState } from "react";
function Shelly() {
  async function fetchApi() {
    return await fetch("http://localhost:8080/api/shelly").then((res) => {
      if (res.ok) {
        return res.json();
      } else return "";
    });
  }
  const [currentJson, setJson] = useState(JSON.parse('{}'));
  useEffect(() => {
    const interval = setInterval(() => {
      fetchApi().then((j) => setJson(j));
    }, 1000);

    return () => clearInterval(interval);
  });
  if (currentJson.timestamp) {
    const date = new Date(currentJson.timestamp * 1000)
    return (
      <div>
        <p>Current Power: {currentJson.power} W</p>
        <p>Last measured Energy (1min): {currentJson.counters[0]} Wm or { Math.round((currentJson.counters[0] / 60000 + Number.EPSILON) * 1000000) / 1000000} kWh</p>
        <p>Total Energy: {currentJson.total} Wm or { Math.round((currentJson.total / 60000 + Number.EPSILON) * 1000) / 1000} kWh</p>
        <p>Timestamp: {date.toUTCString()} </p>
      </div>
    );
  }
  return <div>No Connection to Shelly!</div>;
}

export default Shelly;

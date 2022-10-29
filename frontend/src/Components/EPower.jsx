import React from "react";
import { useEffect, useState } from "react";
function EPower() {
  async function fetchApi() {
    return await fetch("http://localhost:8080/api/shelly")
    .then((res) => {
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
  return <div>current power: {currentJson.power} W</div>;
}

export default EPower;

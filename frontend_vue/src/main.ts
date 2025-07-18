import { createApp } from 'vue'
import App from './App.vue'

import 'bootstrap/dist/css/bootstrap.min.css';
import './assets/main.css'
import VueApexCharts from "vue3-apexcharts";

// Set Bootstrap theme based on browser color scheme
function setBootstrapTheme() {
  const isDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
  document.documentElement.setAttribute('data-bs-theme', isDark ? 'dark' : 'light');
}

setBootstrapTheme();
// Listen for changes in color scheme
window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', setBootstrapTheme);

const app = createApp(App); 
app.use(VueApexCharts);
app.mount('#app');
import { createApp } from "vue";
import { createPinia } from "pinia";
import "./style.css";
import "xterm/css/xterm.css";
import App from "./App.vue";

const pinia = createPinia();
const app = createApp(App);

app.use(pinia);
app.mount("#app");

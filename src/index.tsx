/* @refresh reload */
import { render } from "solid-js/web";

import "./style.css";
import 'uno.css'
import App from "./App";
import { listen } from "@tauri-apps/api/event";

render(() => <App />, document.getElementById("root") as HTMLElement);

await listen("data-received", ({ payload }) => {
    console.log(`Data received: "${payload}"`);
});
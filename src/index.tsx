/* @refresh reload */
import { render } from "solid-js/web";

import "./style.css";
import 'uno.css'
import App from "./App";
import { ThemeProvider } from "./components/ThemeProvider";

render(() => 
    <ThemeProvider>
        <App />
    </ThemeProvider>, 
    document.getElementById("root") as HTMLElement);
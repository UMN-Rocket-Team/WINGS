/* @refresh reload */
import { render } from "solid-js/web";

import "./style.css";
import 'uno.css'
import App from "./App";
import { ThemeProvider } from "./components/ThemeProvider";
import { BackendInteropManagerProvider } from "./components/BackendInteropManagerProvider";

render(() => 
    <ThemeProvider>
        <BackendInteropManagerProvider>
            <App />
        </BackendInteropManagerProvider>
    </ThemeProvider>, 
    document.getElementById("root") as HTMLElement);
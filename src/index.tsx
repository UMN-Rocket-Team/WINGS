/* @refresh reload */
import { render } from "solid-js/web";

import "./style.css";
import 'uno.css'
import App from "./App";
import { ThemeProvider } from "./components/ThemeProvider";
import { BackendInteropManagerProvider } from "./components/BackendInteropManagerProvider";
import { ModalProvider } from "./components/ModalProvider";

render(() => 
    <ThemeProvider>
        <BackendInteropManagerProvider>
            <ModalProvider>
                <App />
            </ModalProvider>
        </BackendInteropManagerProvider>
    </ThemeProvider>, 
    document.getElementById("root") as HTMLElement);
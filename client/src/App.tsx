import { JSX } from "solid-js";
import { BackendProvider } from "./backend_interop/BackendProvider";
import TabPage from "./tabs/TabPage";
import Homepage from "./tabs/Homepage";
import { ModalProvider } from "./core/ModalProvider";
import { ThemeProvider } from "./theme/ThemeProvider";
import { Router, Route } from "@solidjs/router"

const App = (): JSX.Element => {
    return (
        <ThemeProvider>
            <BackendProvider>
                <ModalProvider>
                    <Router>
                        <Route path="/" component={Homepage} />
                        <Route path="/newFlight" component={TabPage} />
                    </Router>
                </ModalProvider>
            </BackendProvider>
        </ThemeProvider>
    );
};

export default App;

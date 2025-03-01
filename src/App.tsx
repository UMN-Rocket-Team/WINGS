import { JSX } from "solid-js";
import { BackendProvider } from "./backend_interop/BackendProvider";
import TabPage from "./tabs/TabPage";
import Homepage from "./tabs/Homepage";
import { ModalProvider } from "./core/ModalProvider";
import { ThemeProvider } from "./theme/ThemeProvider";
import { Router, Route } from "@solidjs/router"
import { DisplaysProvider } from "./components/DisplaysProvider";
import GraphWindow from "./components/GraphWindow";

const App = (): JSX.Element => {
    return (
        <ThemeProvider>
            <BackendProvider>
                <DisplaysProvider>
                    <ModalProvider>
                        <Router>
                            <Route path="/" component={Homepage} />
                            <Route path="/newFlight" component={TabPage} />
                            <Route path="/newFlight/displays/:id" component={GraphWindow} />
                        </Router>
                    </ModalProvider>
                </DisplaysProvider>
            </BackendProvider>
        </ThemeProvider>
    );
};

export default App;

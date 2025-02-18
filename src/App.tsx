import { JSX } from "solid-js";
import { BackendProvider } from "./backend_interop/BackendProvider";
import TabPage from "./tabs/TabPage";
import Homepage from "./tabs/Homepage";
import { ModalProvider } from "./modals/ModalProvider";
import { ThemeProvider } from "./components/ThemeProvider";
import { Router, Route } from "@solidjs/router"
import { DisplaysProvider } from "./components/DisplaysProvider";

const App = (): JSX.Element => {
    return (
        <ThemeProvider>
            <BackendProvider>
                <DisplaysProvider>
                    <ModalProvider>
                        <Router>
                            <Route path="/" component={Homepage} />
                            <Route path="/newFlight" component={TabPage} />
                        </Router>
                    </ModalProvider>
                </DisplaysProvider>
            </BackendProvider>
        </ThemeProvider>
    );
};

export default App;

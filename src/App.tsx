import { JSX } from "solid-js";
import { BackendProvider } from "./backend_interop/BackendProvider";
import TabPage from "./tabs/TabPage";
import Homepage from "./tabs/Homepage";
import { ModalProvider } from "./modals/ModalProvider";
import { ThemeProvider } from "./components/ThemeProvider";
import { Router, Route } from "@solidjs/router"
import DummyComponent from "./graph/DummyComponent";

const App = (): JSX.Element => {
    return (
        <ThemeProvider>
            <BackendProvider>
                <ModalProvider>
                    <Router>
                        <Route path="/" component={Homepage} />
                        <Route path="/newFlight" component={TabPage} />
                        <Route path="/newFlight/:id" component={DummyComponent} />
                    </Router>
                </ModalProvider>
            </BackendProvider>
        </ThemeProvider>
    );
};

export default App;

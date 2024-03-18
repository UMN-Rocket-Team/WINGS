import { JSX } from "solid-js";
import { BackendProvider } from "./backend_interop/BackendProvider";
import FlightViewer from "./tabs/FlightViewer";
import Homepage from "./tabs/Homepage";
import { ModalProvider } from "./modals/ModalProvider";
import { ThemeProvider } from "./components/ThemeProvider";
import { Router, Routes, Route } from "@solidjs/router"

const App = (): JSX.Element => {
    return (
        <ThemeProvider>
            <BackendProvider>
                <ModalProvider>
                    <Router>
                        <Routes>
                            <Route path="/" component={Homepage} />
                            <Route path="/newFlight" component={FlightViewer} />
                        </Routes>
                    </Router>
                </ModalProvider>
            </BackendProvider>
        </ThemeProvider>
    );
};

export default App;

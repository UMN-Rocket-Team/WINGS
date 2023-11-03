import { JSX } from "solid-js";
import { BackendProvider } from "./components/BackendProvider";
import FlightViewer from "./components/FlightViewer";
import Homepage from "./components/Homepage";
import { ModalProvider } from "./components/ModalProvider";
import { ThemeProvider } from "./components/ThemeProvider";
import { Router, Routes, Route } from "@solidjs/router"
import SavedFlightViewer from "./components/SavedFlightViewer";

const App = (): JSX.Element => {
    return (
        <ThemeProvider>
            <BackendProvider>
                <ModalProvider>
                    <Router>
                        <Routes>
                            <Route path="/" component={Homepage} />
                            <Route path="/newFlight" component={FlightViewer} />
                            <Route path="/savedFlight" component={SavedFlightViewer} />
                        </Routes>
                    </Router>
                </ModalProvider>
            </BackendProvider>
        </ThemeProvider>
    );
};

export default App;

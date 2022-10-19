import { JSX } from "solid-js";
import { BackendInteropManagerProvider } from "./components/BackendInteropManagerProvider";
import FlightViewer from "./components/FlightViewer";
import Homepage from "./components/Homepage";
import { ModalProvider } from "./components/ModalProvider";
import { ThemeProvider } from "./components/ThemeProvider";
import { Router, Routes, Route } from "@solidjs/router"

const App = (): JSX.Element => {
    return (
        <ThemeProvider>
            <BackendInteropManagerProvider>
                <ModalProvider>
                    <Router>
                        <Routes>
                            <Route path="/" component={Homepage} />
                            <Route path="/newFlight" component={FlightViewer} />
                        </Routes>
                    </Router>
                </ModalProvider>
            </BackendInteropManagerProvider>
        </ThemeProvider>
    );
};

export default App;

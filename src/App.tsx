import { JSX } from "solid-js";
import { BackendInteropManagerProvider } from "./components/BackendInteropManagerProvider";
import FlightViewer from "./components/FlightViewer";
import Homepage from "./components/Homepage";
import { MainComponentProvider } from "./components/MainComponentProvider";
import { ModalProvider } from "./components/ModalProvider";
import { ThemeProvider } from "./components/ThemeProvider";
import { MainComponentType } from "./core/main_component";

const App = (): JSX.Element => {
    const mainComponents = {
        [MainComponentType.Homepage]: Homepage,
        [MainComponentType.FlightViewer]: FlightViewer,
    };

    return (
        <ThemeProvider>
            <BackendInteropManagerProvider>
                <ModalProvider>
                    <MainComponentProvider components={mainComponents} initialComponentId={MainComponentType.Homepage} />
                </ModalProvider>
            </BackendInteropManagerProvider>
        </ThemeProvider>
    );
};

export default App;

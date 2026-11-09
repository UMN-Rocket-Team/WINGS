import { JSX } from "react";
import { BackendProvider } from "./backend_interop/BackendProvider";
import TabPage from "./tabs/TabPage";
import Homepage from "./tabs/Homepage";
import { ModalProvider } from "./core/ModalProvider";
import { ThemeProvider } from "./theme/ThemeProvider";
import { BrowserRouter, Route } from "react-router-dom"

const AppRouter = () => {
    <BrowserRouter>
        <Route path="/">
            <Homepage />
        </Route>
        <Route path="/newFlight" >
            <TabPage />
        </Route>
    </BrowserRouter>
}

const App = (): JSX.Element => {
    return (
        <ThemeProvider>
            <BackendProvider>
                <ModalProvider>
                    <BrowserRouter>
                    </BrowserRouter>
                </ModalProvider>
            </BackendProvider>
        </ThemeProvider>
    );
};

export default App;

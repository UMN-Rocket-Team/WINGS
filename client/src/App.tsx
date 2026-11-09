import { JSX } from "react";
import { BrowserRouter, Route } from "react-router-dom"
import { ThemeProvider } from "./context/ThemeProvider";
import TauriProvider from "./context/TauriProvider";
import Home from "./views/Home";

const AppRouter = () => {
    <BrowserRouter>
        <Route path="/">
            <Home />
        </Route>
        <Route path="/newFlight" >
            <TabPage />
        </Route>
    </BrowserRouter>
}

const App = (): JSX.Element => {
    return (
        <ThemeProvider>
            <TauriProvider>
                <ModalProvider>
                    <BrowserRouter>
                    </BrowserRouter>
                </ModalProvider>
            </TauriProvider>
        </ThemeProvider>
    );
};

export default App;

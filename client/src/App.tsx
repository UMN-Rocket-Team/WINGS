import { JSX, ReactNode } from "react";
import { BrowserRouter, Route, Routes } from "react-router-dom"
import { ThemeProvider } from "./context/ThemeProvider";
import TauriProvider from "./context/TauriProvider";
import Home from "./views/Home";

const AppRouter = () => {
    return (
        <BrowserRouter>
            <Routes>
                <Route path="/" element={<Home />} />
            </Routes>
        </BrowserRouter>
    );
}

const App = (): JSX.Element => {
    return (
        <ThemeProvider>
            <TauriProvider>
                <AppRouter />
            </TauriProvider>
        </ThemeProvider>
    );
};

export default App;

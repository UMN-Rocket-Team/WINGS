/* @refresh reload */
import React from "react";
import ReactDOM from "react-dom/client";
import '../global.css';

import { StyledEngineProvider } from '@mui/material/styles';
import GlobalStyles from '@mui/material/GlobalStyles';

import App from "./App";

ReactDOM.createRoot(document.getElementById('root')!).render(
    <React.StrictMode>
        <StyledEngineProvider enableCssLayer>
            <GlobalStyles styles="@layer theme, base, mui, components, utilities;" />
            <App />
        </StyledEngineProvider>
    </React.StrictMode>
)
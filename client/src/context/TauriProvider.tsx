import { isTauri } from "@tauri-apps/api/core"
import React, { PropsWithChildren } from "react"
import {BackendProvider} from "./BackendProvider";

type TauriContextValue = {
    isTauri: boolean,
}

export function TauriProvider({children}: PropsWithChildren) {
    const [value, _setValue] = React.useState<TauriContextValue>({isTauri: isTauri()});
    const TauriContext = React.createContext<TauriContextValue>(value);
    const Provider = TauriContext.Provider;
    return (
        <Provider
            value={value}
        >
            {value.isTauri ? <BackendProvider >{children}</BackendProvider> : children}
        </Provider>
    )
}
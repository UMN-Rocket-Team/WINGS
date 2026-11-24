import React, { PropsWithChildren } from "react";
import { WidgetSelector } from '../components/modals/WidgetSelector';

export default function Home({children}: PropsWithChildren): React.ReactNode {
    return (
        <>
            <WidgetSelector></WidgetSelector>
            {children}
        </>
    )
}
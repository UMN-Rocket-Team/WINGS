import { WidgetGrid } from "@/components/widgets/WidgetGrid";
import React, { PropsWithChildren } from "react";

export default function Home({children}: PropsWithChildren): React.ReactNode {
    return (
        <WidgetGrid />
    )
}
import { Grid } from "@mui/material"
import { NewWidgetWidget } from "./NewWidgetWidget"
import { Widget } from "./Widget"
import { WidgetType } from "./types"

export type WidgetGridProps = {
    height: number,
    width: number
}

export function WidgetGrid() {
    return (
        <div className="p-2 gap-4 bg-slate-200 flex grid grid-cols-8 grid-rows-4">
            <Widget
                position={{
                    x: 1,
                    y: 1
                }}
            >
                <NewWidgetWidget />
            </Widget>
            <Widget position={{
                x: 2,
                y: 1
            }}        >
                <NewWidgetWidget />
            </Widget>
            <Widget position={{
                x: 3,
                y: 1
            }}        >
                <NewWidgetWidget />
            </Widget>
            <Widget position={{
                x: 1,
                y: 2
            }}        >
                <NewWidgetWidget />
            </Widget>
            <Widget position={{
                x: 2,
                y: 2
            }}        >
                <NewWidgetWidget />
            </Widget>
            <Widget position={{
                x: 3,
                y: 2
            }}        >
                <NewWidgetWidget />
            </Widget>
        </div>
    )
}
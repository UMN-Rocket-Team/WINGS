import { Component, createEffect, onCleanup, onMount } from "solid-js";
import { CategoryScale, Chart, ChartConfiguration, ChartTypeRegistry, LineController, LineElement, Point, PointElement, LinearScale, TimeScale, Title, Tooltip } from "chart.js";
import 'chartjs-adapter-luxon';
import { GraphStruct } from "./FieldsScreen";
import { useBackend } from "./BackendProvider";
import { parsedPackets } from "../backend_interop/buffers";

// Register the necessary components with ChartJS so that they can be used later
// Note: any components that are not registered here will act like no-ops if they are attempted to be used later!
Chart.register(LineController, CategoryScale, LinearScale, TimeScale, PointElement, LineElement, Title, Tooltip);

type SolidChartProps = {
    graph: GraphStruct;
};

/**
 * A component that displays the parsed data for a given packet field in a line chart
 * 
 * @param props an object containing the packet field to display data for
 */
const SolidChart: Component<SolidChartProps> = (props: SolidChartProps) => {
    const { parsedPacketCount } = useBackend();

    let canvas: HTMLCanvasElement;
    let chart: Chart;

    const colors: string[] = ["#FFD700", "black", "blue", "red"];
    const initialParsedPackets = parsedPackets[0];
    let datasets = []
    for (let i = 0; i < props.graph.y.length; i++) {
        const dataName = `data${i + 1}`;
        const dataValue = {
            label: props.graph.graphName,
            data: initialParsedPackets.map(packetData => ({x: packetData.fieldData[props.graph.x], y: packetData.fieldData[props.graph.y[i]] })) ?? [],
            backgroundColor: colors[i],
            borderColor: colors[i],
            spanGaps: true,
        }
        datasets.push({dataName: dataValue})
    }
    const data = {datasets};

    // const data = {
    //     datasets: [{
    //         label: props.graph.graphName,
    //         // TODO BELOW: MAKE DATA INITIALIZED CORRECTLY
    //         data: initialParsedPackets?.map(packetData => ({ x: packetData.fieldData[props.graph.x], y: packetData.fieldData[props.graph.y[0]] })) ?? [],
    //         backgroundColor: 'dark-blue',
    //         borderColor: 'blue',
    //         spanGaps: true,
    //     }]
    // };

    const config: ChartConfiguration<keyof ChartTypeRegistry, Point[], unknown> = {
        type: "line",
        data: data,
        options: {
            responsive: true,
            maintainAspectRatio: false,
            animation: false,
            parsing: false,
            normalized: true,
            interaction: {
                mode: 'nearest',
                axis: 'x',
                intersect: false
            },
            plugins: {
                decimation: {
                    enabled: true,
                    algorithm: "lttb",
                },
                title: {
                    display: true,
                    text: props.graph.graphName,
                }
            },
            scales: {
                x: {
                    type: "time",
                    time: {
                        unit: 'second',
                        displayFormats: {
                            second: 'HH:mm:ss'
                        }
                    },
                    display: true,
                },
            },
        }
    };

    let lastPacketCount = initialParsedPackets?.length ?? 0;

    // // Add new data to the chart whenever new data is parsed by the packet parser
    createEffect(() => {
        // Update this effect whenever the parsed packet count changes
        const _unused = parsedPacketCount();

        const packetData = parsedPackets[0];

        if (packetData === undefined || lastPacketCount == packetData.length) {
            return;
        }
        for (let i = 0; i < datasets.length; i++) {
            config.data.datasets[i].data.push(...packetData.slice(lastPacketCount).map(packetData => ({ x: packetData.fieldData[props.graph.x], y: packetData.fieldData[props.graph.y[i]] })));
        }


        lastPacketCount = packetData.length;

        chart.update();
    }, { defer: true });

    onMount(() => {
        chart = new Chart(canvas, config);
        const packetData = parsedPackets[0];
        if (packetData === undefined) {
            return;
        }

        for (let i = 0; i < datasets.length; i++) {
            config.data.datasets[i].data.push(...packetData.map(packetData => ({ x: packetData.fieldData[props.graph.x], y: packetData.fieldData[props.graph.y[i]] })));
        }
        // config.data.datasets[0].data.push(...packetData.map(packetData => ({
        //     x: packetData.fieldData[props.graph.x],
        //     y: packetData.fieldData[props.graph.y[0]]
        // })));
        lastPacketCount = packetData.length;
        chart.update();
    });

    onCleanup(() => {
        chart?.destroy();
    });

    return (
        <canvas ref={canvas!} />
    );
};

// TODO WORK ON ADDING A LEGEND AND BETTER AXIS NAMING AND COLOR CHANGING!

export default SolidChart;

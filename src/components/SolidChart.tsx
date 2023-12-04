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
    fieldInPacket: GraphStruct;
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

    const initialParsedPackets = parsedPackets[props.fieldInPacket.packetId];

    const data = {
        datasets: [{
            label: props.fieldInPacket.name,
            data: initialParsedPackets?.map(packetData => ({ x: packetData.timestamp, y: packetData.fieldData[props.fieldInPacket.fieldIndex] })) ?? [],
            backgroundColor: 'dark-blue',
            borderColor: 'blue',
            spanGaps: true,
        }]
    };

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
                    text: props.fieldInPacket.name,
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

    // Add new data to the chart whenever new data is parsed by the packet parser
    createEffect(() => {
        // Update this effect whenever the parsed packet count changes
        const _unused = parsedPacketCount();

        const packetData = parsedPackets[props.fieldInPacket.packetId];

        if (packetData === undefined || lastPacketCount == packetData.length) {
            return;
        }

        config.data.datasets[0].data.push(...packetData.slice(lastPacketCount).map(packetData => ({ x: packetData.timestamp, y: packetData.fieldData[props.fieldInPacket.fieldIndex] })));

        lastPacketCount = packetData.length;

        chart.update();
    }, { defer: true });

    onMount(() => {
        console.log("here")
        chart = new Chart(canvas, config);
        const packetData = parsedPackets[props.fieldInPacket.packetId];
        if (packetData === undefined) {
            return;
        }

        config.data.datasets[0].data.push(...packetData.map(packetData => ({
            x: packetData.timestamp,
            y: packetData.fieldData[props.fieldInPacket.fieldIndex].data
        })));
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

export default SolidChart;
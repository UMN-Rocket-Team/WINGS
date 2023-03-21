import { Component, createEffect, onCleanup, onMount } from "solid-js";
import { CategoryScale, Chart, ChartConfiguration, ChartTypeRegistry, LinearScale, LineController, LineElement, Point, PointElement } from "chart.js";
import { FieldInPacket } from "./FieldsView";
import { useBackendInteropManager } from "./BackendInteropManagerProvider";
import { parsedPackets } from "../backend_interop/buffers";

Chart.register(LineController, CategoryScale, LinearScale, PointElement, LineElement);

type SolidChartProps = {
    fieldInPacket: FieldInPacket;
};

const SolidChart: Component<SolidChartProps> = (props: SolidChartProps) => {
    const { parsedPacketCount } = useBackendInteropManager();

    let canvas: HTMLCanvasElement;
    let chart: Chart;

    const initialParsedPackets = parsedPackets[props.fieldInPacket.packetViewModel.id];

    const data = {
        datasets: [{
            label: 'My First dataset',
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
            },
            scales: {
                x: {
                    type: "linear",
                    min: 0,
                    max: 100,
                    display: true,
                    axis: "x",
                },
                y: {
                    min: 0,
                    max: 100
                }
            }
        }
    };

    let lastPacketCount = initialParsedPackets?.length ?? 0;

    createEffect(() => {
        const _unused = parsedPacketCount();

        const packetData = parsedPackets[props.fieldInPacket.packetViewModel.id];

        console.log("in chart", packetData, lastPacketCount)

        if (packetData === undefined || lastPacketCount == packetData.length) {
            return;
        }

        config.data.datasets[0].data.push(...packetData.slice(lastPacketCount).map(packetData => ({ x: packetData.timestamp, y: packetData.fieldData[props.fieldInPacket.fieldIndex] })));

        lastPacketCount = packetData.length;

        chart.update();
    }, { defer: true });

    onMount(() => {
        chart = new Chart(canvas, config);
    });

    onCleanup(() => {
        chart.destroy();
    });

    return (
        <canvas ref={canvas!} />
    );
};

export default SolidChart;
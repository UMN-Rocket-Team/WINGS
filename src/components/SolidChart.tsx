import { Component, createEffect, onCleanup, onMount } from "solid-js";
import { CategoryScale, Chart, ChartConfiguration, ChartTypeRegistry, LineController, LineElement, Point, PointElement, LinearScale, TimeScale, Title } from "chart.js";
import 'chartjs-adapter-luxon';
import { FieldInPacket } from "./FieldsView";
import { useBackendInteropManager } from "./BackendInteropManagerProvider";
import { parsedPackets } from "../backend_interop/buffers";

Chart.register(LineController, CategoryScale, LinearScale, TimeScale, PointElement, LineElement, Title);

type SolidChartProps = {
    fieldInPacket: FieldInPacket;
};

const SolidChart: Component<SolidChartProps> = (props: SolidChartProps) => {
    const { parsedPacketCount } = useBackendInteropManager();

    let canvas: HTMLCanvasElement;
    let chart: Chart;

    const initialParsedPackets = parsedPackets[props.fieldInPacket.packetId];

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

    createEffect(() => {
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
        chart = new Chart(canvas, config);
    });

    onCleanup(() => {
        chart?.destroy();
    });

    return (
        <canvas ref={canvas!} />
    );
};

export default SolidChart;
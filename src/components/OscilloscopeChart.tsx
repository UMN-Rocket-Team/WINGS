import { Component, createEffect, onCleanup, onMount } from "solid-js";
import { CategoryScale, Chart, ChartConfiguration, ChartTypeRegistry, LineController, LineElement, Point, PointElement, LinearScale, TimeScale, Title, Tooltip } from "chart.js";
import zoomPlugin from 'chartjs-plugin-zoom';
import 'chartjs-adapter-luxon';
import { useBackend } from "../backend_interop/BackendProvider";
import { unDecimatedPackets } from "../backend_interop/buffers";
import { PacketComponentType, PacketField } from "../backend_interop/types";
import { OscilloscopeGraphStruct } from "../modals/OscilloscopeGraphSettingsModal";

// Register the necessary components with ChartJS so that they can be used later
// Note: any components that are not registered here will act like no-ops if they are attempted to be used later!
Chart.register(LineController, CategoryScale, LinearScale, TimeScale, PointElement, LineElement, Title, Tooltip, zoomPlugin);

/**
 * A component that displays the parsed data for a given OscilloscopeGraphStruct in a line chart
 * 
 * @param graph an OscilloscopeGraphStruct that is the graph that is being created
 */
const OscilloscopeGraphDisplayElement: Component<OscilloscopeGraphStruct> = (props) => {
    // Type guard
    if (props.type !== "oscilloscopeGraph") return <div>Invalid graph configuration</div>;

    // Safe cast after type check
    const graph = props as OscilloscopeGraphStruct;


    const { parsedPacketCount, PacketStructureViewModels } = useBackend();

    let containerElement: HTMLDivElement;
    let canvas: HTMLCanvasElement;
    let chart: Chart;

    // Decimation variables:
    var ptr1 = 1;
    var ptr2 = 1;
    var wall = 500;
    var multiple = 2;
    var next = 1;

    const colors: string[] = ["#FFD700", "black", "blue", "red"];
    //adds an empty array if we haven't received data in the packet type we want
    if (unDecimatedPackets[graph.packetID] === undefined) {
        unDecimatedPackets[graph.packetID] = [];
    }

    const initialunDecimatedPackets = unDecimatedPackets[graph.packetID];
    const ratio = initialunDecimatedPackets.length / 100;
    let datasets = []
    for (let i = 0; i < graph.y.length; i++) {
        const dataValue = {
            label: ((PacketStructureViewModels.find(psViewModel => (psViewModel.id === graph.packetID))?.components.find(component => component.type === PacketComponentType.Field && (component.data as PacketField).index === graph.y[i]))?.data as PacketField).name,
            data: initialunDecimatedPackets.map(packetData => ({ x: packetData.fieldData[graph.x], y: packetData.fieldData[graph.y[i]] })) ?? [],
            backgroundColor: graph.colors[i % graph.colors.length],
            borderColor: graph.colors[i % graph.colors.length],
            spanGaps: true,
        };
        datasets.push(dataValue);
    }
    const data = { datasets };

    const resizeObserver = new ResizeObserver((changes) => {
        for (const change of changes) {
            chart.resize(change.contentRect.width, change.contentRect.height);
        }
    });

    const config: ChartConfiguration<keyof ChartTypeRegistry, Point[], unknown> = {
        type: "line",
        data: data,
        options: {
            responsive: false,
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
                    text: graph.displayName,
                },
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
                    min: Date.now() - 10000, 
                    max: Date.now(),
                }
            },
        }
    };

    let lastPacketCount = initialunDecimatedPackets?.length ?? 0;

    createEffect(() => {
        const _unused = parsedPacketCount();

        if (unDecimatedPackets[graph.packetID] === undefined) {
            unDecimatedPackets[graph.packetID] = [];
        }
        const packetData = unDecimatedPackets[graph.packetID];

        if (packetData === undefined || chart === undefined) {
            return;
        }
        for (let i = 0; i < datasets.length; i++) {
            // config.data.datasets[i].data.push(...packetData.slice(lastPacketCount).map(packetData => ({ x: packetData.fieldData[graph.x], y: packetData.fieldData[graph.y[i]] })));
            config.data.datasets[i].data = packetData.map(packetData => ({ x: packetData.fieldData[graph.x], y: packetData.fieldData[graph.y[i]] }));
            // console.log(config.data.datasets[i].data.length);
        }

        // Update x-axis range
        chart.options.scales!.x!.min = Date.now() - 10000;
        chart.options.scales!.x!.max = Date.now();

        chart.update();
    });

    onMount(() => {
        // canvas is set by ref, so elements will be defined by this point
        chart = new Chart(canvas!, config);
        resizeObserver.observe(containerElement!);

        if (unDecimatedPackets[graph.packetID] === undefined) {
            unDecimatedPackets[graph.packetID] = [];
        }
        const packetData = unDecimatedPackets[graph.packetID];
        if (packetData === undefined) {
            return;
        }

        // Adds previous data
        lastPacketCount = packetData.length;
        chart.update();

    });

    onCleanup(() => {
        chart?.destroy();
        resizeObserver.disconnect();
    });

    return (
        <div
            ref={containerElement!}
            // Using absolute here means that the canvas' size will not affect
            // the size of the container it's in. This prevents getting into a
            // situation where the container can never shrink because the canvas
            // has a fixed size applied to it.
            class="relative w-full h-full"
        >
            <canvas
                ref={canvas!}
                // Using absolute positioning prevents the canvas' size from affecting
                // the size of the container it is in. Chart.js will give the canvas a
                // fixed width so that would prevent the container from shrinking.
                class="absolute"
            />
            <button
                class="absolute top-2 right-2 bg-gray-500 text-white p-3 text-xs rounded hover:bg-gray-600"
                onClick={() => chart?.resetZoom('none')}
            >
                Reset Zoom
            </button>
        </div>
    );
};

// TODO WORK ON ADDING A LEGEND AND BETTER COLOR CHANGING!

export default OscilloscopeGraphDisplayElement;

import { Component, createEffect, onCleanup, onMount } from "solid-js";
import { CategoryScale, Chart, ChartConfiguration, ChartTypeRegistry, LineController, LineElement, Point, PointElement, LinearScale, TimeScale, Title, Tooltip } from "chart.js";
import zoomPlugin from 'chartjs-plugin-zoom';
import 'chartjs-adapter-luxon';
import { useBackend } from "../backend_interop/BackendProvider";
import { parsedPackets } from "../backend_interop/buffers";
import { PacketComponentType, PacketField } from "../backend_interop/types";
import { GraphStruct } from "../modals/GraphSettingsModal";

// Register the necessary components with ChartJS so that they can be used later
// Note: any components that are not registered here will act like no-ops if they are attempted to be used later!
Chart.register(LineController, CategoryScale, LinearScale, TimeScale, PointElement, LineElement, Title, Tooltip, zoomPlugin);

/**
 * A component that displays the parsed data for a given graphStruct in a line chart
 * 
 * @param graph a graphStruct that is the graph that is being created
 */
const GraphDisplayElement: Component<GraphStruct> = (props) => {
    // Type guard
    if (props.type !== "graph") return <div>Invalid graph configuration</div>;

    // Safe cast after type check
    const graph = props as GraphStruct;


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
    if (parsedPackets[graph.packetID] === undefined) {
        parsedPackets[graph.packetID] = [];
    }

    const initialParsedPackets = parsedPackets[graph.packetID];
    const ratio = initialParsedPackets.length / 100;
    let datasets = []
    for (let i = 0; i < graph.y.length; i++) {
        const dataValue = {
            label: ((PacketStructureViewModels.find(psViewModel => (psViewModel.id === graph.packetID))?.components.find(component => component.type === PacketComponentType.Field && (component.data as PacketField).index === graph.y[i]))?.data as PacketField).name,
            data: initialParsedPackets.map(packetData => ({ x: packetData.fieldData[graph.x], y: packetData.fieldData[graph.y[i]] })) ?? [],
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
            // Chart.js' responsive mode is not very good. Instead, we manually use
            // chart.resize() to update the chart's size to match a container element
            // that we control. Fixes issue where the size of the graph could only
            // increase and never go down.
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
                zoom: {
                    pan: {
                        enabled: true,
                        mode: 'x'
                    },
                    zoom: {
                        pinch: {
                            enabled: true
                        },
                        wheel: {
                            enabled: true
                        },
                        mode: 'x'
                    },
                    limits: {
                        x: {
                            minDelay: 0,
                            maxDelay: 4000,
                            minDuration: 1000,
                            maxDuration: 20000
                        }
                    }
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
                    //     min: -90,
                    //     max: 50
                    // },
                    // y: {
                    //     min: -90,
                    //     max: 50
                }
            },
        }
    };

    let lastPacketCount = initialParsedPackets?.length ?? 0;

    // // Add new data to the chart whenever new data is parsed by the packet parser
    createEffect(() => {
        // Update this effect whenever the parsed packet count changes
        const _unused = parsedPacketCount();

        if (parsedPackets[graph.packetID] === undefined) {
            parsedPackets[graph.packetID] = [];
        }
        const packetData = parsedPackets[graph.packetID];

        if (packetData === undefined || chart === undefined) {
            return;
        }
        for (let i = 0; i < datasets.length; i++) {
            // config.data.datasets[i].data.push(...packetData.slice(lastPacketCount).map(packetData => ({ x: packetData.fieldData[graph.x], y: packetData.fieldData[graph.y[i]] })));
            config.data.datasets[i].data = packetData.map(packetData => ({ x: packetData.fieldData[graph.x], y: packetData.fieldData[graph.y[i]] }));
            // console.log(config.data.datasets[i].data.length);
        }


        lastPacketCount = packetData.length;
        chart.update();
    }, { defer: true });

    onMount(() => {
        // canvas is set by ref, so elements will be defined by this point
        chart = new Chart(canvas!, config);
        resizeObserver.observe(containerElement!);

        if (parsedPackets[graph.packetID] === undefined) {
            parsedPackets[graph.packetID] = [];
        }
        const packetData = parsedPackets[graph.packetID];
        if (packetData === undefined) {
            return;
        }

        // Adds previous data
        for (let i = 0; i < datasets.length; i++) {
            console.log(config.data.datasets[i].data);
        }
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
            class="w-full h-full"
        >
            <canvas
                ref={canvas!}
                // Using absolute positioning prevents the canvas' size from affecting
                // the size of the container it is in. Chart.js will give the canvas a
                // fixed width so that would prevent the container from shrinking.
                class="absolute"
            />
        </div>
    );
};

// TODO WORK ON ADDING A LEGEND AND BETTER COLOR CHANGING!

export default GraphDisplayElement;

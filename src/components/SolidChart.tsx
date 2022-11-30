import { onCleanup, onMount } from "solid-js";
import { JSX } from "solid-js/jsx-runtime";
import { CategoryScale, Chart, ChartConfiguration, ChartTypeRegistry, LinearScale, LineController, LineElement, Point, PointElement } from "chart.js";

Chart.register(LineController, CategoryScale, LinearScale, PointElement, LineElement);

const SolidChart = (): JSX.Element => {
    let canvas: HTMLCanvasElement;
    let chart: Chart;
    let intervalID: number;
    let x = 0;

    const data = {
        datasets: [{
            label: 'My First dataset',
            data: [{x: 0, y: 3}],
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

    onCleanup(() => clearInterval(intervalID));

    onMount(() => {
        chart = new Chart(canvas, config);

        intervalID = window.setInterval((): void => {
            if (x > 700) {
                window.clearInterval(intervalID);
                return;
            }

            for (let i = 0; i < 100; ++i) {
                config.data.datasets[0].data.push({ x: x++, y: Math.random() * 100 });
            }
    
            chart.options.scales!.x!.min = Math.max(0, x - 500);
            chart.options.scales!.x!.max = x;
            
            chart.update();
        }, 1000);
    });

    onCleanup(() => {
        chart.destroy();
    });

    return (
        <canvas ref={canvas!} />
    );
};

export default SolidChart;
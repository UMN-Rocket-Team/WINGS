import { createSignal, Match, onCleanup, onMount, Switch } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { ThemeContextValue, useTheme } from "./components/ThemeProvider";
import Modal from "./components/Modal";
import Credits from "./components/Credits";

function App() {
    const { theme, setTheme }: ThemeContextValue = useTheme();

    const [greetMsg, setGreetMsg] = createSignal("");
    const [name, setName] = createSignal("");

    const [isCreditModalVisible, setCreditModalVisible] = createSignal(false);

    async function greet() {
        setGreetMsg(await invoke("greet", { name: name() }));
    }

    let unlisten: UnlistenFn | null;

    onMount(async () => {
        unlisten = await listen("data-received", ({ payload }) => {
            console.log(`Data received: "${payload}"`);
        });
    });

    onCleanup(() => unlisten && unlisten());

    return (
        <div class="flex flex-col p-4 gap-4 dark:bg-dark-700 h-full">
            <div class="flex flex-row-reverse">
                <button class="p-2 border-none bg-transparent hover:bg-gray-200 hover:dark:bg-dark-900 border-rounded" 
                        onclick={() => setTheme(theme() === "light" ? "dark" : "light")}>
                    {/* For some reason, when the theme is changed, a match is removed for long enough that the page is re-layed-out when the button does not have an icon */}
                    {/* Workaround: place icons in a fixed-size container so no resize can occur */}
                    <div style={{ width: "28px", height: "28px" }}>
                        <Switch>
                            <Match when={theme() === "light"}>
                                {/* bxs:moon */}
                                <svg xmlns="http://www.w3.org/2000/svg" preserveAspectRatio="xMidYMid meet" viewBox="0 0 24 24"><path fill="currentColor" d="M12 11.807A9.002 9.002 0 0 1 10.049 2a9.942 9.942 0 0 0-5.12 2.735c-3.905 3.905-3.905 10.237 0 14.142c3.906 3.906 10.237 3.905 14.143 0a9.946 9.946 0 0 0 2.735-5.119A9.003 9.003 0 0 1 12 11.807z"/></svg>
                            </Match>
                            <Match when={theme() === "dark"}>
                                {/* bi:sun-fill */}
                                <svg xmlns="http://www.w3.org/2000/svg" preserveAspectRatio="xMidYMid meet" viewBox="0 0 16 16"><path fill="white" d="M8 12a4 4 0 1 0 0-8a4 4 0 0 0 0 8zM8 0a.5.5 0 0 1 .5.5v2a.5.5 0 0 1-1 0v-2A.5.5 0 0 1 8 0zm0 13a.5.5 0 0 1 .5.5v2a.5.5 0 0 1-1 0v-2A.5.5 0 0 1 8 13zm8-5a.5.5 0 0 1-.5.5h-2a.5.5 0 0 1 0-1h2a.5.5 0 0 1 .5.5zM3 8a.5.5 0 0 1-.5.5h-2a.5.5 0 0 1 0-1h2A.5.5 0 0 1 3 8zm10.657-5.657a.5.5 0 0 1 0 .707l-1.414 1.415a.5.5 0 1 1-.707-.708l1.414-1.414a.5.5 0 0 1 .707 0zm-9.193 9.193a.5.5 0 0 1 0 .707L3.05 13.657a.5.5 0 0 1-.707-.707l1.414-1.414a.5.5 0 0 1 .707 0zm9.193 2.121a.5.5 0 0 1-.707 0l-1.414-1.414a.5.5 0 0 1 .707-.707l1.414 1.414a.5.5 0 0 1 0 .707zM4.464 4.465a.5.5 0 0 1-.707 0L2.343 3.05a.5.5 0 1 1 .707-.707l1.414 1.414a.5.5 0 0 1 0 .708z"/></svg>
                            </Match>
                        </Switch>
                    </div>
                </button>
            </div>
            
            <h1 class="text-center">Welcome to Tauri!</h1>

            <div class="flex justify-center">
                <div>
                    <input
                        class="form-element mr-1.25"
                        onChange={(e) => setName(e.currentTarget.value)}
                        placeholder="Enter a name..."
                        />
                    <button type="button" onClick={() => greet()} class="form-element cursor-pointer hover:border-#396cd8">
                        Greet
                    </button>
                </div>
            </div>

            <p>{greetMsg}</p>

            <div class="flex justify-center items-center gap-2">
                <div class="flex items-center">
                    {/* mdi:web */}
                    <svg xmlns="http://www.w3.org/2000/svg" class="dark:text-white" width={28} preserveAspectRatio="xMidYMid meet" viewBox="0 0 24 24"><path fill="currentColor" d="M16.36 14c.08-.66.14-1.32.14-2c0-.68-.06-1.34-.14-2h3.38c.16.64.26 1.31.26 2s-.1 1.36-.26 2m-5.15 5.56c.6-1.11 1.06-2.31 1.38-3.56h2.95a8.03 8.03 0 0 1-4.33 3.56M14.34 14H9.66c-.1-.66-.16-1.32-.16-2c0-.68.06-1.35.16-2h4.68c.09.65.16 1.32.16 2c0 .68-.07 1.34-.16 2M12 19.96c-.83-1.2-1.5-2.53-1.91-3.96h3.82c-.41 1.43-1.08 2.76-1.91 3.96M8 8H5.08A7.923 7.923 0 0 1 9.4 4.44C8.8 5.55 8.35 6.75 8 8m-2.92 8H8c.35 1.25.8 2.45 1.4 3.56A8.008 8.008 0 0 1 5.08 16m-.82-2C4.1 13.36 4 12.69 4 12s.1-1.36.26-2h3.38c-.08.66-.14 1.32-.14 2c0 .68.06 1.34.14 2M12 4.03c.83 1.2 1.5 2.54 1.91 3.97h-3.82c.41-1.43 1.08-2.77 1.91-3.97M18.92 8h-2.95a15.65 15.65 0 0 0-1.38-3.56c1.84.63 3.37 1.9 4.33 3.56M12 2C6.47 2 2 6.5 2 12a10 10 0 0 0 10 10a10 10 0 0 0 10-10A10 10 0 0 0 12 2Z"/></svg>
                    <a href="https://rkt.aem.umn.edu/">Website</a>
                </div>
                {/* Veritcal line */}
                <hr style={{ "width": "0", "height": "100%", "margin": "0" }} />
                <button class="p-2 border-none bg-gray-200 hover:bg-gray-300 dark:bg-dark-900 hover:dark:bg-black dark:text-white border-rounded"
                        onClick={() => setCreditModalVisible(true)}>Credits</button>
                <Modal title="Credits" isVisible={isCreditModalVisible} setVisible={setCreditModalVisible}><Credits /></Modal>
            </div>
        </div>
    );
}

export default App;

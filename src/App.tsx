import { createSignal, onCleanup, onMount } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

function App() {
  const [greetMsg, setGreetMsg] = createSignal("");
  const [name, setName] = createSignal("");

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
    <div class="flex flex-col justify-center text-center m-0 pt-10vh">
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
    </div>
  );
}

export default App;

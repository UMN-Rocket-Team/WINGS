import { ModalProps } from "../core/ModalProvider";
import DefaultModalLayout from "../core/DefaultModalLayout";
import { createSignal, JSX, onMount, Show } from "solid-js";
import { displays, setDisplays, SettingsModalProps } from "../components/DisplaySettingsScreen";
import { DisplayStruct } from "../core/display_registry";
import { produce } from "solid-js/store";
import { store } from "../core/file_handling";
import settingsIcon from "../assets/settings.png";
import infoIcon from "../assets/info-sym.svg";

export class WebEmbedStruct implements DisplayStruct {
    displayName = 'Web Embed';
    packetID = -1;
    type = 'web_embed';
    url: string = 'https://example.com/';
    packetsDisplayed: boolean[] = [false];
}

// The modal that will be displayed to the user when editing a template type
const WebEmbedSettingsModal = (props: ModalProps<SettingsModalProps>): JSX.Element => {
    // Used to restore previous name when user enters something invalid
    let oldName = props.displayStruct.displayName;

    const [displaySettings, setDisplaySettings] = createSignal(false); // Are the modal settings (on the top left of the modal) being displayed?
    const [displayInfo, setDisplayInfo] = createSignal(false); // Is info about the display being displayed?
    
    let infoIconRef: HTMLImageElement | undefined;
    onMount(() => { // Events for hovering over info icon
        infoIconRef?.addEventListener("mouseout", (e) => {
            setDisplayInfo(false);
        });
        infoIconRef?.addEventListener("mouseover", (e) => {
            setDisplayInfo(true);
        });
    });

    /** HandleInput will handle updating the graphs name and also catches blank inputs and reverts to previous name */
    const handleInput = (event: Event) => {
        const newName = (event.target as HTMLElement).textContent || '';
        if (newName.trim() !== '') {
            setDisplayName(newName.trim(), props.index);
            oldName = newName.trim();
        } else {
            (event.target as HTMLElement).textContent = oldName;
        }
    };

    /* HandleKeyDown helps handle updating the graphName by preventing enters(newlines) */
    const handleKeyDown = (event: KeyboardEvent) => {
        if (event.key === 'Enter') {
            event.preventDefault();
        }
    };

    const setDisplayName = (newName: string, index: number) => {
        setDisplays(produce(s => {
            s[index].displayName = newName;
        }));
        store.set("display", displays);
    };

    const setURL = (url: string) => {
        setDisplays(produce(s => {
            (s[props.index] as WebEmbedStruct).url = url;
        }));
        store.set("display", displays);
    };

    const setRandomURL = (urls: string[]) => {
        const random = urls[Math.floor(Math.random() * urls.length)];
        setURL(random);
    };

    const deleteDisplay = () => {
        setDisplays(displays.filter((_, index) => index !== props.index));
        store.set("display", displays);
        props.closeModal({});
    };

    return <DefaultModalLayout close={() => props.closeModal({})} title="Customize Embed">
        <div class="flex flex-col bg-neutral-200 dark:bg-gray-700 p-4 rounded-lg relative min-w-fit">

            <Show when={displayInfo()}>
                <div class="absolute bg-neutral-300 top-[-1px] left-[-1px] dark:bg-neutral-700 p-4 rounded-3xl pt-12 z-[2]">
                    <p class="max-w-prose">This component embeds a remote website</p>
                </div>
            </Show>
            
            <div class='flex flex-row leading-none justify-between mb-4'>
                <img alt="Info" src={infoIcon} ref={infoIconRef} draggable={false} class="relative top-0 w-[23px] dark:invert z-[3]" />

                <h3 contenteditable={true} class="m-2 text-center font-bold w-[82%] absolute left-[50%] translate-x-[-50%]" 
                    onBlur={handleInput} onKeyDown={handleKeyDown}>
                    {props.displayStruct.displayName}
                </h3>

                <img alt="Settings" src={settingsIcon} draggable={false} onClick={() => setDisplaySettings(s => !s)} 
                    class="relative top-0 w-[25px] dark:invert z-[1] cursor-pointer" />
            </div>

            <button
                onClick={() => {
                    setRandomURL([
                        'https://www.youtube.com/embed/L_fcrOyoWZ8?autoplay=1&controls=0&start=24',
                        'https://www.youtube.com/embed/7ghSziUQnhs?autoplay=1&controls=0&start=20',
                        'https://www.youtube.com/embed/4GZRICFNeT0?autoplay=1&controls=0&start=48',
                    ]);
                }}
            >
                Random subway surfers
            </button>

            {/* <button
                onClick={() => {
                    setRandomURL([
                        'https://neal.fun/stimulation-clicker/',
                        'https://orteil.dashnet.org/cookieclicker/',
                    ]);
                }}
            >
                Random clicker game
            </button> */}

            <label>
                URL
                <input
                    type="text"
                    value={(props.displayStruct as WebEmbedStruct).url}
                    onInput={(e) => setURL(e.target.value)}
                />
            </label>

            <Show when={displaySettings()}>
                <div class="absolute bg-neutral-300 dark:bg-neutral-700 p-4 top-0 rounded-3xl right-0 z-[0]">
                    <div class="relative flex items-center justify-center mt-10">
                        <button
                            class="rounded-lg bg-red-500 hover:bg-red-600 flex items-center justify-center p-3"
                            onClick={() => {
                                deleteDisplay();
                            }}>
                            <h3>Remove Display</h3>
                        </button>
                    </div>
                </div>
            </Show>
        </div>
    </DefaultModalLayout>;
};

export default WebEmbedSettingsModal;
import {useNavigate} from "@solidjs/router";
import {Component} from "solid-js";
import Credits from "../components/Credits";
import {useModal} from "../core/ModalProvider";
import ThemeSwitcher from "../theme/ThemeSwitcher";
import logo from "../assets/logo.png";
import {open} from '@tauri-apps/api/dialog';
import {open as openHref} from '@tauri-apps/api/shell';
import {readTextFile} from "@tauri-apps/api/fs";
import {setParsedPackets} from "../backend_interop/buffers";
import {useBackend} from "../backend_interop/BackendProvider";
import {Packet, PacketStructureViewModel} from "../backend_interop/types";
import ErrorModal, {ErrorModalProps} from "../modals/ErrorModal";
import webIcon from "../assets/web.svg";

export type PacketBundle = {
    parsedPacketsArray: Packet[],
    PacketStructureViewModels: PacketStructureViewModel[]
};

/**
 * A component for the homepage/landing page of the application.
 *
 * Allows the user to:
 * - Change the theme
 * - create a new flight
 * - Open an existing flight
 * - Navigate to the UMN Rocket Team website
 * - View the credits for this application
 */
const Homepage: Component = () => {
    const {setPacketStructureViewModels} = useBackend();
    const navigate = useNavigate();
    const {showModal} = useModal();

    const loadFlight = async () => {
        const selectedFilePath = await open({
            title: "Load Flight",
            multiple: false,
            filters: [
                {name: "LoadFlight", extensions: ['json']}
            ]
        });

        if (selectedFilePath === null) {
            return;
        }

        readTextFile(selectedFilePath as string)
            .then((contents) => {
                const contentsJSON = JSON.parse(contents) as PacketBundle;
                setParsedPackets(contentsJSON.parsedPacketsArray);
                setPacketStructureViewModels(contentsJSON.PacketStructureViewModels);

                navigate("/savedFlight");
            })
            .catch((err) => showModal<ErrorModalProps, {}>(ErrorModal, {
                error: "Failed to Load Flight File",
                description: err
            }));
    }

    return (
        <div class="flex flex-col flex-grow p-4 gap-4 dark:bg-dark-700">
            <div class="flex flex-row-reverse">
                <ThemeSwitcher/>
            </div>
            <div class="flex flex-col items-center h-[100%] my-8 px-16 gap-4 bg-gray-100 dark:bg-gray-900 rounded-lg border-2 border-gray-200 dark:border-gray-800">
                <div class="flex items-center justify-start gap-4">
                    <img src={logo} class="h-[50%]" alt="Wings Logo" draggable={false} />
                    <span class="font-black text-[30vh] text-gray-900 dark:text-white">Wings</span>
                </div>
                <span class="dark:text-white text-center text-2xl">The Ground Station of the University of Minnesota Twin Cities Rocket Team</span>
                <div class="flex gap-4 flex-col md:flex-row">
                    <button class="bg-blue-500 text-white px-4 py-2 rounded-lg hover:bg-blue-600"
                            onClick={() => navigate("/newFlight")}>
                        Create New Flight
                    </button>
                    <button class="bg-blue-500 text-white px-4 py-2 rounded-lg hover:bg-blue-600" onClick={loadFlight}>
                        Load Flight File...
                    </button>
                </div>
            </div>
            <div class="flex w-full justify-center relative">
                <div class="flex justify-center gap-2 items-center">
                    <div class="flex justify-center gap-1 items-center">
                        <svg xmlns="http://www.w3.org/2000/svg" class="dark:text-white" width={24}
                             preserveAspectRatio="xMidYMid meet" viewBox="0 0 24 24">
                            <path fill="currentColor"
                                  d="M16.36 14c.08-.66.14-1.32.14-2c0-.68-.06-1.34-.14-2h3.38c.16.64.26 1.31.26 2s-.1 1.36-.26 2m-5.15 5.56c.6-1.11 1.06-2.31 1.38-3.56h2.95a8.03 8.03 0 0 1-4.33 3.56M14.34 14H9.66c-.1-.66-.16-1.32-.16-2c0-.68.06-1.35.16-2h4.68c.09.65.16 1.32.16 2c0 .68-.07 1.34-.16 2M12 19.96c-.83-1.2-1.5-2.53-1.91-3.96h3.82c-.41 1.43-1.08 2.76-1.91 3.96M8 8H5.08A7.923 7.923 0 0 1 9.4 4.44C8.8 5.55 8.35 6.75 8 8m-2.92 8H8c.35 1.25.8 2.45 1.4 3.56A8.008 8.008 0 0 1 5.08 16m-.82-2C4.1 13.36 4 12.69 4 12s.1-1.36.26-2h3.38c-.08.66-.14 1.32-.14 2c0 .68.06 1.34.14 2M12 4.03c.83 1.2 1.5 2.54 1.91 3.97h-3.82c.41-1.43 1.08-2.77 1.91-3.97M18.92 8h-2.95a15.65 15.65 0 0 0-1.38-3.56c1.84.63 3.37 1.9 4.33 3.56M12 2C6.47 2 2 6.5 2 12a10 10 0 0 0 10 10a10 10 0 0 0 10-10A10 10 0 0 0 12 2Z"/>
                        </svg>
                        <button
                            class="p-2 bg-gray-200 hover:bg-gray-300 dark:bg-gray-900 hover:dark:bg-gray-800 dark:text-white rounded-lg"
                            onclick={() => {
                                openHref("https://rkt.aem.umn.edu/").then(() => {});
                            }}>
                            Website
                        </button>
                    </div>
                    <button
                        class="p-2 bg-gray-200 hover:bg-gray-300 dark:bg-gray-900 hover:dark:bg-gray-800 dark:text-white rounded-lg"
                        onClick={() => showModal<{}, {}>(Credits, {})}>Credits
                    </button>
                </div>
            </div>
        </div>
    );
};

export default Homepage;

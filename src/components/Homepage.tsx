import {useNavigate} from "@solidjs/router";
import {Component} from "solid-js";
import Credits from "./Credits";
import {useModal} from "./ModalProvider";
import ThemeSwitcher from "./ThemeSwitcher";
import logo from "../assets/logo.png";
import {open} from '@tauri-apps/api/dialog';
import {readTextFile} from "@tauri-apps/api/fs";
import {setParsedPackets} from "../backend_interop/buffers";
import {useBackend} from "./BackendProvider";
import {Packet, PacketViewModel} from "../backend_interop/types";
import ErrorModal, {ErrorModalProps} from "./ErrorModal";
import webIcon from "../assets/web.svg";

export type PacketBundle = {
    parsedPacketsArray: Packet[],
    packetViewModels: PacketViewModel[]
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
    const {setPacketViewModels} = useBackend();
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
                setPacketViewModels(contentsJSON.packetViewModels);

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
            <div
                class="flex flex-col items-center ma py-8 px-16 gap-4 bg-gray-100 dark:bg-dark-800 border-rounded border-2 border-gray-200 dark:border-dark-900">
                <div class="flex items-center justify-start gap-4">
                    <img src={logo} class="h-20" alt="Wings Logo" draggable={false} />
                    <span class="font-black text-5xl dark:text-white">Wings</span>
                </div>
                <span class="dark:text-white text-center">The Ground Station of the University of Minnesota Twin Cities Rocket Team</span>
                <div class="flex gap-4 flex-col md:flex-row">
                    <button class="homePageButton"
                            onClick={() => navigate("/newFlight")}>
                        Create New Flight
                    </button>
                    <button class="homePageButton" onClick={loadFlight}>
                        {/* <Icon icon="mdi:file-import" width={28} height={28} class="dark:text-white" /> */}
                        Load Flight File...
                    </button>
                </div>
            </div>
            <div class="flex w-full justify-center relative">
                <div class="flex justify-center gap-2 items-center">
                    <div class="flex justify-center gap-1 items-center border-r-1 p-r-2">
                        <img src={webIcon} class="dark:invert w-6 h-6" draggable={false} />
                        <a href="https://rkt.aem.umn.edu/">Website</a>
                    </div>
                    <button
                        class="p-2 border-none bg-gray-200 hover:bg-gray-300 dark:bg-dark-900 hover:dark:bg-black dark:text-white border-rounded"
                        onClick={() => showModal<{}, {}>(Credits, {})}>Credits
                    </button>
                </div>
            </div>
        </div>
    );
};

export default Homepage;
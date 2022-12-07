import { useNavigate } from "@solidjs/router";
import { Component } from "solid-js";
import Credits from "./Credits";
import { useModal } from "./ModalProvider";
import ThemeSwitcher from "./ThemeSwitcher";
import logo from "../assets/logo.png";

const Homepage: Component = () => {
    const navigate = useNavigate();
    const { showModal } = useModal();

    return (
        <div class="flex flex-col flex-grow p-4 gap-4 dark:bg-dark-700">
            <div class="flex flex-row-reverse">
                <ThemeSwitcher />
            </div>
            <div class="flex flex-col items-center ma py-8 px-16 gap-4 bg-gray-100 dark:bg-dark-800 border-rounded border-2 border-gray-200 dark:border-dark-900">
                <div class="flex items-center justify-start gap-4">
                    <img src={logo} class="h-20"  alt="Wings Logo" />
                    <span class="font-black text-5xl dark:text-white">Wings</span>
                </div>
                <span class="dark:text-white text-center">The Ground Station of the University of Minnesota Twin Cities Rocket Team</span>
                <div class="flex gap-4 flex-col md:flex-row">
                    <button class="flex items-center gap-2 bg-blue-600 hover:bg-blue-700 text-white py-2 px-8 border-transparent border-rounded"
                            onClick={() => navigate("/newFlight")}>
                        {/* <Icon icon="mdi:file-import" width={28} height={28} class="dark:text-white" /> */}
                        Create New Flight
                    </button>
                    <button class="flex items-center gap-2 bg-blue-600 hover:bg-blue-700 text-white py-2 px-8 border-transparent border-rounded">
                        {/* <Icon icon="mdi:file-import" width={28} height={28} class="dark:text-white" /> */}
                        Load Flight File...
                    </button>
                    <button class="flex items-center gap-2 bg-blue-600 hover:bg-blue-700 text-white py-2 px-8 border-transparent border-rounded">
                        {/* <Icon icon="bi:collection-play-fill" width={28} height={28} class="dark:text-white" /> */}
                        Load Past Flight...
                    </button>
                    <button class="flex items-center gap-2 bg-blue-600 hover:bg-blue-700 text-white py-2 px-8 border-transparent border-rounded">
                        {/* <Icon icon="ri:live-fill" width={28} height={28} class="dark:text-white" /> */}
                        View Live Flight
                    </button>
                </div>
            </div>
            <div class="flex w-full justify-center relative">
                <div class="flex justify-center gap-2 items-center">
                    <div class="flex justify-center gap-1 items-center">
                        {/* mdi:web */}
                        <svg xmlns="http://www.w3.org/2000/svg" class="dark:text-white" width={24} preserveAspectRatio="xMidYMid meet" viewBox="0 0 24 24"><path fill="currentColor" d="M16.36 14c.08-.66.14-1.32.14-2c0-.68-.06-1.34-.14-2h3.38c.16.64.26 1.31.26 2s-.1 1.36-.26 2m-5.15 5.56c.6-1.11 1.06-2.31 1.38-3.56h2.95a8.03 8.03 0 0 1-4.33 3.56M14.34 14H9.66c-.1-.66-.16-1.32-.16-2c0-.68.06-1.35.16-2h4.68c.09.65.16 1.32.16 2c0 .68-.07 1.34-.16 2M12 19.96c-.83-1.2-1.5-2.53-1.91-3.96h3.82c-.41 1.43-1.08 2.76-1.91 3.96M8 8H5.08A7.923 7.923 0 0 1 9.4 4.44C8.8 5.55 8.35 6.75 8 8m-2.92 8H8c.35 1.25.8 2.45 1.4 3.56A8.008 8.008 0 0 1 5.08 16m-.82-2C4.1 13.36 4 12.69 4 12s.1-1.36.26-2h3.38c-.08.66-.14 1.32-.14 2c0 .68.06 1.34.14 2M12 4.03c.83 1.2 1.5 2.54 1.91 3.97h-3.82c.41-1.43 1.08-2.77 1.91-3.97M18.92 8h-2.95a15.65 15.65 0 0 0-1.38-3.56c1.84.63 3.37 1.9 4.33 3.56M12 2C6.47 2 2 6.5 2 12a10 10 0 0 0 10 10a10 10 0 0 0 10-10A10 10 0 0 0 12 2Z"/></svg>
                        <a href="https://rkt.aem.umn.edu/">Website</a>
                    </div>
                    {/* Vertical line */}
                    <hr style={{ "width": "0", "height": "100%", "margin": "0" }} />
                    <button class="p-2 border-none bg-gray-200 hover:bg-gray-300 dark:bg-dark-900 hover:dark:bg-black dark:text-white border-rounded"
                            onClick={() => showModal<{}, {}>(Credits, {})}>Credits</button>
                </div>
                <button class="absolute right-0 bottom-0 border-none bg-transparent hover:bg-gray-200 hover:dark:bg-dark-300 border-rounded">
                    {/* bxs:lock-alt */}
                    <svg xmlns="http://www.w3.org/2000/svg" class="dark:text-white" width={24} preserveAspectRatio="xMidYMid meet" viewBox="0 0 24 24"><path fill="currentColor" d="M20 12c0-1.103-.897-2-2-2h-1V7c0-2.757-2.243-5-5-5S7 4.243 7 7v3H6c-1.103 0-2 .897-2 2v8c0 1.103.897 2 2 2h12c1.103 0 2-.897 2-2v-8zM9 7c0-1.654 1.346-3 3-3s3 1.346 3 3v3H9V7z"/></svg>
                </button>
            </div>
        </div>
    );
};

export default Homepage;
import { Component, JSX } from "solid-js";
import { WebEmbedStruct } from "../modals/WebEmbedSettingsModal";

const WebEmbed: Component<WebEmbedStruct> = (video): JSX.Element => {
    return (
        <iframe
            // credentialless
            // anonymous
            class="w-full h-full"
            src={video.url}
            // frameborder="0"
            allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
            referrerpolicy="strict-origin-when-cross-origin"
            allowfullscreen
        ></iframe>
    );
};

export default WebEmbed;

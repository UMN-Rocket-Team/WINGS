import { Component, createEffect, JSX, onCleanup, onMount } from "solid-js";
import * as THREE from "three";
import { RocketStruct } from "../modals/RocketSettingsModal";
import { useBackend } from "../backend_interop/BackendProvider";
import { unDecimatedPackets } from "../backend_interop/buffers";

const FOV = 75; // degrees

interface RocketModel {
    bodyTubeRadius: number; // inches
    bodyTubeLength: number; // inches
    noseConeLength: number; // inches
    numFins: number;
    finThickness: number; // inches (usually just leave as some arbitrary small value eg. 0.01)
    finRootChord: number; // inches
    centerOfGravity: number; // inches from tip
    bodyTubeColor: number; // hex
    noseConeColor: number; // hex
    finColor: number; // hex
    backgroundColor: number; // hex
}

export const ROCKET_MODELS: Record<string, RocketModel> = {
    // see interface above for what the fields mean
    'irec-2025': {
        bodyTubeRadius: 6.2 / 2,
        bodyTubeLength: 110,
        noseConeLength: 40,
        numFins: 4,
        finThickness: 0.01,
        finRootChord: 14,
        centerOfGravity: 75.757,
        bodyTubeColor: 0x000000,
        noseConeColor: 0x000000,
        finColor: 0x000000,
        backgroundColor: 0xffffff,
    },
    'thomas-weber-gopher': {
        bodyTubeRadius: 3.15 / 2,
        bodyTubeLength: 37,
        noseConeLength: 5,
        numFins: 4,
        finThickness: 0.01,
        finRootChord: 3.5,
        centerOfGravity: 27.342,
        bodyTubeColor: 0xa97835,
        noseConeColor: 0xff00ff,
        finColor: 0xfff8dc,
        backgroundColor: 0xaaaaff,
    }
};

const RocketElement: Component<RocketStruct> = (rocket): JSX.Element => {
    const { parsedPacketCount } = useBackend();

    const model = ROCKET_MODELS[rocket.rocketModel];
    if (!model) {
        return <p>Unknown rocket model: {rocket.rocketModel}</p>;
    }

    let containerElement: HTMLDivElement;

    const scene = new THREE.Scene();
    const camera = new THREE.PerspectiveCamera(
        FOV, // vertical field of view
        1,   // aspect ratio (updated later in resize observer)
        0.1, // near clipping plane
        1000 // far clipping plane
    );
    const renderer = new THREE.WebGLRenderer({
        antialias: true
    });
    renderer.setClearColor(model.backgroundColor);

    // const controls = new OrbitControls(camera, renderer.domElement);

    // Set this up such that the origin is the top of the rocket
    // with the rocket's tube in the -y direction (like openrocket)
    const rocketGeometryGroup = new THREE.Group();
    // Rotate around the center of gravity.
    rocketGeometryGroup.position.y = model.centerOfGravity;

    // Use this to rotate the rocket
    const rocketRotationGroup = new THREE.Group();
    rocketRotationGroup.add(rocketGeometryGroup);
    scene.add(rocketRotationGroup);

    const bodyTubeMaterial = new THREE.MeshBasicMaterial({color: model.bodyTubeColor});
    const bodyTubeGeometry = new THREE.CylinderGeometry(
        model.bodyTubeRadius, // radius top
        model.bodyTubeRadius, // radius bottom
        model.bodyTubeLength, // height
        20, // radial segments (just enough to look smooth)
        1, // height segments (avoid unnecessary triangles)
        false, // solid ends (bottom might be visible)
    );
    bodyTubeGeometry.translate(0, model.bodyTubeLength / 2, 0);
    const bodyTube = new THREE.Mesh(bodyTubeGeometry, bodyTubeMaterial);
    bodyTube.position.y = -(model.noseConeLength + model.bodyTubeLength);
    rocketGeometryGroup.add(bodyTube);

    const noseConeMaterial = new THREE.MeshBasicMaterial({color: model.noseConeColor});
    const noseConeGeometry = new THREE.ConeGeometry(
        model.bodyTubeRadius, // radius
        model.noseConeLength, // height
        20, // radial segments (just enough to look smooth)
        1, // height segments (avoid unnecessary triangles)
        true, // open ended (avoid unnecessary triangles)
    );
    noseConeGeometry.translate(0, model.noseConeLength / 2, 0);
    const noseCone = new THREE.Mesh(noseConeGeometry, noseConeMaterial);
    noseCone.position.y = -(model.noseConeLength);
    rocketGeometryGroup.add(noseCone);

    const finMaterial = new THREE.MeshBasicMaterial({
        color: model.finColor,
        // The shape geometry is 2D, we want to be visible from both sides
        side: THREE.DoubleSide
    });
    const finShape = new THREE.Shape();
    finShape.moveTo(1 + 0.5, 0);
    finShape.lineTo(1 - 0.75, 1);
    finShape.lineTo(1 - 1.25, 1);
    finShape.lineTo(1 - 1, 0);
    const finMesh = new THREE.ExtrudeGeometry(finShape, {
        depth: model.finThickness,
        bevelEnabled: true,
        bevelSegments: 1,
        bevelSize: 0,
        bevelThickness: 0
    });
    finMesh.translate(0, 0, -model.finThickness / 2);
    finMesh.scale(model.finRootChord, 3.5, 3.5);
    finMesh.rotateZ(Math.PI / 2);

    // All the fins are the same geometry so instanced rendering (faster) can be used.
    const instancedFins = new THREE.InstancedMesh(finMesh, finMaterial, model.numFins);
    for (let i = 0; i < model.numFins; i++) {
        // This mesh is only used for doing matrix calculations
        const dummy = new THREE.Mesh();

        const angle = (2 * Math.PI / model.numFins) * i;
        dummy.rotateY(angle + Math.PI / 2);
        dummy.position.z = (model.bodyTubeRadius ) * Math.cos(angle);
        dummy.position.x = (model.bodyTubeRadius ) * Math.sin(angle);
        dummy.position.y = -(model.bodyTubeLength + model.noseConeLength);

        // Update the matrix for the instances mesh
        dummy.updateMatrix();
        instancedFins.setMatrixAt(i, dummy.matrix);
    }
    rocketGeometryGroup.add(instancedFins);

    // TODO: figure these out more generally ...
    camera.position.y = 5;
    camera.position.z = ((model.bodyTubeLength + model.noseConeLength) / 1.5) / Math.tan((FOV / 2) * Math.PI / 180);

    const updateFromPackets = () => {
        const packets = unDecimatedPackets[rocket.packetID];
        if (!packets) {
            return;
        }
        const lastPacket = packets[packets.length - 1];
        if (!lastPacket) {
            return;
        }

        const roll = rocket.fieldRoll === -1 ? 0 : lastPacket.fieldData[rocket.fieldRoll];
        const pitch = rocket.fieldPitch === -1 ? 0 : lastPacket.fieldData[rocket.fieldPitch];
        const yaw = rocket.fieldYaw === -1 ? 0 : lastPacket.fieldData[rocket.fieldYaw];

        // UFC sends angles in degrees, but we need radians
        rocketRotationGroup.rotation.x = roll * Math.PI / 180.0;
        rocketRotationGroup.rotation.y = yaw * Math.PI / 180.0;
        rocketRotationGroup.rotation.z = -pitch * Math.PI / 180.0;
    };

    createEffect(() => {
        // Update this effect whenever the parsed packet count changes (meaning new
        // packets got parsed)
        const _ignored = parsedPacketCount();
        updateFromPackets();
    });
    updateFromPackets();

    const animate = () => {
        renderer.render(scene, camera);
    };

    const resizeObserver = new ResizeObserver((changes) => {
        for (const change of changes) {
            const {width, height} = change.contentRect;
            camera.aspect = width / height;
            camera.updateProjectionMatrix();
            renderer.setSize(width, height);
        }

        // Avoid flicker after resizing
        animate();
    });

    onMount(() => {
        resizeObserver.observe(containerElement!);

        // prevent canvas size from affecting the size of the parent
        // element so that the parent element can still shrink
        renderer.domElement.style.position = 'absolute';

        containerElement!.appendChild(renderer.domElement);
        renderer.setAnimationLoop(animate);
    });

    onCleanup(() => {
        renderer.dispose();
    });

    return (
        <div
            class="h-full w-full gap-2 flex flex-col align-center justify-center text-center relative"
            ref={containerElement!}
        />
    );
};

export default RocketElement;

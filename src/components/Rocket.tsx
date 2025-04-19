import { Component, JSX, onCleanup, onMount } from "solid-js";
import * as THREE from "three";
import { RocketStruct } from "../modals/RocketSettingsModal";

const RocketElement: Component<RocketStruct> = (rocket): JSX.Element => {
    let containerElement: HTMLDivElement;

    const scene = new THREE.Scene();
    const camera = new THREE.PerspectiveCamera(
        75, // fov
        1, // aspect ratio (updated later in resize observer)
        0.1, // near clipping plane
        100 // far clipping plane
    );
    const renderer = new THREE.WebGLRenderer({
        antialias: true
    });

    const rocketGroup = new THREE.Group();
    scene.add(rocketGroup);

    // Measurements should be in inches
    const BODY_TUBE_RADIUS = 3.15 / 2;
    const BODY_TUBE_LENGTH = 37;
    const NOSE_CONE_LENGTH = 5;
    const NUM_FINS = 4;

    const BODY_TUBE_COLOR = 0xA97835;
    const NOSE_CONE_COLOR = 0xff00ff;
    const FIN_COLOR = 0xfff8dc;

    const bodyTubeMaterial = new THREE.MeshBasicMaterial({color: BODY_TUBE_COLOR});
    const bodyTubeGeometry = new THREE.CylinderGeometry(
        BODY_TUBE_RADIUS, // radius top
        BODY_TUBE_RADIUS, // radius bottom
        BODY_TUBE_LENGTH, // height
        20, // radial segments (just enough to look smooth)
        1, // height segments (avoid unnecessary triangles)
        true, // open ended (avoid unnecessary triangles)
    );
    bodyTubeGeometry.translate(0, BODY_TUBE_LENGTH / 2, 0);
    const bodyTube = new THREE.Mesh(bodyTubeGeometry, bodyTubeMaterial);
    bodyTube.position.y = -(NOSE_CONE_LENGTH + BODY_TUBE_LENGTH);
    rocketGroup.add(bodyTube);

    const noseConeMaterial = new THREE.MeshBasicMaterial({color: NOSE_CONE_COLOR});
    const noseConeGeometry = new THREE.ConeGeometry(
        BODY_TUBE_RADIUS, // radius
        NOSE_CONE_LENGTH, // height
        20, // radial segments (just enough to look smooth)
        1, // height segments (avoid unnecessary triangles)
        true, // open ended (avoid unnecessary triangles)
    );
    noseConeGeometry.translate(0, NOSE_CONE_LENGTH / 2, 0);
    const noseCone = new THREE.Mesh(noseConeGeometry, noseConeMaterial);
    noseCone.position.y = -(NOSE_CONE_LENGTH);
    rocketGroup.add(noseCone);

    const finMaterial = new THREE.MeshBasicMaterial({
        color: FIN_COLOR,
        // The shape geometry is 2D, we want to be visible from both sides
        side: THREE.DoubleSide
    });
    const finShape = new THREE.Shape();
    finShape.moveTo(0, 0);
    finShape.lineTo(0.25, 1);
    finShape.lineTo(0.75, 1);
    finShape.lineTo(1, 0);
    const finGeometry = new THREE.ShapeGeometry(finShape);
    finGeometry.scale(3.5, 3.5, 3.5);
    finGeometry.rotateZ(Math.PI / 2);

    // All the fins are the same geometry so instanced rendering (faster) can be used.
    const instancedFins = new THREE.InstancedMesh(finGeometry, finMaterial, NUM_FINS);
    for (let i = 0; i < NUM_FINS; i++) {
        // This mesh is only used for doing matrix calculations
        const dummy = new THREE.Mesh();

        const angle = (2 * Math.PI / NUM_FINS) * i;
        dummy.rotateY(angle + Math.PI / 2);
        dummy.position.z = BODY_TUBE_RADIUS * Math.cos(angle);
        dummy.position.x = BODY_TUBE_RADIUS * Math.sin(angle);
        dummy.position.y = -(BODY_TUBE_LENGTH + NOSE_CONE_LENGTH);

        // Update the matrix for the instances mesh
        dummy.updateMatrix();
        instancedFins.setMatrixAt(i, dummy.matrix);
    }
    rocketGroup.add(instancedFins);

    camera.position.y = -20;
    camera.position.z = 40;

    const animate = () => {
        rocketGroup.rotation.y += 0.01;

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

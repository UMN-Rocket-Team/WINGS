import { Component, JSX, onCleanup, onMount } from "solid-js";
import * as THREE from "three";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls.js";
import { RocketStruct } from "../modals/RocketSettingsModal";

const RocketElement: Component<RocketStruct> = (rocket): JSX.Element => {
    // Physical dimensions. Lengths in inches.
    const BODY_TUBE_RADIUS = 3.15 / 2;
    const BODY_TUBE_LENGTH = 37;
    const NOSE_CONE_LENGTH = 5;
    const NUM_FINS = 4;
    const FIN_WIDTH = 0.1;

    // In inches from tip of nose cone.
    const CENTER_OF_GRAVITY = 27.342;

    // Colors, in hex 0xRRGGBB
    const BODY_TUBE_COLOR = 0xa97835;
    const NOSE_CONE_COLOR = 0xff00ff;
    const FIN_COLOR = 0xfff8dc;
    const BACKGROUND_COLOR = 0xaaaaff;

    let containerElement: HTMLDivElement;

    const scene = new THREE.Scene();
    const camera = new THREE.PerspectiveCamera(
        75,  // fov
        1,   // aspect ratio (updated later in resize observer)
        0.1, // near clipping plane
        100  // far clipping plane
    );
    const renderer = new THREE.WebGLRenderer({
        antialias: true
    });
    renderer.setClearColor(BACKGROUND_COLOR);

    const controls = new OrbitControls(camera, renderer.domElement);

    // Set this up such that the origin is the top of the rocket
    // with the rocket's tube in the -y direction (like openrocket)
    const rocketGeometryGroup = new THREE.Group();
    // Rotate around the center of gravity.
    rocketGeometryGroup.position.y = CENTER_OF_GRAVITY;

    // Use this to rotate the rocket
    const rocketRotationGroup = new THREE.Group();
    rocketRotationGroup.add(rocketGeometryGroup);
    scene.add(rocketRotationGroup);

    const bodyTubeMaterial = new THREE.MeshBasicMaterial({color: BODY_TUBE_COLOR});
    const bodyTubeGeometry = new THREE.CylinderGeometry(
        BODY_TUBE_RADIUS, // radius top
        BODY_TUBE_RADIUS, // radius bottom
        BODY_TUBE_LENGTH, // height
        20, // radial segments (just enough to look smooth)
        1, // height segments (avoid unnecessary triangles)
        false, // solid ends (bottom might be visible)
    );
    bodyTubeGeometry.translate(0, BODY_TUBE_LENGTH / 2, 0);
    const bodyTube = new THREE.Mesh(bodyTubeGeometry, bodyTubeMaterial);
    bodyTube.position.y = -(NOSE_CONE_LENGTH + BODY_TUBE_LENGTH);
    rocketGeometryGroup.add(bodyTube);

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
    rocketGeometryGroup.add(noseCone);

    const finMaterial = new THREE.MeshBasicMaterial({
        color: FIN_COLOR,
        // The shape geometry is 2D, we want to be visible from both sides
        side: THREE.DoubleSide
    });
    const finShape = new THREE.Shape();
    finShape.moveTo(1 + 0.5, 0);
    finShape.lineTo(1 - 0.75, 1);
    finShape.lineTo(1 - 1.25, 1);
    finShape.lineTo(1 - 1, 0);
    const finMesh = new THREE.ExtrudeGeometry(finShape, {
        depth: FIN_WIDTH,
        bevelEnabled: true,
        bevelSegments: 1,
        bevelSize: 0,
        bevelThickness: 0
    });
    finMesh.translate(0, 0, -FIN_WIDTH / 2);
    finMesh.scale(3.5, 3.5, 3.5);
    finMesh.rotateZ(Math.PI / 2);

    // All the fins are the same geometry so instanced rendering (faster) can be used.
    const instancedFins = new THREE.InstancedMesh(finMesh, finMaterial, NUM_FINS);
    for (let i = 0; i < NUM_FINS; i++) {
        // This mesh is only used for doing matrix calculations
        const dummy = new THREE.Mesh();

        const angle = (2 * Math.PI / NUM_FINS) * i;
        dummy.rotateY(angle + Math.PI / 2);
        dummy.position.z = (BODY_TUBE_RADIUS ) * Math.cos(angle);
        dummy.position.x = (BODY_TUBE_RADIUS ) * Math.sin(angle);
        dummy.position.y = -(BODY_TUBE_LENGTH + NOSE_CONE_LENGTH);

        // Update the matrix for the instances mesh
        dummy.updateMatrix();
        instancedFins.setMatrixAt(i, dummy.matrix);
    }
    rocketGeometryGroup.add(instancedFins);

    camera.position.y = 0;
    camera.position.z = 40;

    const animate = () => {
        // rocketRotationGroup.rotation.x += 0.01;
        rocketRotationGroup.rotation.y += 0.01;
        // rocketRotationGroup.rotation.z += 0.01;

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

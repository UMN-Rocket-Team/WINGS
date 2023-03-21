import { Packet, PacketData } from "./types";

export const parsedPackets: Record<number, PacketData[]> = [];

export const pushParsedPackets = (packets: Packet[]): void => {
    let sortedNewParsedPackets: Record<number, PacketData[]> = [];

    for (const packet of packets) {
        if (sortedNewParsedPackets[packet.structureId] === undefined) {
            sortedNewParsedPackets[packet.structureId] = [];
        }
        const packetData: PacketData = { fieldData: packet.fieldData, timestamp: packet.timestamp };
        sortedNewParsedPackets[packet.structureId].push(packetData);
    }

    for (const structureId in sortedNewParsedPackets) {
        if (parsedPackets[structureId] === undefined) {
            parsedPackets[structureId] = [];
        }
        parsedPackets[structureId].push(...sortedNewParsedPackets[structureId]);
    }
};
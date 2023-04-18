import { Packet, PacketData } from "./types";

/**
 * The global map between packet ids and the list of received packet data for the packet with that id
 */
export const parsedPackets: Record<number, PacketData[]> = [];

/**
 * Inserts the given list of parsed packets into the global map ({@link parsedPackets}).
 * 
 * @param packets the newly parsed packets to insert into the global map
 */
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
        if (parsedPackets[+structureId] === undefined) {
            parsedPackets[+structureId] = [];
        }
        parsedPackets[+structureId].push(...sortedNewParsedPackets[structureId]);
    }
};

const clearParsedPackets = (): void => {
    for (const structureId in parsedPackets) {
        delete parsedPackets[+structureId];
    }
}

export const setParsedPackets = (packets: Packet[]): void => {
    clearParsedPackets();
    pushParsedPackets(packets);
}
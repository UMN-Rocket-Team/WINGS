import { Packet } from "./types";

let unparsedPackets: Packet[] = [];

export const pushUnparsedPackets = (packets: Packet[]) => unparsedPackets = [ ...unparsedPackets, ...packets ];

export const popUnparsedPackets = (): Packet[] => {
    const packetsToPop = unparsedPackets;
    unparsedPackets = [];
    return packetsToPop;
}
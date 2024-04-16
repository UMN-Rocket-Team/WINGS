import { Packet, PacketData } from "./types";

/**
 * The global map between packet ids and the list of received packet data for the packet with that id
 */
export const parsedPackets: Record<number, PacketData[]> = [];

/**
 * Global variables
 */
let ptr1: number = 1;
let ptr2: number = 1;
let wall: number = 500;
let multiple: number = 2;
let next: number = 1;

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
        const packetData: PacketData = {
            fieldData: packet.fieldData,
            metaData: packet.metaData
        };
        sortedNewParsedPackets[packet.structureId].push(packetData);
    }

    for (const structureId in sortedNewParsedPackets) {
        if (parsedPackets[+structureId] === undefined) {
            parsedPackets[+structureId] = [];
        }
        if (ptr2 + sortedNewParsedPackets[structureId].length > (2 * wall)) {
            // Need to do a while loop here to add the elements to the end of the array till it hits 2wall
            // This loop will only run once, when we are at the max capacity of the array for the first time
            var ctr = 0;
            while (ptr2 < (2 * wall)) {
                parsedPackets[+structureId].push(sortedNewParsedPackets[structureId][ctr]);
                ptr2++;
                ctr++;
            }

            // ptr2 = 2wall
            // ctr = if we have 3 packets to add, and we have ptr2 = 999 before, we add 1 packet and have 2 left and ctr = 1 so basically
            // ctr = number of packets added and the index for the next packet to add
            // packets_left = sortedNewParsedPackets[structureId].length - ctr (3 - 1 = 2) perfect
            var packets_left = sortedNewParsedPackets[structureId].length - ctr;

            // At this point ptr2 is at 2wall == 1000 and we need to start the decimation process
            // While this function is not the most efficient, it is the most readable and the most correct without adding extra complexity
            // The variable next is only used in this for loop and nowhere else, so it is safe to use it here
            // Next is initialized to 1 and is used to keep track of the count till next packet to add, it is always updated to multiple/2 and multiple is always a power of 2
            for (let i = 0; i < packets_left; i++) {
                if (next > 0) {
                    // While next is not 0, we are skipping a packet in parsed packets by incrementing ctr and decrementing next
                    ctr++;
                    next--;
                } else { // next == 0 (we are adding a packet to the end and removing a packet from the starting half)
                    parsedPackets[+structureId].splice(ptr1, 1); // remove a packet from the starting half of the array
                    ptr1++;
                    if (ptr1 >= wall) {
                        ptr1 = 1;
                        multiple = multiple * 2;
                    }
                    parsedPackets[+structureId].push(sortedNewParsedPackets[structureId][ctr]); // add a packet to the end of the array

                    ctr++; // increment ctr
                    next = multiple / 2; // resets next
                }
            }
        }
        ptr2 = parsedPackets[+structureId].length; // ptr2 should basically always be the same as 2wall once we reach max capacitity, but good to have this anyways
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
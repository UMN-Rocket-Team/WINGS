import { Packet, PacketData } from "./types";
import { createSignal } from "solid-js";

/**
 * The global map between packet ids and the list of received packet data for the packet with that id
 */
export const parsedPackets: Record<number, PacketData[]> = [];

export const lastParsedPacket: Record<number, PacketData> = [];

/**
 * Global variables
 */
type PacketStructureId = {
    structureId: string,
    ptr1: number,
    ptr2: number,
    wall: number,
    multiple: number,
    next: number
}



const decVars: Record<number, PacketStructureId> = [];

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
        // 
        if (parsedPackets[+structureId] === undefined) {
            let decimationVars: PacketStructureId = {
                structureId: structureId,
                ptr1: 1,
                ptr2: 1,
                wall: 50,
                multiple: 2,
                next: 1
            }
            parsedPackets[+structureId] = [];
            decVars[+structureId] = decimationVars;
        }

        // 

        if (decVars[+structureId].ptr2 + sortedNewParsedPackets[+structureId].length > (2 * decVars[+structureId].wall)) {
            // Need to do a while loop here to add the elements to the end of the array till it hits 2wall
            // This loop will only run once, when we are at the max capacity of the array for the first time
            var ctr = 0;
            while (decVars[+structureId].ptr2 < (2 * decVars[+structureId].wall)) {
                lastParsedPacket[+structureId] = sortedNewParsedPackets[+structureId][ctr]; // For Readout.tsx
                
                parsedPackets[+structureId].push(sortedNewParsedPackets[+structureId][ctr]);
                decVars[+structureId].ptr2++;
                ctr++;
            }

            // ptr2 = 2wall
            // ctr = if we have 3 packets to add, and we have ptr2 = 999 before, we add 1 packet and have 2 left and ctr = 1 so basically
            // ctr = number of packets added and the index for the next packet to add
            // packets_left = sortedNewParsedPackets[structureId].length - ctr (3 - 1 = 2) perfect
            var packets_left = sortedNewParsedPackets[+structureId].length - ctr;

            // At this point ptr2 is at 2wall == 1000 and we need to start the decimation process
            // While this function is not the most efficient, it is the most readable and the most correct without adding extra complexity
            // The variable next is only used in this for loop and nowhere else, so it is safe to use it here
            // Next is initialized to 1 and is used to keep track of the count till next packet to add, it is always updated to multiple/2 and multiple is always a power of 2
            for (let i = 0; i < packets_left; i++) {
                if (decVars[+structureId].next > 0) {
                    lastParsedPacket[+structureId] = sortedNewParsedPackets[+structureId][ctr]; // For Readout.tsx

                    // While next is not 0, we are skipping a packet in parsed packets by incrementing ctr and decrementing next
                    ctr++;
                    decVars[+structureId].next--;
                } else { // next == 0 (we are adding a packet to the end and removing a packet from the starting half)
                    
                    lastParsedPacket[+structureId] = sortedNewParsedPackets[+structureId][ctr]; // For Readout.tsx

                    parsedPackets[+structureId].splice(decVars[+structureId].ptr1, 1); // remove a packet from the starting half of the array
                    // console.log("deleted packet");
                    decVars[+structureId].ptr1++;
                    if (decVars[+structureId].ptr1 >= decVars[+structureId].wall) {
                        decVars[+structureId].ptr1 = 1;
                        decVars[+structureId].multiple  *= 2;
                    }
                    parsedPackets[+structureId].push(sortedNewParsedPackets[+structureId][ctr]); // add a packet to the end of the array
                    // console.log("added packet");
                    ctr++; // increment ctr
                    decVars[+structureId].next = decVars[+structureId].multiple / 2; // resets next
                }
            }
        } else {
            // If we have enough space to add all the packets from the new parsed packets
            // We just add them to the end of the array
            parsedPackets[+structureId].push(...sortedNewParsedPackets[structureId]);

            lastParsedPacket[+structureId] = sortedNewParsedPackets[+structureId][sortedNewParsedPackets[+structureId].length - 1]; // For Readout.tsx
        }
        // console.log(parsedPackets[+structureId].length)
        decVars[+structureId].ptr2 = parsedPackets[+structureId].length; // ptr2 should basically always be the same as 2wall once we reach max capacitity, but good to have this anyways
        
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
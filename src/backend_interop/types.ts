import { PacketFieldType, RustPacketFieldType } from "../core/packet_field_type";

export type RustRefreshAndReadResult = {
    new_available_port_names: SerialPortNames[] | null,
    parsed_packets: RustPacket[] | null,
};

export type RefreshAndReadResult = {
    new_available_port_names: SerialPortNames[] | null,
    parsed_packets: Packet[] | null,
}

export type SerialPortNames = {
    /**
     * The name of the serial port. On Windows, can be `COM[0-9]+`. On Unix, can be a file path like `/dev/ttyUSB[0-9]+`.
     * 
     * Used to identify a serial port to the backend.
     */
    name: string,
    manufacturer_name: string | null,
    product_name: string | null,
};

export type RustPacket = {
    structure_id: number,
    field_data: PacketFieldValue[],
    timestamp: number,
};

export type Packet = PacketData & {
    structureId: number,
};

export type PacketData = {
    fieldData: number[],
    timestamp: number,
};

export type PacketFieldValue = UnsignedByte | SignedByte | UnsignedShort | SignedShort | UnsignedInteger | SignedInteger | UnsignedLong | SignedLong | Float | Double;

type UnsignedByte = {
    unsignedByte: number,
};
type SignedByte = {
    signedByte: number,
};
type UnsignedShort = {
    unsignedShort: number,
};
type SignedShort = {
    signedShort: number,
};
type UnsignedInteger = {
    unsignedInteger: number,
};
type SignedInteger = {
    signedInteger: number,
};
type UnsignedLong = {
    unsignedLong: number,
};
type SignedLong = {
    signedLong: number,
};
type Float = {
    float: number,
};
type Double = {
    double: number,
};

export enum PacketComponentType {
    Field = 0,
    Delimiter = 1,
    Gap = 2,
};

export type RustPacketViewModel = {
    id: number,
    name: string,
    components: RustPacketComponent[],
};

type RustPacketComponent = {
    Field: RustPacketField;
} | {
    Delimiter: RustPacketDelimiter;
} | {
    Gap: PacketGap;
};

export type RustPacketField = {
    index: number,
    metadata_type: PacketMetadataType;
    name: string;
    offset_in_packet: number;
    type: RustPacketFieldType;
};

export type RustPacketDelimiter = {
    index: number;
    name: string,
    identifier: number[],
    offset_in_packet: number,
};

export type PacketViewModel = {
    id: number,
    name: string,
    components: PacketComponent[],
};

export type PacketComponent = {
    type: PacketComponentType;
    data: PacketField | PacketDelimiter | PacketGap;
};

export type PacketField = {
    index: number;
    name: string,
    type: PacketFieldType,
    offsetInPacket: number,
    metadataType: PacketMetadataType
};

export enum PacketMetadataType {
    None = "None",
    Timestamp = "Timestamp",
};

export type PacketDelimiter = {
    index: number;
    name: string,
    identifier: string,
    offsetInPacket: number,
};

export type PacketGap = {
    index: number,
    size: number,
    offset: number,
};

export type RadioTestResult = {
    packets_attempted: number;
    packets_written: number;
    packets_read: number;
};
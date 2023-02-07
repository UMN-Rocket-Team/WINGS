export type RefreshAndReadResult = {
    newAvailablePortNames: SerialPortNames[] | null,
    parsedPackets: Packet[] | null,
}

export type SerialPortNames = {
    /**
     * The name of the serial port. On Windows, can be `COM[0-9]+`. On Unix, can be a file path like `/dev/ttyUSB[0-9]+`.
     * 
     * Used to identify a serial port to the backend.
     */
    name: string,
    manufacturerName: string | null,
    productName: string | null,
};

export type Packet = PacketData & {
    structureId: number,
};

export type PacketData = {
    fieldData: number[],
    timestamp: number,
};

export type PacketFieldValue = {
    type: PacketFieldType,
    data: number,
};

export enum PacketComponentType {
    Field = "Field",
    Delimiter = "Delimiter",
    Gap = "Gap",
};

export enum PacketFieldType {
    UnsignedByte = "Unsigned Byte",
    SignedByte = "Signed Byte",
    UnsignedShort = "Unsigned Short",
    SignedShort = "Signed Short",
    UnsignedInteger = "Unsigned Integer",
    SignedInteger = "Signed Integer",
    UnsignedLong = "Unsigned Long",
    SignedLong = "Signed Long",
    Float = "Float",
    Double = "Double"
};

export type PacketViewModel = {
    id: number,
    name: string,
    components: PacketComponent[],
};

export type PacketComponent = {
    type: PacketComponentType,
    data: PacketField | PacketDelimiter | PacketGap,
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
    index: number,
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
    packetsAttempted: number,
    packetsWritten: number,
    packetsRead: number,
};
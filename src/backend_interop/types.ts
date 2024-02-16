/**
 * The type of the payload of the `serial-update` event brodcast by the backend.
 */
export type SerialUpdateResult = {
    /**
     * The list of available serial ports
     */
    newAvailablePortNames: SerialPortNames[] | null,
    /**
     * The list of new parsed packets
     */
    parsedPackets: Packet[] | null,
}

/**
 * The names of a serial port.
 */
export type SerialPortNames = {
    /**
     * The name of the serial port. On Windows, can be `COM[0-9]+`. On Unix, can be a file path like `/dev/ttyUSB[0-9]+`.
     * 
     * Used to identify a serial port to the backend.
     */
    name: string,
    /**
     * The name of the manufacturer of either the port or device
     */
    manufacturerName: string | null,
    /**
     * The name of the product
     */
    productName: string | null,
};

/**
 * DataPackets formatted to be used by graphs and other forms of display
 */
export type Packet = PacketData & {
    structureId: number,
};

/**
 * The type of unidentified radio data
 */
export type PacketData = {
    fieldData: number[],
    metaData: number[]
};

/**
 * The type of each field value inside of a parsed packet.
 */


/**
 * An enumeration of all the possible types of {@link PacketComponent}s
 */
export enum PacketComponentType {
    Field = "Field",
    Delimiter = "Delimiter",
    Gap = "Gap",
};

/**
 * An enumeration of all the supported types for {@link PacketField}s
 */
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

/**
 * An enumeration of all the supported types for {@link PacketStructureViewModelUpdate}s
 */
export enum PacketStructureViewModelUpdateType {
    CreateOrUpdate = "CreateOrUpdate",
    Delete = "Delete"
};

/**
 * A "tagged union" containing updated data for this packet view model
 */
export type PacketStructureViewModelUpdate = {
    /**
     * The type of this packet view model update
     */
    type: PacketStructureViewModelUpdateType,
    /**
     * If this packet view model update is for a created or modified packet view model, the new packet view model.
     * If this packet view model update is for a deleted packet view model, the id of the deleted packet view model.
     */
    data: PacketStructureViewModel | number,
};

/**
 * The type of a view model for a backend packet structure
 */
export type PacketStructureViewModel = {
    /**
     * The identifier of this packet structure
     */
    id: number,
    /**
     * The name of the packet structure
     */
    name: string,
    /**
     * An ascending-ordered list of components inside this packet structure
     */
    components: PacketComponent[],
};

/**
 * A "tagged union" containing data for this packet component whether it is a packet field, delimiter, or gap
 */
export type PacketComponent = {
    /**
     * The type of this packet component
     */
    type: PacketComponentType,
    /**
     * The type-specific data for this packet component
     */
    data: PacketField | PacketDelimiter | PacketGap,
};

/**
 * The type of a view model into a location in a packet containing data
 */
export type PacketField = {
    /**
     * The index of this packet inside its packet structure
     */
    index: number;
    /**
     * The name assigned to this packet
     */
    name: string,
    /**
     * The data type this packet's data is in
     */
    type: PacketFieldType,
    /**
     * The byte offset of this packet inside its packet structure
     */
    offsetInPacket: number,
    /**
     * The metadata type of this packet for special parsing rules 
     */
    metadataType: PacketMetadataType
};

/**
 * An enumeration for the possible special parsing rules for a {@link PacketField}
 */
export enum PacketMetadataType {
    None = "None",
    Timestamp = "Timestamp",
};

/**
 * The type of a view model into a location in a packet containing an packet-specific identifier
 */
export type PacketDelimiter = {
    /**
     * The index of this delimiter inside its packet structure
     */
    index: number,
    /**
     * The name assigned to this delimiter
     */
    name: string,
    /**
     * A hex string representing the raw bytes of this delimiter's identifier
     */
    identifier: string,
    /**
     * The byte offset of this delimiter inside its packet structure
     */
    offsetInPacket: number,
};

/**
 * The type of a view model into an empty location in a packet
 */
export type PacketGap = {
    /**
     * The index of this gap inside its packet structure
     */
    index: number,
    /**
     * The size in bytes of this gap
     */
    size: number,
    /**
     * The byte offset of this delimiter inside its packet structure
     */
    offset: number,
};

/**
 * State sent from backend by the sending loop.
 */
export type SendingLoopState = {
    /**
     * Number of test packets that have been sent.
     */
    packetsSent: number,
};

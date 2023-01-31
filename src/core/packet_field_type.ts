export enum RustPacketFieldType {
    UnsignedByte = "UnsignedByte",
    SignedByte = "SignedByte",
    UnsignedShort = "UnsignedShort",
    SignedShort = "SignedShort",
    UnsignedInteger = "UnsignedInteger",
    SignedInteger = "SignedInteger",
    UnsignedLong = "UnsignedLong",
    SignedLong = "SignedLong",
    Float = "Float",
    Double = "Double",
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

export const toPacketFieldType = (rustPacketFieldType: RustPacketFieldType): PacketFieldType => {
    switch (rustPacketFieldType) {
        case RustPacketFieldType.SignedByte:
            return PacketFieldType.SignedByte;
        case RustPacketFieldType.UnsignedByte:
            return PacketFieldType.UnsignedByte;
        case RustPacketFieldType.SignedShort:
            return PacketFieldType.SignedShort;
        case RustPacketFieldType.UnsignedShort:
            return PacketFieldType.UnsignedShort;
        case RustPacketFieldType.SignedInteger:
            return PacketFieldType.SignedInteger;
        case RustPacketFieldType.UnsignedInteger:
            return PacketFieldType.UnsignedInteger;
        case RustPacketFieldType.SignedLong:
            return PacketFieldType.SignedLong;
        case RustPacketFieldType.UnsignedLong:
            return PacketFieldType.UnsignedLong;
        case RustPacketFieldType.Float:
            return PacketFieldType.Float;
        case RustPacketFieldType.Double:
            return PacketFieldType.Double;
        default:
            throw new Error(`${rustPacketFieldType} is not a recognized RustPacketFieldType!`);
    }
};

export const toRustPacketFieldType = (packetFieldType: PacketFieldType): RustPacketFieldType => {
    return packetFieldType.replace(" ", "") as unknown as RustPacketFieldType;
};
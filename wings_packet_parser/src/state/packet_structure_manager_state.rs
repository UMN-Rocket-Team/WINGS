use crate::{
    models::packet_structure::PacketStructure, packet_structure_manager::PacketStructureManager,
};

pub fn default_packet_structure_manager() -> PacketStructureManager {
    let mut packet_structure_manager = PacketStructureManager::default();

    let mut daq_structure = PacketStructure::default();
    daq_structure.ez_make(
        "4869205468657265 F64 F64 F64 F64 F64 476f6f6462796521",
        &["Timestamp", "LoadCell", "Pressure", "Temp1", "Temp2"],
        false,
    );
    daq_structure.name = "daq".to_owned();
    packet_structure_manager
        .register_packet_structure(&mut daq_structure)
        .expect("Failed to register daq packet");

    let mut daq_structure = PacketStructure::default();
    daq_structure.ez_make(
        "aa692054686572aa F64 F64 F64 F64 F64 F64 aa6f6f64627965aa",
        &[
            "Time",
            "PSI",
            "Newtons",
            "Impulse",
            "Burn_time",
            "Max_pressure",
        ],
        false,
    );
    daq_structure.name = "daq_adv".to_owned();
    packet_structure_manager
        .register_packet_structure(&mut daq_structure)
        .expect("Failed to register daq packet");

    let mut draw_structure = PacketStructure::default();
    draw_structure.ez_make(
        "ba5eba11 d5a1d5a1 F64 F64 F64 d5a1d5a1 ca11ab1e",
        &["Timestamp", "rkt_speed", "rkt_speed_also"],
        false,
    );
    draw_structure.name = "draw".to_owned();
    packet_structure_manager
        .register_packet_structure(&mut draw_structure)
        .expect("Failed to register draw packet");

    //################################
    //Leep Hardcoded packets start here
    //################################
    let mut leep_gps_data_structure = PacketStructure::default();
    leep_gps_data_structure.ez_make(
        "b5a6 u32 06 u8 u8 F32 F32 F32 F32",
        &[
            "Timestamp",
            "fixType",
            "satsInView",
            "GPS_TimeStamp",
            "lat",
            "long",
            "altitude",
        ],
        true,
    );
    leep_gps_data_structure.name = "leep_gps".to_owned();
    packet_structure_manager
        .register_packet_structure(&mut leep_gps_data_structure)
        .expect("Failed to register leep gps data packet");

    let mut leep_volt_data_structure = PacketStructure::default();
    leep_volt_data_structure.ez_make("b5a6 u32 07 F32", &["Timestamp", "voltage"], true);
    leep_volt_data_structure.name = "leep_volt".to_owned();
    packet_structure_manager
        .register_packet_structure(&mut leep_volt_data_structure)
        .expect("Failed to register leep gps data packet");
    //################################
    //UFC Hardcoded packets start here
    //################################

    let mut ufc_alt_structure = PacketStructure::default();
    ufc_alt_structure.ez_make(
        "ba5eba11 u32 08 u8 u16 F32 F32 ca11ab1e",
        &["Timestamp", "state", "pkt_len", "temperature", "pressure"],
        true,
    );
    ufc_alt_structure.name = "ufc_alt".to_owned();
    packet_structure_manager
        .register_packet_structure(&mut ufc_alt_structure)
        .expect("Failed to register test packet");

    let mut ufc_bno_structure = PacketStructure::default();
    ufc_bno_structure.ez_make(
        "ba5eba11 u32 04 u8 u16 F32 F32 F32 F32 F32 F32 F32 F32 F32 ca11ab1e",
        &[
            "Timestamp",
            "state",
            "pkt_len",
            "acc_x",
            "acc_y",
            "acc_z",
            "gyro_x",
            "gyro_y",
            "gyro_z",
            "eul_heading",
            "eul_roll",
            "eul_pitch",
        ],
        true,
    );
    ufc_bno_structure.name = "ufc_bno".to_owned();
    packet_structure_manager
        .register_packet_structure(&mut ufc_bno_structure)
        .expect("Failed to register test packet");

    let mut ufc_gps_structure = PacketStructure::default();
    ufc_gps_structure.ez_make("ba5eba11 u32 10 u8 u16 u32 u8 u8 u8 _1 i32 u32 F32 F32 u32 u32 u8 _2 u32 u32 F32 i32 i32 i32 u32 ca11ab1e",
    &["Timestamp","state","pkt_len","time_of_week","time_hour",
    "time_min","time_sec","time_nanosec","timeAccuracy","pos_lat",
    "pos_lon","height_msl","height_elip","fixType","numSatellites",
    "verticalAccuracy","horizontalAccuracy","pDOP","vel_north",
    "vel_east","vel_down","vel_accuracy"],true);
    ufc_gps_structure.name = "ufc_gps".to_owned();
    packet_structure_manager
        .register_packet_structure(&mut ufc_gps_structure)
        .expect("Failed to register test packet");

    let mut ufc_sense_structure = PacketStructure::default();
    ufc_sense_structure.ez_make(
        "ba5eba11 u32 02 u8 u16 F32 F32 F32 F32 F32 F32 F32 F32 F32 F32 F32 F32 ca11ab1e",
        &[
            "Timestamp",
            "state",
            "pkt_len",
            "low_accel_x",
            "low_accel_y",
            "low_accel_z",
            "high_accel_x",
            "high_accel_y",
            "high_accel_z",
            "mag_x",
            "mag_y",
            "mag_z",
            "gyro_x",
            "gyro_y",
            "gyro_z",
        ],
        true,
    );
    ufc_sense_structure.name = "ufc_sense".to_owned();
    packet_structure_manager
        .register_packet_structure(&mut ufc_sense_structure)
        .expect("Failed to register test packet");

    let mut ufc_pitot_center = PacketStructure::default();
    ufc_pitot_center.ez_make(
        "ba5eba11 u32 20 u8 u16 F32 F32 ca11ab1e",
        &[
            "Timestamp",
            "state",
            "pkt_len",
            "center_port",
            "static_port",
        ],
        true,
    );
    ufc_pitot_center.name = "ufc_pitot_center".to_owned();
    packet_structure_manager
        .register_packet_structure(&mut ufc_pitot_center)
        .expect("Failed to register test packet");

    let mut ufc_pitot_radial = PacketStructure::default();
    ufc_pitot_radial.ez_make(
        "ba5eba11 u32 40 u8 u16 F32 F32 F32 F32 ca11ab1e",
        &[
            "Timestamp",
            "state",
            "pkt_len",
            "up_port",
            "down_port",
            "left_port",
            "right_port",
        ],
        true,
    );
    ufc_pitot_radial.name = "ufc_pitot_radial".to_owned();
    packet_structure_manager
        .register_packet_structure(&mut ufc_pitot_radial)
        .expect("Failed to register test packet");

    let mut ufc_test_structure = PacketStructure::default();
    ufc_test_structure.ez_make(
        "ba5eba11 0040 _2 i64 u16 u16 u8 u8 _4 ca11ab1e",
        &[
            "Timestamp",
            "rkt_speed",
            "rkt_speed_also",
            "rkt_budget",
            "var8",
        ],
        true,
    );
    ufc_test_structure.name = "ufc_test".to_owned();
    packet_structure_manager
        .register_packet_structure(&mut ufc_test_structure)
        .expect("Failed to register test packet");
    //################################
    //UFC Hardcoded packets end here
    //################################

    packet_structure_manager
}

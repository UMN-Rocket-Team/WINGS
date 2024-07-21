use std::sync::Mutex;

use crate::{
    models::packet_structure::PacketStructure,
    packet_structure_manager::PacketStructureManager,
};

use super::generic_state::PacketStructureManagerState;

pub fn default_packet_structure_manager() -> PacketStructureManagerState {
    let mut packet_structure_manager = PacketStructureManager::default();

    let mut draw_structure = PacketStructure::default();
    draw_structure.ez_make("ba5eba11 d5a1d5a1 F64 F64 F64 d5a1d5a1 ca11ab1e",
        &["Timestamp","rkt_speed","rkt_speed_also"]);
    draw_structure.name = "draw".to_owned();
    packet_structure_manager.register_packet_structure(&mut draw_structure).expect("Failed to register draw packet");


    //################################
    //UFC Hardcoded packets start here
    //################################
    
    //UFC Raw
    let mut ufc_raw_data_structure = PacketStructure::default();
    ufc_raw_data_structure.ez_make("ba5eba11 0001 _2 i64 i16 i16 i16 u16 u16 u16 u16 u16 u16 u16 u16 u8 u8 u8 i16 i16 i16 i16 i16 i16 i16 i16 i16 _4 ca11ab1e",
    &["Timestamp",
            "acc16g_X" ,"acc16g_Y" ,"acc16g_Z",
            "gyroX"    ,"gyroY"    ,"gyroZ",
            "magX"     ,"magY"     ,"magZ",
            "therm","press",
            "acc100g_X","acc100g_Y","acc100g_Z",
            "accBNO_X" ,"accBNO_Y" ,"accBNO_Z",
            "gyroBNO_X","gyroBNO_Y","gyroBNO_Z",
            "magBNO_X" ,"magBNO_Y" ,"magBNO_Z"]);
    ufc_raw_data_structure.name = "UFC_raw_data".to_owned();
    packet_structure_manager.register_packet_structure(&mut ufc_raw_data_structure).expect("Failed to register raw data packet");

    //UFC Real
    let mut ufc_real_units_structure = PacketStructure::default();
    ufc_real_units_structure.ez_make("ba5eba11 0002 _2 i64 F32 F32 F32 F32 F32 F32 i16 i16 i16 F32 F32 F32 _4 ca11ab1e",
&["Timestamp",
        "acc100g_X","acc100g_Y","acc100g_Z",
        "acc16g_X" ,"acc16g_Y" ,"acc16g_Z",
        "gyroX"    ,"gyroY"    ,"gyroZ",
        "magX"     ,"magY"     ,"magZ",]);
    ufc_real_units_structure.name = "UFC_real_data".to_owned();
    packet_structure_manager.register_packet_structure(&mut ufc_real_units_structure).expect("Failed to register real data packet");

    //UFC BNO
    let mut ufc_bno_structure = PacketStructure::default();
    ufc_bno_structure.ez_make("ba5eba11 0004 _2 i64 F32 F32 F32 F32 F32 F32 F32 F32 F32 _4 ca11ab1e",
&["Timestamp",
        "acc_X" ,"acc_Y" ,"acc_Z",
        "gyro_X","gyro_Y","gyro_Z",
        "mag_X" ,"mag_Y" ,"mag_Z",]);
        ufc_bno_structure.name = "UFC_BNO_data".to_owned();
    packet_structure_manager.register_packet_structure(&mut ufc_bno_structure).expect("Failed to register bno packet");

    //UFC Temp Baro
    let mut ufc_baro_structure = PacketStructure::default();
    ufc_baro_structure.ez_make("ba5eba11 0008 _2 i64 F32 F32 _4 ca11ab1e",
&["Timestamp",
        "therm","press"]);
        ufc_baro_structure.name = "UFC_temp_baro_data".to_owned();
    packet_structure_manager.register_packet_structure(&mut ufc_baro_structure).expect("Failed to register temp-baro packet");

    //UFC GPS 
    let mut ufc_gps_structure = PacketStructure::default();
    ufc_gps_structure.ez_make("ba5eba11 0010 _2 i64 u32 u8 u8 u8 i32 u32 F32 F32 u32 u32 u8 u8 u32 u32 F32 i32 i32 i32 u32 _4 ca11ab1e",
    &["Timestamp",
            "time_of_week",
            "time_hour","time_min","time_sec",
            "time_nanosec",
            "time_accuracy",
            "pos_lat","pos_lon",
            "height_msl","height_elip",
            "fixType","numSatellites",
            "verticalAccuracy","horizontalAccuracy",
            "pDOP",
            "vel_north","vel_east","vel_down",
            "velAccuracy"]);
    ufc_gps_structure.name = "UFC_gps_data".to_owned();
    packet_structure_manager.register_packet_structure(&mut ufc_gps_structure).expect("Failed to register gps packet");

    let mut ufc_test_structure = PacketStructure::default();
    ufc_test_structure.ez_make("ba5eba11 0040 _2 i64 u16 u16 u8 u8 _4 ca11ab1e",
    &["Timestamp","rkt_speed","rkt_speed_also","rkt_budget","var8"]);
    ufc_test_structure.name = "UFC_test_packet".to_owned();
    packet_structure_manager.register_packet_structure(&mut ufc_test_structure).expect("Failed to register test packet");
    //################################
    //UFC Hardcoded packets end here
    //################################

    //################################
    //Altus Metrum Hardcoded packets start here
    //################################

    // TeleMega IMU Data Packet.
    let mut telemega_imu_data = PacketStructure::default();
    telemega_imu_data.ez_make("2A6C u16 08 u8 i16 i32 i16 i16 i16 i16 i16 i16 i16 i16 i16 i16", 
        &["Timestamp",
        "Angle from vertical in degrees","accel","pressure (Pa * 10)","temperature (°C * 100)",
        "accel_x","accel_y","accel_z",
        "gyro_x","gyro_y","gyro_z",
        "mag_x","mag_y","mag_z"]);
    telemega_imu_data.name = "Altus TeleMega IMU Sensor Data".to_owned();
    packet_structure_manager.register_packet_structure(&mut telemega_imu_data).expect("Failed to register telemetrum imu packet");

    // TeleMega Kalman Data Packet.
    let mut telemega_kalman_structure = PacketStructure::default();
    telemega_kalman_structure.ez_make("2A6C u16 09 u8 i16 i16 i8 i8 i8 i8 i8 i8 i32 i16 i16 i16 i16 i16 i16", 
    &["Timestamp","state",
        "v_batt","v_pyro",
        "sense_1","sense_2","sense_3","sense_4","sense_5","sense_6",
        "ground_pres","ground_accel",
        "accel_plus_g","accel_minus_g",
        "acceleration","speed","height"]);
    telemega_kalman_structure.name = "Altus TeleMega Kalman and Voltage Data".to_owned();
    packet_structure_manager.register_packet_structure(&mut telemega_kalman_structure).expect("Failed to register telemetrum kalman packet");

    //TeleMetrum sensor data packet
    let mut metrum_sensor_data = PacketStructure::default();
    metrum_sensor_data.ez_make("2C0E u16 0A u8 i16 i32 i16 i16 i16 i16 i16 i16 i16", 
    &["Timestamp","Flight state",
    "accelerometer",
    "pressure sensor (Pa * 10)","temperature sensor (°C * 100)",
    "acceleration m/s² * 16","speed m/s * 16","height m",
    "battery voltage","drogue continuity sense","main continuity sense"]);
    metrum_sensor_data.name = "Altus TeleMetrum v2 Sensor Data".to_owned();
    packet_structure_manager.register_packet_structure(&mut metrum_sensor_data).expect("Failed to register sensor2 packet");

    //TeleMetrum calibration data
    let mut metrum_calibration_data = PacketStructure::default();
    metrum_calibration_data.ez_make("2C0E u16 0B _3 i32 i16 i16 i16", 
    &["Timestamp",
            "ground_pres","ground_accel",
            "accel_plus_g","accel_minus_g"]);
    metrum_calibration_data.name = "Altus TeleMetrum v2 Calibration Data".to_owned();
    packet_structure_manager.register_packet_structure(&mut metrum_calibration_data).expect("Failed to register telemetrum calibration packet");

    //AltusMetrum config packet
    let mut altus_config_packet = PacketStructure::default();
    altus_config_packet.ez_make("2C0E u16 04 u8 u16 u8 u8 u16 u16 u16 u64 u64", 
    &["Timestamp",
            "Device type","Flight number",
            "Config major version","Config minor version",
            "Apogee deploy delay in seconds","Main deploy alt in meters","Maximum flight log size (kB)",
            "Radio ID String","Software Version String"]);
    altus_config_packet.name = "Altus Configuration Data".to_owned();
    packet_structure_manager.register_packet_structure(&mut altus_config_packet).expect("Failed to register altusmetrum config packet");

    //AltusMetrum gps packet
    let mut altus_gps_packet = PacketStructure::default();
    altus_gps_packet.ez_make("2C0E u16 05 u8 i16 i32 i32 u8 u8 u8 u8 u8 u8 u8 u8 u8 u8 u16 i16 u8", 
    &["Timestamp","flags",
            "altitude (m)",
            "latitude degrees * 107","longitude degrees * 107",
            "year","month","day","hour","minute","second",
            "pdop * 5","hdop * 5","vdop * 5","mode",
            "ground_speed cm/s", "climb_rate cm/s",
            "course / 2"]);
    altus_gps_packet.name = "Altus GPS Location".to_owned();
    packet_structure_manager.register_packet_structure(&mut altus_gps_packet).expect("Failed to register altusmetrum gps packet");

    //AltusMetrum satellite packet
    let mut altus_satellite_packet = PacketStructure::default();
    altus_satellite_packet.ez_make("2C0E u16 06 u8 u8 u8 u8 u8 u8 u8 u8 u8 u8 u8 u8 u8 u8 u8 u8 u8 u8 u8 u8 u8 u8 u8 u8 u8", 
    &["Timestamp","channels",
            "sat_1_svid", "sat_1_c_n_1",
            "sat_2_svid", "sat_2_c_n_1",
            "sat_3_svid", "sat_3_c_n_1",
            "sat_4_svid", "sat_4_c_n_1",
            "sat_5_svid", "sat_5_c_n_1",
            "sat_6_svid", "sat_6_c_n_1",
            "sat_7_svid", "sat_7_c_n_1",
            "sat_8_svid", "sat_8_c_n_1",
            "sat_9_svid", "sat_9_c_n_1",
            "sat_10_svid", "sat_10_c_n_1",
            "sat_11_svid", "sat_11_c_n_1",
            "sat_12_svid", "sat_12_c_n_1"]);
    altus_satellite_packet.name = "Altus GPS Satellite Data".to_owned();
    packet_structure_manager.register_packet_structure(&mut altus_satellite_packet).expect("Failed to register altusmetrum satellite packet");

    Mutex::new(packet_structure_manager)
}
use std::sync::Mutex;

use crate::{
    models::packet_structure::PacketStructure,
    state::mutex_utils::use_state_in_mutex,
    packet_structure_manager::PacketStructureManager,
};

pub struct PacketStructureManagerState {
    pub(crate) packet_structure_manager: Mutex<PacketStructureManager>,
    pub sending_loop_structure: PacketStructure
}

impl Default for PacketStructureManagerState {
    ///The default configuration for a packetStructureManager(the test packet you see when creating a new flight)
    fn default() -> Self {
        let mut draw_structure = PacketStructure::default();
        draw_structure.ez_make("ba5eba11 _4 F64 F64 F64 _4 ca11ab1e");
        draw_structure.fields[0].name = "Timestamp".to_owned();
        draw_structure.fields[1].name = "rkt_speed".to_owned();
        draw_structure.fields[2].name = "rkt_speed_also".to_owned();
        draw_structure.name = "draw".to_owned();

        let mut test_structure = PacketStructure::default();
        test_structure.ez_make("ba5eba11 0010 _2 i64 u16 u16 u8 u8 _4 ca11ab1e");
        test_structure.fields[0].name = "Timestamp".to_owned();
        test_structure.fields[1].name = "rkt_speed".to_owned();
        test_structure.fields[2].name = "rkt_speed_also".to_owned();
        test_structure.fields[3].name = "rkt_budget".to_owned();
        test_structure.fields[4].name = "var8".to_owned();
        test_structure.name = "test".to_owned();

        let mut raw_data_structure = PacketStructure::default();
        raw_data_structure.ez_make("ba5eba11 0001 _2 i64 i16 i16 i16 u16 u16 u16 u16 u16 u16 u16 u16 u8 u8 u8 i16 i16 i16 i16 i16 i16 i16 i16 i16 _4 ca11ab1e");
        raw_data_structure.fields[0].name = "Timestamp".to_owned();
        raw_data_structure.fields[1].name = "acc16g_X".to_owned();
        raw_data_structure.fields[2].name = "acc16g_Y".to_owned();
        raw_data_structure.fields[3].name = "acc16g_Z".to_owned();
        raw_data_structure.fields[4].name = "gyroX".to_owned();
        raw_data_structure.fields[5].name = "gyroY".to_owned();
        raw_data_structure.fields[6].name = "gyroZ".to_owned();
        raw_data_structure.fields[7].name = "magX".to_owned();
        raw_data_structure.fields[8].name = "magY".to_owned();
        raw_data_structure.fields[9].name = "magZ".to_owned();
        raw_data_structure.fields[10].name = "therm".to_owned();
        raw_data_structure.fields[11].name = "press".to_owned();
        raw_data_structure.fields[12].name = "acc100g_X".to_owned();
        raw_data_structure.fields[13].name = "acc100g_Y".to_owned();
        raw_data_structure.fields[14].name = "acc100g_Z".to_owned();
        raw_data_structure.fields[15].name = "accBNO_X".to_owned();
        raw_data_structure.fields[16].name = "accBNO_Y".to_owned();
        raw_data_structure.fields[17].name = "accBNO_Z".to_owned();
        raw_data_structure.fields[18].name = "magBNO_X".to_owned();
        raw_data_structure.fields[19].name = "magBNO_Y".to_owned();
        raw_data_structure.fields[20].name = "magBNO_Z".to_owned();
        raw_data_structure.name = "raw_data".to_owned();

        let mut real_units_structure = PacketStructure::default();
        real_units_structure.ez_make("ba5eba11 0002 _2 i64 i16 i16 i16 i16 i16 i16 i16 i16 i16 i16 i16 i8 i8 i8 i16 i16 i16 i16 i16 i16 i16 i16 i16 _4 ca11ab1e");
        real_units_structure.fields[0].name = "Timestamp".to_owned();
        real_units_structure.fields[1].name = "acc16g_X".to_owned();
        real_units_structure.fields[2].name = "acc16g_Y".to_owned();
        real_units_structure.fields[3].name = "acc16g_Z".to_owned();
        real_units_structure.fields[4].name = "gyroX".to_owned();
        real_units_structure.fields[5].name = "gyroY".to_owned();
        real_units_structure.fields[6].name = "gyroZ".to_owned();
        real_units_structure.fields[7].name = "magX".to_owned();
        real_units_structure.fields[8].name = "magY".to_owned();
        real_units_structure.fields[9].name = "magZ".to_owned();
        real_units_structure.fields[10].name = "therm".to_owned();
        real_units_structure.fields[11].name = "press".to_owned();
        real_units_structure.fields[12].name = "acc100g_X".to_owned();
        real_units_structure.fields[13].name = "acc100g_Y".to_owned();
        real_units_structure.fields[14].name = "acc100g_Z".to_owned();
        real_units_structure.fields[15].name = "accBNO_X".to_owned();
        real_units_structure.fields[16].name = "accBNO_Y".to_owned();
        real_units_structure.fields[17].name = "accBNO_Z".to_owned();
        real_units_structure.fields[18].name = "gyroBNO_X".to_owned();
        real_units_structure.fields[19].name = "gyroBNO_Y".to_owned();
        real_units_structure.fields[20].name = "gyroBNO_Z".to_owned();
        real_units_structure.fields[18].name = "magBNO_X".to_owned();
        real_units_structure.fields[19].name = "magBNO_Y".to_owned();
        real_units_structure.fields[20].name = "magBNO_Z".to_owned();
        real_units_structure.name = "real_data".to_owned();

        let mut gps_structure = PacketStructure::default();
        gps_structure.ez_make("ba5eba11 0004 _2 i64 u32 u8 u8 u8 i32 u32 F32 F32 u32 u32 u8 u8 u32 u32 F32 i32 i32 i32 u32 _4 ca11ab1e");
        gps_structure.fields[0].name = "Timestamp".to_owned();
        gps_structure.fields[1].name = "time_of_week".to_owned();
        gps_structure.fields[2].name = "time_hour".to_owned();
        gps_structure.fields[3].name = "time_min".to_owned();
        gps_structure.fields[4].name = "time_sec".to_owned();
        gps_structure.fields[5].name = "time_nanosec".to_owned();
        gps_structure.fields[6].name = "timeAccuracy".to_owned();
        gps_structure.fields[7].name = "pos_lat".to_owned();
        gps_structure.fields[8].name = "pos_lon".to_owned();
        gps_structure.fields[9].name = "height_msl".to_owned();
        gps_structure.fields[10].name = "height_elip".to_owned();
        gps_structure.fields[11].name = "fixType".to_owned();
        gps_structure.fields[12].name = "numSatellites".to_owned();
        gps_structure.fields[13].name = "verticalAccuracy".to_owned();
        gps_structure.fields[14].name = "horizontalAccuracy".to_owned();
        gps_structure.fields[15].name = "pDOP".to_owned();
        gps_structure.fields[16].name = "vel_north".to_owned();
        gps_structure.fields[17].name = "vel_east".to_owned();
        gps_structure.fields[18].name = "vel_down".to_owned();
        gps_structure.fields[19].name = "velAccuracy".to_owned();
        gps_structure.name = "gps_data".to_owned();

        // Telemega Kalman and Voltage Data Packet Contents.
        let mut telemega_kalman_structure = PacketStructure::default();
        telemega_kalman_structure.ez_make("2A6C u16 09 u8 i16 i16 i8 i8 i8 i8 i8 i8 i32 i16 i16 i16 i16 i16 i16");
        telemega_kalman_structure.fields[0].name = "Timestamp".to_owned();
        telemega_kalman_structure.fields[1].name = "state".to_owned();
        telemega_kalman_structure.fields[2].name = "v_batt".to_owned();
        telemega_kalman_structure.fields[3].name = "v_pyro".to_owned();
        telemega_kalman_structure.fields[4].name = "sense_1".to_owned();
        telemega_kalman_structure.fields[5].name = "sense_2".to_owned();
        telemega_kalman_structure.fields[6].name = "sense_3".to_owned();
        telemega_kalman_structure.fields[7].name = "sense_4".to_owned();
        telemega_kalman_structure.fields[8].name = "sense_5".to_owned();
        telemega_kalman_structure.fields[9].name = "sense_6".to_owned();
        telemega_kalman_structure.fields[10].name = "ground_pres".to_owned();
        telemega_kalman_structure.fields[11].name = "ground_accel".to_owned();
        telemega_kalman_structure.fields[12].name = "accel_plus_g".to_owned();
        telemega_kalman_structure.fields[13].name = "accel_minus_g".to_owned();
        telemega_kalman_structure.fields[14].name = "acceleration".to_owned();
        telemega_kalman_structure.fields[15].name = "speed".to_owned();
        telemega_kalman_structure.fields[16].name = "height".to_owned();
        telemega_kalman_structure.name = "telemega_kalman".to_owned();

        let mut metrum_sensor_structure = PacketStructure::default();
        metrum_sensor_structure.ez_make("2C0E u16 01 u8 i16 i16 i16 i16 i16 i16 i16 i16 i16 i16 i16 i16 i16");
        metrum_sensor_structure.fields[0].name = "Timestamp".to_owned();
        metrum_sensor_structure.fields[1].name = "Flight state".to_owned();
        metrum_sensor_structure.fields[2].name = "accelerometer".to_owned();
        metrum_sensor_structure.fields[3].name = "pressure sensor".to_owned();
        metrum_sensor_structure.fields[4].name = "temperature sensor".to_owned();
        metrum_sensor_structure.fields[5].name = "battery voltage".to_owned();
        metrum_sensor_structure.fields[6].name = "drogue continuity sense".to_owned();
        metrum_sensor_structure.fields[7].name = "main continuity sense".to_owned();
        metrum_sensor_structure.fields[8].name = "acceleration m/s² * 16".to_owned();
        metrum_sensor_structure.fields[9].name = "speed m/s * 16".to_owned();
        metrum_sensor_structure.fields[10].name = "height m".to_owned();
        metrum_sensor_structure.fields[11].name = "Average barometer reading on ground".to_owned();
        metrum_sensor_structure.fields[12].name = "ground_accel".to_owned();
        metrum_sensor_structure.fields[13].name = "accel_plus_g".to_owned();
        metrum_sensor_structure.fields[14].name = "accel_minus_g".to_owned();
        metrum_sensor_structure.name = "TeleMetrum v1.x Sensor Data".to_owned();

        let mut metrum_sensor_2_structure = PacketStructure::default();
        metrum_sensor_2_structure.ez_make("2C0E u16 0A u8 i16 i32 i16 i16 i16 i16 i16 i16 i16");
        metrum_sensor_2_structure.fields[0].name = "Timestamp".to_owned();
        metrum_sensor_2_structure.fields[1].name = "Flight state".to_owned();
        metrum_sensor_2_structure.fields[2].name = "accelerometer".to_owned();
        metrum_sensor_2_structure.fields[3].name = "pressure sensor (Pa * 10)".to_owned();
        metrum_sensor_2_structure.fields[4].name = "temperature sensor (°C * 100)".to_owned();
        metrum_sensor_2_structure.fields[5].name = "acceleration m/s² * 16".to_owned();
        metrum_sensor_2_structure.fields[6].name = "speed m/s * 16".to_owned();
        metrum_sensor_2_structure.fields[7].name = "height m".to_owned();
        metrum_sensor_2_structure.fields[8].name = "battery voltage".to_owned();
        metrum_sensor_2_structure.fields[9].name = "drogue continuity sense".to_owned();
        metrum_sensor_2_structure.fields[10].name = "main continuity sense".to_owned();
        metrum_sensor_2_structure.name = "TeleMetrum v2 Sensor Data".to_owned();

        let mut packet_structure_manager = PacketStructureManager::default();
        packet_structure_manager.register_packet_structure(&mut draw_structure).expect("Failed to register draw packet");
        packet_structure_manager.register_packet_structure(&mut test_structure).expect("Failed to register test packet");
        packet_structure_manager.register_packet_structure(&mut raw_data_structure).expect("Failed to register test packet");
        packet_structure_manager.register_packet_structure(&mut real_units_structure).expect("Failed to register test packet");
        packet_structure_manager.register_packet_structure(&mut gps_structure).expect("Failed to register test packet");
        packet_structure_manager.register_packet_structure(&mut telemega_kalman_structure).expect("Failed to register kalman packet");
        packet_structure_manager.register_packet_structure(&mut metrum_sensor_structure).expect("Failed to register sensor packet");
        packet_structure_manager.register_packet_structure(&mut metrum_sensor_2_structure).expect("Failed to register sensor2 packet");
        Self {
            packet_structure_manager: Mutex::new(packet_structure_manager),
            sending_loop_structure: metrum_sensor_2_structure
        }
    }
}

pub fn use_packet_structure_manager<ReturnType, ErrorType>(
    packet_structure_manager_state: &PacketStructureManagerState,
    callback: &mut dyn FnMut(&mut PacketStructureManager) -> Result<ReturnType, ErrorType>,
) -> Result<ReturnType, String>
where
    ErrorType: std::fmt::Display,
{
    use_state_in_mutex(
        &packet_structure_manager_state.packet_structure_manager,
        callback,
    )
}

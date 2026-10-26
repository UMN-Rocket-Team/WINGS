pub mod communication_manager;
pub mod data_processing;
pub mod drivers;
pub mod file_handling;
pub mod models;
pub mod state;
pub mod packet_generator;
pub mod packet_structure_manager;

pub mod prelude {
    pub use drivers::*;
    pub use file_handling::*;
    pub use models::*;
    pub use state::*;
    pub use crate::communication_manager::*;
    pub use crate::data_processing::*;
    pub mod drivers {
        pub use crate::drivers::{
            aim::{
                aim_adapter::*,
                aim_parser::*
            },
            binary_file::{
                binary_file_adapter::*
            },
            csv_file::{
                csv_file_adapter::*,
            },
            featherweight::{
                featherweight_adapter::*,
                featherweight_parser::*,
            },
            midwest::{
                midwest_adapter::*,
                midwest_parser::*,
            },
            serial::{
                serial_packet_parser::*,
                serial_port_adapter::*,
            },
            teledongle::{
                teledongle_adapter::*,
                teledongle_packet_parser::*,
            }
        };
    }
    pub mod file_handling {
        pub use crate::file_handling::{
            config_struct::*,
            log_handlers::*
        };
    }
    pub mod models {
        pub use crate::models::{
            packet_parser::*,
            packet_structure::*,
            packet_view_model::*,
            packet::*,
            product::*,
        };
    }
    pub mod state {
        pub use crate::state::{
            generic_state::*,
            mutex_utils::*,
            packet_structure_manager_state::*,
            uart::*
        };
    }
    pub use crate::packet_generator::*;
    pub use crate::packet_structure_manager::*;
}
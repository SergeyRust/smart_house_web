use rql::prelude::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)] pub struct Room {
    pub id: u64,
    pub name: String,
    pub devices: Vec<Device>, }

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)] pub struct Device {
    pub id: u64,
    pub name: String,
    pub is_on: bool,
    pub device_type: DeviceType, }

pub enum DeviceType {
    Socket, Thermo
}

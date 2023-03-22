use rql::Id;
use serde::{Deserialize, Serialize};
use crate::errors::SmartHouseError;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Room {
    pub name: String,
    pub devices: Vec<Id<Device>>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Device {
    pub room_id: Id<Room>,
    pub name: String,
    pub is_on: bool,
    pub device_type: DeviceType,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RoomDto {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DeviceDto {
    pub device_name: String,
    pub is_on: bool,
    pub device_type: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AddDeviceDto {
    pub room_name: String,
    pub device_name: String,
    pub device_type: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct RemoveDeviceDto {
    pub room_name: String,
    pub device_name: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum DeviceType {
    Socket,
    Thermo,
}

impl DeviceType {

    pub fn parse(value: &str) -> Result<DeviceType, SmartHouseError> {
        match value {
            // regexp...
            "Socket" | "socket" => Ok(DeviceType::Socket),
            "Thermo" | "thermo" => Ok(DeviceType::Thermo),
            _ => Err(SmartHouseError::WrongRequestDataError("wrong device type".to_owned()))
        }
    }

    pub fn to_str(&self) -> String {
        match self {
            DeviceType::Socket => "Socket".to_owned(),
            DeviceType::Thermo => "Thermo".to_owned()
        }
    }
}


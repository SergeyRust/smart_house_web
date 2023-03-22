use rql::{schema, mashup, Table};
use crate::domain::{Room, Device};

schema! {
    pub Database {
        room: Room,
        device: Device,
    }
}

pub mod room {

    use rql::{HasRows};
    use crate::repository::Database;
    use crate::domain:: Room;
    use crate::errors::{ROOM_ERROR, SmartHouseError};

    impl Room {
        // adding room without devices is the only available
        pub fn add(db: &Database, name: &str) -> Result<(), SmartHouseError> {
            if let Some(room_with_equal_name) = db.room().find(|r| r.name.eq(name)) {
                let err = format!["{} {} {}", ROOM_ERROR, room_with_equal_name.name, "already"];
                return Err(SmartHouseError::WrongRequestDataError(err));
            }
            db.room_mut().insert( Room { name: name.to_owned(), devices: vec![]});
            Ok(())
        }

        pub fn remove(db: &Database, room_name: &str) -> Result<(), SmartHouseError> {
            let binding = db.room_mut();
            let room = binding.find(|r| r.name.as_str().eq(room_name));
            if room.is_none() {
                let err = format!["{} {} {}", ROOM_ERROR, room_name, "doesn't"];
                return Err(SmartHouseError::WrongRequestDataError(err))
            }
            let room_to_delete = room.unwrap().id;
            db.room_mut().delete_one(room_to_delete);
            db.device_mut().delete_where(|d| d.room_id.eq(&room_to_delete));
            Ok(())
        }

        pub fn rooms(db: &Database) -> Option<Vec<String>> {
            let rooms = db.room_mut().rows_mut()
                .map(|r| String::from(&r.name))
                .collect::<Vec<String>>();
            if rooms.is_empty() {
                None
            } else {
                Some(rooms)
            }
        }
    }
}

pub mod device {

    use rql::{HasRows, Id};
    use crate::repository::Database;
    use crate::domain::{DeviceDto, DeviceType};
    use crate::domain::Device;
    use crate::errors::{ ROOM_ERROR, SmartHouseError};

    impl Device {

        pub fn add(db: &Database, room_name: &str, device_name: &str, device_type: DeviceType)
            -> Result<(), SmartHouseError> {
            let binding = db.room();
            let room = binding.find(|r| r.name.as_str().eq(room_name));
            if let Some(room) = room  {
                db.device_mut().insert(Device {
                    room_id: room.id,
                    name: device_name.to_string(),
                    is_on: false,
                    device_type,
                });
                Ok(())
            } else {
                let err = format!["{} {} {}", ROOM_ERROR, room_name, "doesn't"];
                Err(SmartHouseError::WrongRequestDataError(err))
            }
        }

        pub fn remove(db: &Database, room_name: &str, device_name: &str)
            -> Result<(), SmartHouseError> {
            let mut device_to_delete: Id<Device> = Default::default();
            for (device, _room) in db.device()
                .relate(&*db.room(), |d, r|
                    d.room_id == r.id && r.name.as_str().eq(room_name) && d.name.as_str().eq(device_name)) {
                device_to_delete = device.id;
            };
            let deleted = db.device_mut().delete_one(device_to_delete);
            if deleted.is_some()  {
                Ok(())
            } else {
                let err = format!["{} {} {}", ROOM_ERROR, room_name, "doesn't"];
                Err(SmartHouseError::WrongRequestDataError(err))
            }
        }

        pub fn devices(db: &Database, room_name: &str) -> Result<Vec<DeviceDto>, SmartHouseError> {
            let mut devices = vec![];
            for (_room, device) in db.room()
                .relate(&*db.device(), |r, d| d.room_id == r.id && r.name.as_str().eq(room_name))
                .select(|(r, d)| (r,d) ) {
                devices.push(DeviceDto {
                    device_name: String::from(&device.name),
                    is_on: false,
                    device_type: device.device_type.to_str(),
                })
            };
            if devices.is_empty() {
                Err(SmartHouseError::WrongRequestDataError("Room doesn't have devices".to_string()))
            } else {
                Ok(devices)
            }
        }
    }
}
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
    use crate::database::Database;
    use crate::domain::{DeviceDto, Room, RoomDto};
    use crate::errors::{ROOM_ERROR, SmartHouseError};

    impl Room {
        // adding room without devices is the only available
        pub fn add(db: &Database, name: &str) -> Result<(), SmartHouseError> {
            if let Some(room_with_equal_name) = db.room().find(|r| r.name.eq(name)) {
                let err = format!["{} {} {}", ROOM_ERROR, room_with_equal_name.name, "already"];
                return Err(SmartHouseError::WrongRequestDataError(err));
            }
            db.room_mut().insert( Room {name: name.to_owned(), devices: vec![]});
            Ok(())
        }

        pub fn remove(db: &Database, name: &str) -> Result<(), SmartHouseError> {
            let temp = db.room_mut();
            let room = temp.find(|r| r.name.eq(name));
            if room.is_none() {
                let err = format!["{} {} {}", ROOM_ERROR, name, "doesn't"];
                return Err(SmartHouseError::WrongRequestDataError(err));
            }
            db.room_mut().remove(room.unwrap().id);
            Ok(())
        }

        pub fn rooms(db: &Database) -> Option<Vec<RoomDto>> {
            let rooms = db.room_mut().rows_mut()
                .map(|r| {
                    let name = String::from(&r.name);
                    let devices = r.devices.iter()
                        .map(|d| (d.clone()).into())
                        .collect::<Vec<DeviceDto>>();
                    RoomDto { name, devices: Some(devices) }
                    })
                .collect::<Vec<RoomDto>>();
            if rooms.is_empty() {
                None
            } else {
                Some(rooms)
            }
        }
    }
}

pub mod device {
    use rql::{HasRows, Row};
    use crate::database::Database;
    use crate::domain::{DeviceDto, DeviceType, Room};
    use crate::domain::Device;
    use crate::errors::{DEVICE_ERROR, ROOM_ERROR, SmartHouseError};

    impl Device {

        pub fn add(db: &Database, room_name: &str, device_name: &str, device_type: DeviceType) -> Result<(), SmartHouseError> {
            let room = Self::find_room(db, room_name)?;
            let device = Device {
                name: device_name.parse().unwrap(),
                is_on: false,
                device_type
            };
            room.devices.push(device);
            Ok(())
        }

        pub fn remove(db: &Database, room_name: &str, device_name: &str) -> Result<(), SmartHouseError> {
            let room = Self::find_room(db, room_name)?;
            let is_exists = room.devices.iter()
                .find(|d| d.name.eq(device_name))
                .is_some();
            if !is_exists {
                let err = format!["{} {}", DEVICE_ERROR, device_name];
                Err(SmartHouseError::WrongRequestDataError(err))
            } else {
                db.room().remove(room.id);
                Ok(())
            }
        }

        pub fn devices(db: &Database, room_name: &str) -> Result<Vec<DeviceDto>, SmartHouseError> {
            let room = Self::find_room(db, room_name)?;
            let res = room.devices.iter()
                .map(|d| (d.clone()).into())
                .collect::<Vec<DeviceDto>>();
            Ok(res)
        }

        fn find_room<'a>(db: &'a Database, room_name: &'a str) -> Result<Row<'a, Room>, SmartHouseError> {
            let room_opt = db.room().find(|r| r.name.eq(room_name));
            if room_opt.is_none() {
                let err = format!["{} {} {}", ROOM_ERROR, room_name, "doesn't"];
                Err(SmartHouseError::WrongRequestDataError(err))
            } else {
                Ok(room_opt.unwrap().clone())
            }
        }
    }
}
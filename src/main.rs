mod errors;
mod repository;
mod domain;

use std::collections::HashMap;
use actix_web::{App, HttpServer, Responder, web};
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json, Query};
use rql::HumanReadable;
use crate::repository::Database;
use crate::domain::{AddDeviceDto, Device, DeviceType, RemoveDeviceDto, Room, RoomDto};

const ROOM_ADDED: &str = "room {} added";
const ROOM_REMOVED: &str = "room {} removed";
const COULD_NOT_ADD_ROOM: &str = "could not add room ";
const COULD_NOT_REMOVE_ROOM: &str = "could not remove room ";
const DEVICE_ADDED: &str = "device {} added";
const DEVICE_REMOVED: &str = "device {} removed";
const COULD_NOT_ADD_DEVICE: &str = "could not add device ";
const COULD_NOT_PARSE_DEVICE_TYPE: &str = "could not parse device type";
const COULD_NOT_REMOVE_DEVICE: &str = "could not remove device ";

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(Database::new("database", HumanReadable).unwrap()))
            .service(web::resource("/smart-house/room/add").route(web::post().to(add_room)))
            .service(web::resource("/smart-house/room/remove").route(web::delete().to(remove_room)))
            .service(web::resource("/smart-house/room").route(web::get().to(rooms)))
            .service(web::resource("/smart-house/device/add").route(web::post().to(add_device)))
            .service(web::resource("/smart-house/device/remove").route(web::delete().to(remove_device)))
            .service(web::resource("/smart-house/device").route(web::get().to(devices)))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

async fn add_room(db: Data<Database>, room: Json<RoomDto>) -> impl Responder {
    let added = Room::add(db.as_ref(), room.name.as_str());
    if added.is_ok() {
        format!["{} {}", ROOM_ADDED, room.name.as_str()]
            .customize()
            .with_status(StatusCode::OK)
    } else {
        (String::from(COULD_NOT_ADD_ROOM) + room.name.as_str())
            .customize()
            .with_status(StatusCode::BAD_REQUEST)
    }
}

async fn remove_room(db: Data<Database>, query: Query<HashMap<String, String>>) -> impl Responder {
    if let Some(room_name) = query.get("room_name") {
        let removed = Room::remove(db.as_ref(), room_name.as_str());
        if removed.is_ok() {
            format!["{} {}", ROOM_REMOVED, room_name.as_str()]
                .customize()
                .with_status(StatusCode::OK)
        } else {
            (String::from(COULD_NOT_REMOVE_ROOM) + room_name.as_str())
                .customize()
                .with_status(StatusCode::BAD_REQUEST)
        }
    } else {
        "".to_owned().customize().with_status(StatusCode::BAD_REQUEST)
    }
}

async fn rooms(db: Data<Database>) -> impl Responder {
    let rooms = Room::rooms(db.as_ref());
    if rooms.is_some() {
        Json(rooms)
            .customize()
            .with_status(StatusCode::OK)
    } else {
        Json(None)
            .customize()
            .with_status(StatusCode::NO_CONTENT)
    }
}

async fn add_device(db: Data<Database>, device: Json<AddDeviceDto>) -> impl Responder {
    let device_type = DeviceType::parse(device.device_type.as_str());
    if device_type.is_err() {
        String::from(COULD_NOT_PARSE_DEVICE_TYPE)
            .customize()
            .with_status(StatusCode::BAD_REQUEST)
    } else {
        let added = Device::add(
            db.as_ref(),
            device.room_name.as_str(),
            device.device_name.as_str(),
            device_type.unwrap()
        );
        if added.is_ok() {
            format!["{} {}", DEVICE_ADDED, device.device_name.as_str()]
                .customize()
                .with_status(StatusCode::OK)
        } else {
            (String::from(COULD_NOT_ADD_DEVICE) + device.device_name.as_str())
                .customize()
                .with_status(StatusCode::BAD_REQUEST)
        }
    }
}

async fn remove_device(db: Data<Database>, dto: Json<RemoveDeviceDto>) -> impl Responder {
    let device_name = dto.device_name.as_str();
    let removed = Device::remove(db.as_ref(), dto.room_name.as_str(), device_name);
    if removed.is_ok() {
        format!["{DEVICE_REMOVED} {device_name}"]
            .customize()
            .with_status(StatusCode::OK)
    } else {
        (String::from(COULD_NOT_REMOVE_DEVICE) + device_name)
            .customize()
            .with_status(StatusCode::BAD_REQUEST)
    }

}

async fn devices(db: Data<Database>, query: Query<HashMap<String, String>>) -> impl Responder {
    if let Some(room_name) = query.get("room_name") {
        let devices = Device::devices(db.as_ref(), room_name.as_str());
        if devices.is_ok() {
            Json(devices.unwrap())
                .customize()
                .with_status(StatusCode::OK)
        } else {
            Json(vec![])
                .customize()
                .with_status(StatusCode::NO_CONTENT)
        }
    } else {
        Json(vec![]).customize().with_status(StatusCode::BAD_REQUEST)
    }
}




use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io;

use thiserror::Error;

pub const ROOM_ERROR : &str = "room with name {} {} exist";

#[derive(Error, Debug)]
pub enum SmartHouseError {
    NetworkError(#[from] io::Error),
    WrongRequestDataError(String),
    CommandError(#[from] DeviceError)
}

#[derive(Debug, Error)]
pub enum DeviceError {
    SocketError(&'static str), ThermoError(&'static str),
}

impl Display for DeviceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CommandError :{}", self.source().unwrap())
    }
}

impl Display for SmartHouseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SmartHouseError :{}", self.source().unwrap())
    }
}

use std::{io::Read, net::TcpStream, rc::Rc};

use serde::Deserialize;

use crate::{ControlH, Hardware, HardwareBridge, HardwareItem, Value, HardwareError};

pub struct WindowsBridge {}

const IP: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 55555;

#[derive(Deserialize, Debug, Clone)]
enum HardwareType {
    Control,
    Fan,
    Temp,
}

#[derive(Deserialize, Debug, Clone)]
struct BaseHardware {
    #[serde(rename = "Id")]
    id: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Index")]
    index: usize,
    #[serde(rename = "Index2")]
    index2: usize,
    #[serde(rename = "Type")]
    hardware_type: HardwareType,
}

impl HardwareBridge for WindowsBridge {
    fn generate_hardware() -> Hardware {
        let mut hardware = Hardware::default();

        let mut stream = try_connect();
        println!("Connected to the server!");

        let mut data = String::new();
        stream.read_to_string(&mut data).unwrap();
        let base_hardware_list = serde_json::from_str::<Vec<BaseHardware>>(&data).unwrap();
        
        for base_hardware in base_hardware_list {
            match base_hardware.hardware_type {
                HardwareType::Control => hardware.controls.push(Rc::new(ControlH {
                    name: base_hardware.name,
                    hardware_id: base_hardware.id,
                    info: String::new(),
                    bridge: Box::new(InternalControl {
                        io: base_hardware.index,
                        enable: base_hardware.index2,
                    }),
                })),
                HardwareType::Fan => hardware.controls.push(Rc::new(ControlH {
                    name: base_hardware.name,
                    hardware_id: base_hardware.id,
                    info: String::new(),
                    bridge: Box::new(InternalSensor {
                        index: base_hardware.index,
                    }),
                })),
                HardwareType::Temp => hardware.controls.push(Rc::new(ControlH {
                    name: base_hardware.name,
                    hardware_id: base_hardware.id,
                    info: String::new(),
                    bridge: Box::new(InternalSensor {
                        index: base_hardware.index,
                    }),
                })),
            }
        }

        hardware
    }
}

fn try_connect() -> TcpStream {
    for port in DEFAULT_PORT..65535 {
        match TcpStream::connect((IP, port)) {
            Ok(stream) => return stream,
            Err(_) => continue,
        }
    }
    panic!("can't find connection")
}


#[derive(Debug)]
struct InternalSensor {
    index: usize,
}

#[derive(Debug)]
struct InternalControl {
    io: usize,
    enable: usize,
}

impl Drop for InternalControl {
    fn drop(&mut self) {
        info!("pwm sould be set to auto");
        // TODO: set to auto
    }
}

impl HardwareItem for InternalSensor {
    fn get_value(&self) -> Result<Value, crate::HardwareError> {     
        println!("get value");
        return Ok(4);
    }

    fn set_value(&self, value: Value) -> Result<(), crate::HardwareError> {
        panic!("can't set the value of a sensor");
    }

    fn set_mode(&self, value: Value) -> Result<(), HardwareError> {
        panic!("can't set the mode of a sensor");
    }
}

impl HardwareItem for InternalControl {
    fn get_value(&self) -> Result<Value, crate::HardwareError> {
        panic!("can't get the value of a control");
    }

    fn set_value(&self, value: Value) -> Result<(), crate::HardwareError> {
        debug!("set value {} to a control", value);
        Ok(())
    }

    fn set_mode(&self, value: Value) -> Result<(), HardwareError> {
        debug!("set mode {} to a control", value);
        Ok(())
    }
}

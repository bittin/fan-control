use std::collections::HashMap;

use lm_sensors::{feature, value, ChipRef, FeatureRef, LMSensors, SubFeatureRef};

use crate::{ControlH, FanH, Hardware, HardwareBridge, HardwareError, HardwareType, TempH};

pub struct LinuxBridge {
    lib: &'static LMSensors,
    sensors: HashMap<String, Sensor<'static>>,
}

impl Drop for LinuxBridge {
    fn drop(&mut self) {
        let boxed = Box::new(self.lib);
        let ptr = Box::into_raw(boxed);
        unsafe {
            let _raw = Box::from_raw(ptr);
        }
    }
}

#[derive(Debug, Clone)]
struct Sensor<'a> {
    hardware_type: HardwareType,
    sub_feature_ref: SubFeatureRef<'a>,
    name: String,
    info: String,
    id: String,
}

impl HardwareBridge for LinuxBridge {
    fn new() -> impl HardwareBridge {
        let lib = lm_sensors::Initializer::default().initialize().unwrap();
        let boxed = Box::new(lib);
        let leaked: &'static mut LMSensors = Box::leak(boxed);

        let mut sensors = HashMap::new();

        for chip_ref in leaked.chip_iter(None) {
            for feature_ref in chip_ref.feature_iter() {
                match feature_ref.kind() {
                    Some(feature_kind) => match feature_kind {
                        feature::Kind::Fan => {
                            let Ok(sub_feature_ref) =
                                feature_ref.sub_feature_by_kind(value::Kind::FanInput)
                            else {
                                continue;
                            };

                            if let Some((id, name, info)) =
                                generate_id_name_info(&chip_ref, &feature_ref, &sub_feature_ref)
                            {
                                let sensor = Sensor {
                                    hardware_type: HardwareType::Fan,
                                    sub_feature_ref,
                                    name,
                                    info,
                                    id: id.clone(),
                                };
                                sensors.insert(id, sensor);
                            }
                        }
                        feature::Kind::Temperature => {
                            let Ok(sub_feature_ref) =
                                feature_ref.sub_feature_by_kind(value::Kind::TemperatureInput)
                            else {
                                continue;
                            };

                            if let Some((id, name, info)) =
                                generate_id_name_info(&chip_ref, &feature_ref, &sub_feature_ref)
                            {
                                let sensor = Sensor {
                                    hardware_type: HardwareType::Temp,
                                    sub_feature_ref,
                                    name,
                                    info,
                                    id: id.clone(),
                                };
                                sensors.insert(id, sensor);
                            }
                        }
                        _ => continue,
                    },
                    None => continue,
                };
            }
        }

        LinuxBridge {
            lib: leaked,
            sensors,
        }
    }

    fn hardware(&self) -> Hardware {
        let mut hardware = Hardware::default();

        for sensor in self.sensors.values() {
            match sensor.hardware_type {
                HardwareType::Control => hardware.controls.push(ControlH {
                    name: sensor.name.clone(),
                    hardware_id: sensor.id.clone(),
                    info: sensor.info.clone(),
                }),
                HardwareType::Fan => hardware.fans.push(FanH {
                    name: sensor.name.clone(),
                    hardware_id: sensor.id.clone(),
                    info: sensor.info.clone(),
                }),
                HardwareType::Temp => hardware.temps.push(TempH {
                    name: sensor.name.clone(),
                    hardware_id: sensor.id.clone(),
                    info: sensor.info.clone(),
                }),
            }
        }

        hardware
    }

    fn value(&self, hardware_id: &str) -> Result<Option<i32>, crate::HardwareError> {
        match self.sensors.get(hardware_id) {
            Some(sensor) => match sensor.sub_feature_ref.raw_value() {
                Ok(value) => Ok(Some(value as i32)),
                Err(e) => {
                    eprintln!("{}", e);
                    Err(HardwareError::LmSensors)
                }
            },
            None => Err(HardwareError::IdNotFound),
        }
    }

    fn set_value(&self, hardware_id: &str, value: i32) -> Result<(), crate::HardwareError> {
        todo!()
    }

    fn info(&self, hardware_id: &str) -> Result<String, crate::HardwareError> {
        match self.sensors.get(hardware_id) {
            Some(sensor) => Ok(sensor.info.clone()),
            None => Err(HardwareError::IdNotFound),
        }
    }
}

fn generate_id_name_info(
    chip_ref: &ChipRef,
    feature_ref: &FeatureRef,
    sub_feature_ref: &SubFeatureRef,
) -> Option<(String, String, String)> {
    let Some(chip_path) = chip_ref.path() else {
        return None;
    };

    let bus = chip_ref.bus();

    let Ok(label) = feature_ref.label() else {
        return None;
    };

    let Ok(chip_name) = chip_ref.name() else {
        return None;
    };

    let Some(Ok(sub_feature_name)) = sub_feature_ref.name() else {
        return None;
    };

    let id = format!("{}-{}", chip_name, sub_feature_name);
    let name = format!("{} {} {}", label, chip_name, sub_feature_name);
    let info = format!(
        "chip path: {}\nchip name: {}\nbus: {}\nlabel: {}\nfeature: {}",
        chip_path.display(),
        chip_name,
        bus,
        label,
        sub_feature_name
    );

    Some((id, name, info))
}

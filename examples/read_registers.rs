extern crate libmodbus_sys as raw;
extern crate libmodbus_rs;

use libmodbus_rs::modbus::{Modbus};
use std::collections::HashMap;

fn main() {
    let device: String = std::env::args().nth(1).unwrap();
    let slave_id: i32 = std::env::args().nth(2).unwrap().parse().unwrap();

    let mut modbus = Modbus::new_rtu(&device, 9600, 'N', 8, 1);
    let _ = modbus.set_slave(slave_id);
    let _ = modbus.set_debug(true);
    // let _ = modbus.rtu_set_rts(raw::MODBUS_RTU_RTS_DOWN);

    // Hashmap die einige Bezeichnungen speichert
    let mut register_names = HashMap::new();

    register_names.insert(0, "Version Nummer Major");
    register_names.insert(1, "Version Nummer Minor");
    register_names.insert(2, "Version Nummer Patch");
    register_names.insert(3, "Modbus Adresse dynamisch");
    register_names.insert(10, "Sensor1 Nummer wird intern zur Identifikation genutzt");
    register_names.insert(11, "Sensor1 aktueller ADC Wert dynamisch");
    register_names.insert(12, "Sensor1 ADC Wert bei Nullgas Kalibrationsdaten");
    register_names.insert(13, "Sensor1 ADC Wert bei Messgas Kalibrationsdaten");
    register_names.insert(14, "Sensor1 SI Einheit Kalibrationsdaten");
    register_names.insert(19, "Sensor1 Konfiguration 16Bit COILS des Sensors");
    register_names.insert(20, "Sensor2 Nummer wird intern zur Identifikation genutzt");
    register_names.insert(21, "Sensor2 aktueller ADC Wert dynamisch");
    register_names.insert(22, "Sensor2 ADC Wert bei Nullgas Kalibrationsdaten");
    register_names.insert(23, "Sensor2 ADC Wert bei Messgas Kalibrationsdaten");
    register_names.insert(24, "Sensor2 SI Einheit Kalibrationsdaten");
    register_names.insert(29, "Sensor2 Konfiguration 16Bit COILS des Sensors");



    match modbus.connect() {
        Err(_) => { modbus.free(); }
        Ok(_) => {
            let tab_reg = modbus.read_registers(0, 30).unwrap();
            println!("");
            for i in 0..30 {
                print!("register[{:02}]={1:06} (0x{1:04X})", i, &tab_reg[i as usize]);
                if register_names.contains_key(&i) {
                    println!("\t{}", register_names.get(&i).unwrap());
                } else {
                    println!("");
                }
            }
        }
    }
}

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct KeyCMD {
    pub key: String,
    pub val: f32,
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
pub struct CMD {
    pub id: u8,
    pub val: f32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CmdMap {
    pub map: HashMap<String, u8>,
}

impl CmdMap {
    pub fn new() -> Self {
        let mut cmd_map: HashMap<String, u8> = HashMap::new();
        cmd_map.insert("enable".to_string(), 1);
        cmd_map.insert("target".to_string(), 2);
        cmd_map.insert("loop_mode".to_string(), 3);
        cmd_map.insert("voltage_limit".to_string(), 4);
        cmd_map.insert("voltage_power".to_string(), 5);
        cmd_map.insert("velocity_limit".to_string(), 6);
        cmd_map.insert("torque_limit".to_string(), 7);

        cmd_map.insert("torque_p".to_string(), 11);
        cmd_map.insert("torque_i".to_string(), 12);
        cmd_map.insert("torque_d".to_string(), 13);
        cmd_map.insert("torque_ramp".to_string(), 14);
        cmd_map.insert("torque_tf".to_string(), 15);

        cmd_map.insert("velocity_p".to_string(), 21);
        cmd_map.insert("velocity_i".to_string(), 22);
        cmd_map.insert("velocity_d".to_string(), 23);
        cmd_map.insert("velocity_ramp".to_string(), 24);
        cmd_map.insert("velocity_tf".to_string(), 25);

        cmd_map.insert("position_p".to_string(), 31);
        cmd_map.insert("position_i".to_string(), 32);
        cmd_map.insert("position_d".to_string(), 33);
        cmd_map.insert("position_ramp".to_string(), 34);
        cmd_map.insert("position_tf".to_string(), 35);

        cmd_map.insert("conf_base".to_string(), 100);
        cmd_map.insert("conf_velocity".to_string(), 101);
        cmd_map.insert("conf_position".to_string(), 102);
        cmd_map.insert("conf_torque".to_string(), 103);

        cmd_map.insert("conf_torque_pid".to_string(), 104);
        cmd_map.insert("conf_velocity_pid".to_string(), 105);
        cmd_map.insert("conf_position_pid".to_string(), 106);

        cmd_map.insert("conf_limit".to_string(), 107);
        cmd_map.insert("conf_voltage_offset".to_string(), 108);

        cmd_map.insert("stream_states".to_string(), 150);
        cmd_map.insert("stream_q".to_string(), 151);
        cmd_map.insert("stream_current".to_string(), 152);
        cmd_map.insert("stream_time".to_string(), 153);

        Self { map: cmd_map }
    }

    pub fn key2id(&mut self, key: &str) -> u8 {
        match self.map.get(key).copied() {
            Some(id) => id,
            None => 0,
        }
    }
}

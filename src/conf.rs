use crate::cmd::KeyCMD;

pub struct Conf {
    pub loop_ms: u64,
    pub i2c_sleep_us: u64,

    pub is_print_serial: bool,
    pub is_send_mqtt: bool,
    pub is_mqtt_conf: bool,
    pub is_fire_water: bool,
    pub is_stream_q: bool,
    pub is_stream_states: bool,
    pub is_stream_current: bool,
    pub is_stream_time: bool,
}

impl Conf {
    pub fn new() -> Self {
        Self {
            loop_ms: 2000,
            i2c_sleep_us: 250,

            is_print_serial: true,
            is_send_mqtt: false,
            is_mqtt_conf: true,
            is_fire_water: false,
            is_stream_q: false,
            is_stream_states: true,
            is_stream_current: false,
            is_stream_time: false,
        }
    }

    pub fn update(&mut self, key_cmd: KeyCMD) -> bool {
        let key = key_cmd.key;
        let val = key_cmd.val;
        let switch = if val >= 1.0 { true } else { false };

        if key == "enable" {
            self.is_mqtt_conf = switch;
        } else if key == "loop_sleep_ms" && val > 10.0 && val < 2000.0 {
            self.loop_ms = val as u64;
        } else if key == "i2c_sleep_us" && val > 50.0 && val < 1000.0 {
            self.i2c_sleep_us = val as u64;
        } else if key == "is_send_mqtt" {
            self.is_send_mqtt = switch;
        } else if key == "is_print_serial" {
            self.is_print_serial = switch;
        } else if key == "is_fire_water" {
            self.is_fire_water = switch;
        } else if key == "is_stream_q" {
            self.is_stream_q = switch;
        } else if key == "is_stream_states" {
            self.is_stream_states = switch;
        } else if key == "is_stream_current" {
            self.is_stream_current = switch;
        } else if key == "is_stream_debug" {
            self.is_stream_time = switch;
        } else {
            return false;
        }

        true
    }
}

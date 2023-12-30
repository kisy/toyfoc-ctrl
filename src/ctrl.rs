use embedded_hal::blocking::i2c::{Read, Write, WriteRead};
use serde::{Deserialize, Serialize};

use crate::cmd::{CmdMap, CMD};

#[derive(Debug)]
pub enum FocCtrlError {
    WriteErr,
    ReadErr,
    WriteReadErr,
    WriteReadMatchErr,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RxData {
    pub motor_id: u8,
    pub cmd_id: u8,
    pub cmd_key: String,
    pub first: f32,
    pub second: f32,
    pub third: f32,
}

pub trait FocCtrlInterface {
    fn send_cmd(&mut self, cmd: CMD) -> Result<(), FocCtrlError>;
    fn read_data(&mut self) -> Result<RxData, FocCtrlError>;

    fn exchange(&mut self, cmd_id: u8) -> Result<RxData, FocCtrlError>;

    fn read_q(&mut self) -> Result<RxData, FocCtrlError>;
    fn read_current(&mut self) -> Result<RxData, FocCtrlError>;
    fn read_states(&mut self) -> Result<RxData, FocCtrlError>;

    fn read_time(&mut self) -> Result<RxData, FocCtrlError>;
}

#[derive(Debug)]
pub struct FocCtrl<I2C> {
    pub i2c: I2C,
    pub addr: u8,
    pub cmd_map: CmdMap,
}

#[derive(Serialize, Copy, Clone, Debug)]
pub struct MotorStates {
    pub motor_id: u8,
    pub ts: f32,

    pub uq: f32,
    pub id: f32,
    pub iq: f32,

    pub velocity: f32,
    pub position: f32,
    pub torque: f32,

    pub iu: f32,
    pub iv: f32,
    pub iw: f32,

    pub debug: f32,
}

impl<I2C, E> FocCtrl<I2C>
where
    I2C: Read<Error = E> + Write<Error = E> + WriteRead<Error = E>,
{
    pub fn new(i2c: I2C, addr: u8) -> Self {
        Self {
            i2c,
            addr,
            cmd_map: CmdMap::new(),
        }
    }

    pub fn loop_cmd(&mut self, cmd: CMD) -> Result<RxData, FocCtrlError> {
        let mut exchange_data = RxData {
            motor_id: self.addr,
            cmd_id: 0,
            cmd_key: "".to_string(),
            first: 0.0,
            second: 0.0,
            third: 0.0,
        };
        if cmd.id > 0 && cmd.id < 100 {
            self.send_cmd(cmd)?;
        } else if cmd.id >= 100 && cmd.id < 200 {
            exchange_data = self.exchange(cmd.id)?;
        }

        // ctrl-device conf
        if cmd.id >= 200 {
            exchange_data.first = cmd.val;
        }

        Ok(exchange_data)
    }
}

impl<I2C, E> FocCtrlInterface for FocCtrl<I2C>
where
    I2C: Read<Error = E> + Write<Error = E> + WriteRead<Error = E>,
{
    fn send_cmd(&mut self, cmd: CMD) -> Result<(), FocCtrlError> {
        let mut buf = [0u8; 5];
        buf[0] = cmd.id;
        buf[1..=4].copy_from_slice(cmd.val.to_le_bytes().as_ref());
        self.i2c
            .write(self.addr, &buf)
            .map_err(|_| FocCtrlError::WriteErr)
    }

    fn read_data(&mut self) -> Result<RxData, FocCtrlError> {
        let mut buf = [0u8; 20];

        // self.i2c
        //     .write_read(self.addr, &[cmd_id], &mut buf)
        //     .map_err(|_| FocCtrlError::ReadErr)?;

        self.i2c
            .read(self.addr, &mut buf)
            .map_err(|_| FocCtrlError::ReadErr)?;

        let cmd_id = buf[0];
        let f32_first = f32::from_le_bytes(buf[1..5].try_into().unwrap());
        let f32_second = f32::from_le_bytes(buf[5..9].try_into().unwrap());
        let f32_third = f32::from_le_bytes(buf[9..13].try_into().unwrap());

        Ok(RxData {
            motor_id: self.addr,
            cmd_id,
            cmd_key: "".to_string(),
            first: f32_first,
            second: f32_second,
            third: f32_third,
        })
    }

    fn exchange(&mut self, cmd_id: u8) -> Result<RxData, FocCtrlError> {
        let mut buf = [0u8; 13];

        match self.i2c.write_read(self.addr, &[cmd_id], &mut buf) {
            Ok(_) => {}
            Err(_e) => {
                return Err(FocCtrlError::WriteReadErr);
            }
        }

        let new_cmd_id = buf[0];

        if new_cmd_id != cmd_id {
            return Err(FocCtrlError::WriteReadMatchErr);
        }

        let f32_first = f32::from_le_bytes(buf[1..5].try_into().unwrap());
        let f32_second = f32::from_le_bytes(buf[5..9].try_into().unwrap());
        let f32_third = f32::from_le_bytes(buf[9..13].try_into().unwrap());

        Ok(RxData {
            motor_id: self.addr,
            cmd_id: new_cmd_id,
            cmd_key: "".to_string(),
            first: f32_first,
            second: f32_second,
            third: f32_third,
        })
    }

    fn read_q(&mut self) -> Result<RxData, FocCtrlError> {
        let cmd_id = self.cmd_map.key2id("stream_q");
        self.exchange(cmd_id)
    }

    fn read_current(&mut self) -> Result<RxData, FocCtrlError> {
        let cmd_id = self.cmd_map.key2id("stream_current");
        self.exchange(cmd_id)
    }

    fn read_states(&mut self) -> Result<RxData, FocCtrlError> {
        let cmd_id = self.cmd_map.key2id("stream_states");
        self.exchange(cmd_id)
    }

    fn read_time(&mut self) -> Result<RxData, FocCtrlError> {
        let cmd_id = self.cmd_map.key2id("stream_time");
        self.exchange(cmd_id)
    }
}

pub fn format_seconds(seconds: u64) -> String {
    let mut timer_string = String::new();

    let days = seconds / 86400;
    if days > 0 {
        timer_string.push_str(format!("{}d", days).as_str());
    }
    let hours = (seconds - days * 86400) / 3600;
    if hours > 0 {
        timer_string.push_str(format!("{}h", hours).as_str());
    }
    let minutes = (seconds - days * 86400 - hours * 3600) / 60;
    if minutes > 0 {
        timer_string.push_str(format!("{}m", minutes).as_str());
    }
    let seconds = seconds - days * 86400 - hours * 3600 - minutes * 60;

    timer_string.push_str(format!("{:.0}s", seconds).as_str());

    timer_string
}

use std::io::Write;
use std::mem;
use crate::types::*;

pub mod models200;
pub mod models700;

#[derive(Debug, Clone, Copy)]
pub struct SunspecID {
    pub id: [u8; 4]
}

#[derive(Debug, Clone)]
pub struct Model1 {
    pub start_addr: u16,
    pub model_number: u16,
    pub qtd: u16,
    pub manufacturer: FixString,
    pub model: FixString,
    pub options: FixString,
    pub version: FixString,
    pub serial_number: FixString,
    pub device_address: u16,
    pub pad: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct ModelEnd {
    pub model_number: u16,
    pub qtd: u16
}

// Declare the struct
pub trait Models {
    // This new function acts as a constructor
    fn new () -> Self;
}

impl Models for SunspecID {
    fn new () -> SunspecID {
        let mut ret = SunspecID {
            id: [0; 4],
        };
        srt_to_vec_u8("SunS", &mut ret.id);
        return ret;
    }
}

impl Models for Model1 {
    fn new () -> Model1 {
        let ret = Model1 {
            start_addr: 0,
            model_number: 1,
            qtd: 66,
            manufacturer: FixString { length: 16, value: String::new() },
            model: FixString { length: 16, value: String::new() },
            options: FixString { length: 8, value: String::new() },
            version: FixString { length: 8, value: String::new() },
            serial_number: FixString { length: 16, value: String::new() },
            device_address: 0,
            pad: 0,
        };
        return ret;
    }
}

impl Models for ModelEnd {

    fn new () -> ModelEnd {
        let ret = ModelEnd {
            model_number: 0xFFFF,
            qtd: 0,
        };
        return ret;
    }
}

fn vec_u8_to_vec_u16(src: &[u8], dst: &mut Vec<u16>, mut idx: usize, size: usize) -> usize {
    for i in (0..size).step_by(2) {
        dst[idx] = (src[i] as u16) << 8;
        dst[idx] += src[i+1] as u16;
        idx += 1;
    }
    idx
}

pub fn srt_to_vec_u8(src: &str, mut dst: &mut [u8]){
    dst.write(src.as_bytes()).unwrap();
}

impl From<SunspecID> for Vec<u16> {
    fn from(from: SunspecID) -> Self {
        let size = mem::size_of::<SunspecID>() / 2;
        let mut registers: Vec<u16> = vec![0; size];
        
        vec_u8_to_vec_u16(&from.id, &mut registers, 0, from.id.len());
        registers
    }
}

impl From<Vec<u16>> for SunspecID {
    fn from(from: Vec<u16>) -> Self {
        let mut sunspec = SunspecID::new();
        sunspec.id[0] = (from[0] >> 8) as u8;
        sunspec.id[1] = (from[0] & 0xFF) as u8;
        sunspec.id[2] = (from[1] >> 8) as u8;
        sunspec.id[3] = (from[1] & 0xFF) as u8;
        sunspec
    }
}

impl From<FixString> for Vec<u16> {
    fn from(from: FixString) -> Self {
        let mut regs = String::encode(from.value);
        for _i in 0..(from.length-(regs.len() as u16)){
            regs.push(0);
        }
        regs
    }
}

impl From<Model1> for Vec<u16> {
    fn from(from: Model1) -> Self {
        let mut registers: Vec<u16> = vec![0; 2];
        registers[0] = from.model_number;
        registers[1] = from.qtd;

        registers.extend(Vec::<u16>::from(from.manufacturer));
        registers.extend(Vec::<u16>::from(from.model));
        registers.extend(Vec::<u16>::from(from.options));
        registers.extend(Vec::<u16>::from(from.version));
        registers.extend(Vec::<u16>::from(from.serial_number));

        registers.push(from.device_address);
        registers.push(from.pad);
        registers
    }
}

impl From<(Vec<u16>, u16, &Model1)> for Model1 {
    fn from(from: (Vec<u16>, u16, &Model1)) -> Self {
        let mut model1 = from.2.clone();
        let mut offset = from.1 - model1.start_addr;
        let mut idx = 0;
        loop {
            match offset {
                66 => model1.device_address = from.0[idx],
                _ => {}
            }
            offset += 1;
            idx += 1;
            if from.0.len() == idx {
                break;
            }
        }
        model1
    }
}

impl From<ModelEnd> for Vec<u16> {
    fn from(from: ModelEnd) -> Self {
        let size = mem::size_of::<ModelEnd>() / 2;
        let mut registers: Vec<u16> = vec![0; size];
        registers[0] = from.model_number;
        registers[1] = from.qtd;
        registers
    }
}

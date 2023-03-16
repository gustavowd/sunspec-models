use std::io::Write;
use std::mem;

pub mod models200;
pub mod models700;


#[derive(Debug, Clone, Copy)]
pub struct SunspecID {
    pub id: [u8; 4]
}

#[derive(Debug, Clone, Copy)]
pub struct Model1 {
    pub model_number: u16,
    pub qtd: u16,
    pub manufacturer: [u8; 32],
    pub model: [u8; 32],
    pub options: [u8; 16],
    pub version: [u8; 16],
    pub serial_number: [u8; 32],
    pub device_address: u16,
    pub pad: u16
}

#[derive(Debug, Clone, Copy)]
pub struct ModelEnd {
    pub model_number: u16,
    pub qtd: u16
}

// Declare the struct
pub trait Sunspec {
    // This new function acts as a constructor
    // allowing us to add additional logic to instantiating a struct
    // This particular method belongs to the trait
    fn new () -> Self;
}

impl Sunspec for SunspecID {
    fn new () -> SunspecID {
        let mut ret = SunspecID {
            id: [0; 4],
        };
        srt_to_vec_u8("SunS", &mut ret.id);
        return ret;
    }
}

impl Sunspec for Model1 {
    fn new () -> Model1 {
        let ret = Model1 {
            model_number: 1,
            qtd: 66,
            manufacturer: [0; 32],
            model: [0; 32],
            options: [0; 16],
            version: [0; 16],
            serial_number: [0; 32],
            device_address: 0,
            pad: 0,
        };
        return ret;
    }
}



impl Sunspec for ModelEnd {

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

fn f32_to_vec_u16(src: f32, dst: &mut Vec<u16>, idx: usize) {
    let tmp = src.to_bits();
    dst[idx] = (tmp >> 16) as u16;
    dst[idx+1] = (tmp & 0xFFFF) as u16;
}

fn u32_to_vec_u16(src: u32, dst: &mut Vec<u16>, idx: usize) {
    dst[idx] = (src >> 16) as u16;
    dst[idx+1] = (src & 0xFFFF) as u16;
}

fn u64_to_vec_u16(src: u64, dst: &mut Vec<u16>, idx: usize) {
    dst[idx] = (src >> 48) as u16;
    dst[idx+1] = ((src >> 32) & 0xFFFF) as u16;
    dst[idx+2] = ((src >> 24) & 0xFFFF) as u16;
    dst[idx+3] = (src & 0xFFFF) as u16;
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

impl From<Model1> for Vec<u16> {
    fn from(from: Model1) -> Self {
        let size = mem::size_of::<Model1>() / 2;
        let mut registers: Vec<u16> = vec![0; size];
        registers[0] = from.model_number;
        registers[1] = from.qtd;
        
        let mut idx = vec_u8_to_vec_u16(&from.manufacturer, &mut registers, 2, from.manufacturer.len());
        idx = vec_u8_to_vec_u16(&from.model, &mut registers, idx, from.model.len());
        idx = vec_u8_to_vec_u16(&from.options, &mut registers, idx, from.options.len());
        idx = vec_u8_to_vec_u16(&from.version, &mut registers, idx, from.version.len());
        idx = vec_u8_to_vec_u16(&from.serial_number, &mut registers, idx, from.serial_number.len());
        registers[idx] = from.device_address;
        registers[idx+1] = from.pad;
        registers
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

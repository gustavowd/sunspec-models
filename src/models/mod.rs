use std::io::Write;
use std::mem;
use crate::types::*;

pub mod models200;
pub mod models700;

pub const SUNSPEC_INIT_ADDR: u16 = 40000;

#[derive(Debug, Clone, Copy)]
pub struct SunspecID {
    pub id: [u8; 4]
}

#[derive(Debug, Clone)]
pub struct Model {
    pub start_addr: u16,
    pub end_addr: u16,
    pub model_number: u16,
    pub qtd: u16,
    pub data: Vec<DataTypes>,
}

#[derive(Debug, Clone)]
pub struct Models {
    pub id: SunspecID,
    pub models: Vec<Model>,
}

// Declare the struct
pub trait SunspecModels {
    // This new function acts as a constructor
    fn new (model_number: u16) -> Self;
    fn update_data(&mut self, point: &str, value: &DataTypes);
    fn get_data(&mut self, point: &str) -> DataTypes;
    fn get_string(&mut self, point: &str) -> Option<String>;
    fn get_f32(&mut self, point: &str) -> Option<f32>;
    fn get_u16(&mut self, point: &str) -> Option<u16>;
    fn get_u32(&mut self, point: &str) -> Option<u32>;
    fn get_u64(&mut self, point: &str) -> Option<u64>;
    fn get_u128(&mut self, point: &str) -> Option<u128>;
    fn get_i16(&mut self, point: &str) -> Option<i16>;
    fn get_i32(&mut self, point: &str) -> Option<i32>;
    fn get_i64(&mut self, point: &str) -> Option<i64>;
}


impl SunspecID {
    fn new () -> SunspecID {
        let mut ret = SunspecID {
            id: [0; 4],
        };
        srt_to_vec_u8("SunS", &mut ret.id);
        return ret;
    }
}

impl Models {
    pub fn new () -> Models {
        Models { id: SunspecID::new(), models: Vec::new() }
    }
    pub fn compute_addr (&mut self) {
        let mut idx = 0;
        let mut end_addr = 0;
        for model in self.models.iter_mut() {
            if idx == 0 {
                model.start_addr = SUNSPEC_INIT_ADDR + 2;
            }else{
                model.start_addr = end_addr;
            }
            idx += 1;
            model.end_addr = model.start_addr + model.qtd + 2;
            end_addr = model.end_addr;
        }
    }
}

fn model1() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 1,
        qtd: 66,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecString(Point { name: "Mn", offset: 2, length: 16, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "Md", offset: 18, length: 16, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "Opt", offset: 34, length: 8, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "Ver", offset: 42, length: 8, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "SN", offset: 50, length: 16, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DA", offset: 66, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Pad", offset: 67, length: 1, write_access: false, value: 0 } ));
    ret
}

fn model_end() -> Model {
    Model {
        start_addr: 0,
        model_number: 0xFFFF,
        qtd: 0,
        end_addr: 0,
        data: Vec::new(),
    }
}

impl SunspecModels for Model {
    fn new (model_number: u16) -> Model {
        match model_number {
            1 => return model1(),
            213 => return models200::model213(),
            701 => return models700::model701(),
            _ => return model_end(),
        }
    }

    fn update_data(&mut self, point: &str, value: &DataTypes) {
        for data in self.data.iter_mut() {
            match data {
                DataTypes::SunspecString(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()){
                        match value {
                            DataTypes::SunspecString(update_value) =>  data.value = update_value.value.clone(),
                            _ => {},
                        };
                    }
                },
                DataTypes::SunspecF32(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()){
                        match value {
                            DataTypes::SunspecF32(update_value) =>  data.value = update_value.value,
                            _ => {},
                        };
                    }
                },
                DataTypes::SunspecU16(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()){
                        match value {
                            DataTypes::SunspecU16(update_value) =>  data.value = update_value.value,
                            _ => {},
                        };
                    }
                },
                DataTypes::SunspecU32(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()){
                        match value {
                            DataTypes::SunspecU32(update_value) =>  data.value = update_value.value,
                            _ => {},
                        };
                    }
                },
                DataTypes::SunspecU64(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()){
                        match value {
                            DataTypes::SunspecU64(update_value) =>  data.value = update_value.value,
                            _ => {},
                        };
                    }
                },
                DataTypes::SunspecU128(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()){
                        match value {
                            DataTypes::SunspecU128(update_value) =>  data.value = update_value.value,
                            _ => {},
                        };
                    }
                },
                DataTypes::SunspecI16(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()){
                        match value {
                            DataTypes::SunspecI16(update_value) =>  data.value = update_value.value,
                            _ => {},
                        };
                    }
                },
                DataTypes::SunspecI32(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()){
                        match value {
                            DataTypes::SunspecI32(update_value) =>  data.value = update_value.value,
                            _ => {},
                        };
                    }
                },
                DataTypes::SunspecI64(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()){
                        match value {
                            DataTypes::SunspecI64(update_value) =>  data.value = update_value.value,
                            _ => {},
                        };
                    }
                },
            }
        }
    }

    fn get_data(&mut self, point: &str) -> DataTypes {
        for data in self.data.iter() {
            match data {
                DataTypes::SunspecString(data2) => {
                    if data2.name.contains(point){
                        return data.clone();
                    }
                },
                DataTypes::SunspecU16(data2) => {
                    if data2.name.contains(point){
                        return data.clone();
                    }
                },
                DataTypes::SunspecU32(data2) => {
                    if data2.name.contains(point){
                        return data.clone();
                    }
                },
                DataTypes::SunspecU64(data2) => {
                    if data2.name.contains(point){
                        return data.clone();
                    }
                },
                DataTypes::SunspecU128(data2) => {
                    if data2.name.contains(point){
                        return data.clone();
                    }
                },
                DataTypes::SunspecI16(data2) => {
                    if data2.name.contains(point){
                        return data.clone();
                    }
                },
                DataTypes::SunspecI32(data2) => {
                    if data2.name.contains(point){
                        return data.clone();
                    }
                },
                DataTypes::SunspecI64(data2) => {
                    if data2.name.contains(point){
                        return data.clone();
                    }
                },
                DataTypes::SunspecF32(data2) => {
                    if data2.name.contains(point){
                        return data.clone();
                    }
                }
            };
        }
        return DataTypes::SunspecU16(Point { name: "", offset: 0, length: 1, write_access: false, value: 0 } )
    }

    fn get_f32(&mut self, point: &str) -> Option<f32> {
        for data in self.data.iter() {
            match data {
                DataTypes::SunspecF32(data2) => {
                    if data2.name.contains(point){
                        return Some(data2.value);
                    }
                },
                _ => return None,
            }
        }
        return None
    }

    fn get_string(&mut self, point: &str) -> Option<String> {
        for data in self.data.iter() {
            match data {
                DataTypes::SunspecString(data2) => {
                    if data2.name.contains(point){
                        return Some(data2.value.clone());
                    }
                },
                _ => return None,
            }
        }
        return None
    }

    fn get_u16(&mut self, point: &str) -> Option<u16> {
        for data in self.data.iter() {
            match data {
                DataTypes::SunspecU16(data2) => {
                    if data2.name.contains(point){
                        return Some(data2.value);
                    }
                },
                _ => return None,
            }
        }
        return None
    }

    fn get_u32(&mut self, point: &str) -> Option<u32> {
        for data in self.data.iter() {
            match data {
                DataTypes::SunspecU32(data2) => {
                    if data2.name.contains(point){
                        return Some(data2.value);
                    }
                },
                _ => return None,
            }
        }
        return None
    }

    fn get_u64(&mut self, point: &str) -> Option<u64> {
        for data in self.data.iter() {
            match data {
                DataTypes::SunspecU64(data2) => {
                    if data2.name.contains(point){
                        return Some(data2.value);
                    }
                },
                _ => return None,
            }
        }
        return None
    }

    fn get_u128(&mut self, point: &str) -> Option<u128> {
        for data in self.data.iter() {
            match data {
                DataTypes::SunspecU128(data2) => {
                    if data2.name.contains(point){
                        return Some(data2.value);
                    }
                },
                _ => return None,
            }
        }
        return None
    }

    fn get_i16(&mut self, point: &str) -> Option<i16> {
        for data in self.data.iter() {
            match data {
                DataTypes::SunspecI16(data2) => {
                    if data2.name.contains(point){
                        return Some(data2.value);
                    }
                },
                _ => return None,
            }
        }
        return None
    }

    fn get_i32(&mut self, point: &str) -> Option<i32> {
        for data in self.data.iter() {
            match data {
                DataTypes::SunspecI32(data2) => {
                    if data2.name.contains(point){
                        return Some(data2.value);
                    }
                },
                _ => return None,
            }
        }
        return None
    }

    fn get_i64(&mut self, point: &str) -> Option<i64> {
        for data in self.data.iter() {
            match data {
                DataTypes::SunspecI64(data2) => {
                    if data2.name.contains(point){
                        return Some(data2.value);
                    }
                },
                _ => return None,
            }
        }
        return None
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

impl From<Model> for Vec<u16> {
    fn from(from: Model) -> Self {
        let mut registers: Vec<u16> = vec![0; 2];
        registers[0] = from.model_number;
        registers[1] = from.qtd;

        for data in from.data.iter() {
            match data {
                DataTypes::SunspecF32(data) => registers.extend(f32::encode(data.value)),
                DataTypes::SunspecU16(data) => registers.extend(u16::encode(data.value)),
                DataTypes::SunspecU32(data) => registers.extend(u32::encode(data.value)),
                DataTypes::SunspecU64(data) => registers.extend(u64::encode(data.value)),
                DataTypes::SunspecU128(data) => registers.extend(u128::encode(data.value)),
                DataTypes::SunspecI16(data) => registers.extend(i16::encode(data.value)),
                DataTypes::SunspecI32(data) => registers.extend(i32::encode(data.value)),
                DataTypes::SunspecI64(data) => registers.extend(i64::encode(data.value)),
                DataTypes::SunspecString(data) => registers.extend(Point::<String>::encode(data.clone())),
            }
        }
        registers
    }
}

impl From<(Vec<u16>, u16, u16, &Model)> for Model {
    fn from(from: (Vec<u16>, u16, u16, &Model)) -> Self {
        let mut model1 = from.3.clone();
        let mut offset = from.1 - model1.start_addr;
        let mut qtd = from.2;

        while qtd > 0 {
            for data in model1.data.iter_mut() {
                match data {
                    DataTypes::SunspecString(data) => {
                        if offset == data.offset {
                            data.value = Point::<String>::decode(from.0.clone()).value;
                            offset += data.offset;
                            qtd -= data.length;
                        }
                    },
                    DataTypes::SunspecU16(data) => {
                        if offset == data.offset {
                            data.value = u16::decode(from.0.clone());
                            offset += data.offset;
                            qtd -= data.length;
                        }
                    },
                    DataTypes::SunspecU32(data) => {
                        if offset == data.offset {
                            data.value = u32::decode(from.0.clone());
                            offset += data.offset;
                            qtd -= data.length;
                        }
                    },
                    DataTypes::SunspecU64(data) => {
                        if offset == data.offset {
                            data.value = u64::decode(from.0.clone());
                            offset += data.offset;
                            qtd -= data.length;
                        }
                    },
                    DataTypes::SunspecU128(data) => {
                        if offset == data.offset {
                            data.value = u128::decode(from.0.clone());
                            offset += data.offset;
                            qtd -= data.length;
                        }
                    },
                    DataTypes::SunspecI16(data) => {
                        if offset == data.offset {
                            data.value = i16::decode(from.0.clone());
                            offset += data.offset;
                            qtd -= data.length;
                        }
                    },
                    DataTypes::SunspecI32(data) => {
                        if offset == data.offset {
                            data.value = i32::decode(from.0.clone());
                            offset += data.offset;
                            qtd -= data.length;
                        }
                    },
                    DataTypes::SunspecI64(data) => {
                        if offset == data.offset {
                            data.value = i64::decode(from.0.clone());
                            offset += data.offset;
                            qtd -= data.length;
                        }
                    },
                    _ => {}
                }
            }
        }
        model1
    }
}

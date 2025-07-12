use std::io::Write;
use std::mem;
use crate::types::*;

pub mod model1;
pub mod model2;
pub mod model3;
pub mod model4;
pub mod model5;
pub mod model6;
pub mod model7;
pub mod model8;
pub mod model9;
pub mod model10;
pub mod model11;
pub mod model12;
pub mod model13;
pub mod model14;
pub mod model15;
pub mod model16;
pub mod model17;
pub mod model18;
pub mod model19;
pub mod model101;
pub mod model102;
pub mod model103;
pub mod model111;
pub mod model112;
pub mod model113;
pub mod model120;
pub mod model121;
pub mod model122;
pub mod model123;
pub mod model124;
pub mod model125;
pub mod model126;
pub mod model127;
pub mod model128;
pub mod model129;
pub mod model130;
pub mod model131;
pub mod model132;
pub mod model133;
pub mod model134;
pub mod model135;
pub mod model136;
pub mod model137;
pub mod model138;
pub mod model139;
pub mod model140;
pub mod model141;
pub mod model142;
pub mod model143;
pub mod model144;
pub mod model145;
pub mod model160;
pub mod model201;
pub mod model202;
pub mod model203;
pub mod model204;
pub mod model211;
pub mod model212;
pub mod model213;
pub mod model214;
pub mod model220;
pub mod model302;
pub mod model303;
pub mod model304;
pub mod model305;
pub mod model306;
pub mod model307;
pub mod model308;
pub mod model401;
pub mod model402;
pub mod model403;
pub mod model404;
pub mod model501;
pub mod model502;
pub mod model601;
pub mod model701;
pub mod model702;
pub mod model703;
pub mod model704;
pub mod model705;
pub mod model706;
pub mod model707;
pub mod model708;
pub mod model709;
pub mod model710;
pub mod model711;
pub mod model712;
pub mod model801;
pub mod model802;
pub mod model803;
pub mod model804;
pub mod model805;
pub mod model806;
pub mod model807;
pub mod model808;
pub mod model809;
pub mod model63001;
pub mod model63002;
pub mod model64001;
pub mod model64020;
pub mod model64061;
pub mod model64101;
pub mod model64110;
pub mod model64111;
pub mod model64112;
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
    pub update: bool,
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
    fn update_data_by_index(&mut self, index: usize, value: &DataTypes);
    fn get_data(&self, point: &str) -> DataTypes;
    fn get_data_index(&self, point: &str) -> Option<usize>;
    fn get_string(&self, point: &str) -> Option<String>;
    fn get_string_by_index(&self, idx: usize) -> Option<String>;
    fn get_f32(&self, point: &str) -> Option<f32>;
    fn get_f32_by_index(&self, idx: usize) -> Option<f32>;
    fn get_u16(&self, point: &str) -> Option<u16>;
    fn get_u16_by_index(&self, idx: usize) -> Option<u16>;
    fn get_u32(&self, point: &str) -> Option<u32>;
    fn get_u32_by_index(&self, idx: usize) -> Option<u32>;
    fn get_u64(&self, point: &str) -> Option<u64>;
    fn get_u64_by_index(&self, idx: usize) -> Option<u64>;
    fn get_u128(&self, point: &str) -> Option<u128>;
    fn get_u128_by_index(&self, idx: usize) -> Option<u128>;
    fn get_i16(&self, point: &str) -> Option<i16>;
    fn get_i16_by_index(&self, idx: usize) -> Option<i16>;
    fn get_i32(&self, point: &str) -> Option<i32>;
    fn get_i32_by_index(&self, idx: usize) -> Option<i32>;
    fn get_i64(&self, point: &str) -> Option<i64>;
    fn get_i64_by_index(&self, idx: usize) -> Option<i64>;
    fn print(&self);
}


impl SunspecID {
    fn new () -> SunspecID {
        let mut ret = SunspecID {
            id: [0; 4],
        };
        srt_to_vec_u8("SunS", &mut ret.id).unwrap();
        ret
    }
}

impl Models {
    pub fn new () -> Models {
        Models { id: SunspecID::new(), models: Vec::new() }
    }

    pub fn get_model_index(&self, model_number: u16) -> Option<usize> {
        for (idx, model) in self.models.iter().enumerate() {
            if model_number == model.model_number {
                return Some(idx);
            }
        }
        None
    }

    pub fn compute_addr (&mut self) {
        let mut end_addr = 0;
        for (idx, model) in self.models.iter_mut().enumerate() {
            if idx == 0 {
                model.start_addr = SUNSPEC_INIT_ADDR + 2;
            }else{
                model.start_addr = end_addr;
            }
            model.end_addr = model.start_addr + model.qtd + 2;
            end_addr = model.end_addr;
        }
    }
}

fn model_end() -> Model {
    Model {
        start_addr: 0,
        model_number: 0xFFFF,
        qtd: 0,
        end_addr: 0,
        update: false,
        data: Vec::new(),
    }
}

impl SunspecModels for Model {
    fn new (model_number: u16) -> Model {
        match model_number {
            1 => model1::model1(),
            2 => model2::model2(),
            3 => model3::model3(),
            4 => model4::model4(),
            5 => model5::model5(),
            6 => model6::model6(),
            7 => model7::model7(),
            8 => model8::model8(),
            9 => model9::model9(),
            10 => model10::model10(),
            11 => model11::model11(),
            12 => model12::model12(),
            13 => model13::model13(),
            14 => model14::model14(),
            15 => model15::model15(),
            16 => model16::model16(),
            17 => model17::model17(),
            18 => model18::model18(),
            19 => model19::model19(),
            101 => model101::model101(),
            102 => model102::model102(),
            103 => model103::model103(),
            111 => model111::model111(),
            112 => model112::model112(),
            113 => model113::model113(),
            120 => model120::model120(),
            121 => model121::model121(),
            122 => model122::model122(),
            123 => model123::model123(),
            124 => model124::model124(),
            125 => model125::model125(),
            126 => model126::model126(),
            127 => model127::model127(),
            128 => model128::model128(),
            129 => model129::model129(),
            130 => model130::model130(),
            131 => model131::model131(),
            132 => model132::model132(),
            133 => model133::model133(),
            134 => model134::model134(),
            135 => model135::model135(),
            136 => model136::model136(),
            137 => model137::model137(),
            138 => model138::model138(),
            139 => model139::model139(),
            140 => model140::model140(),
            141 => model141::model141(),
            142 => model142::model142(),
            143 => model143::model143(),
            144 => model144::model144(),
            145 => model145::model145(),
            160 => model160::model160(),
            201 => model201::model201(),
            202 => model202::model202(),
            203 => model203::model203(),
            204 => model204::model204(),
            211 => model211::model211(),
            212 => model212::model212(),
            213 => model213::model213(),
            214 => model214::model214(),
            220 => model220::model220(),
            302 => model302::model302(),
            303 => model303::model303(),
            304 => model304::model304(),
            305 => model305::model305(),
            306 => model306::model306(),
            307 => model307::model307(),
            308 => model308::model308(),
            401 => model401::model401(),
            402 => model402::model402(),
            403 => model403::model403(),
            404 => model404::model404(),
            501 => model501::model501(),
            502 => model502::model502(),
            601 => model601::model601(),
            701 => model701::model701(),
            702 => model702::model702(),
            703 => model703::model703(),
            704 => model704::model704(),
            705 => model705::model705(),
            706 => model706::model706(),
            707 => model707::model707(),
            708 => model708::model708(),
            709 => model709::model709(),
            710 => model710::model710(),
            711 => model711::model711(),
            712 => model712::model712(),
            801 => model801::model801(),
            802 => model802::model802(),
            803 => model803::model803(),
            804 => model804::model804(),
            805 => model805::model805(),
            806 => model806::model806(),
            807 => model807::model807(),
            808 => model808::model808(),
            809 => model809::model809(),
            63001 => model63001::model63001(),
            63002 => model63002::model63002(),
            64001 => model64001::model64001(),
            64020 => model64020::model64020(),
            64101 => model64101::model64101(),
            64110 => model64110::model64110(),
            64111 => model64111::model64111(),
            64112 => model64112::model64112(),
            _ => model_end(),
        }
    }

    fn update_data(&mut self, point: &str, value: &DataTypes) {
        for data_tmp in self.data.iter_mut() {
            match data_tmp {
                DataTypes::SunspecString(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()){
                        if let DataTypes::SunspecString(update_value) = value {
                            data.value = update_value.value.clone();
                        }
                    }
                },
                DataTypes::SunspecF32(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()){
                        if let DataTypes::SunspecF32(update_value) = value {
                            data.value = update_value.value;
                        }
                    }
                },
                DataTypes::SunspecU16(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()){
                        if let DataTypes::SunspecU16(update_value) = value {
                            data.value = update_value.value;
                        }
                    }
                },
                DataTypes::SunspecU32(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()){
                        if let DataTypes::SunspecU32(update_value) = value {
                            data.value = update_value.value;
                        }
                    }
                },
                DataTypes::SunspecU64(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()){
                        if let DataTypes::SunspecU64(update_value) = value {
                            data.value = update_value.value;
                        }
                    }
                },
                DataTypes::SunspecU128(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()){
                        if let DataTypes::SunspecU128(update_value) = value {
                            data.value = update_value.value;
                        }
                    }
                },
                DataTypes::SunspecI16(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()){
                        if let DataTypes::SunspecI16(update_value) = value {
                            data.value = update_value.value;
                        }
                    }
                },
                DataTypes::SunspecI32(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()){
                        if let DataTypes::SunspecI32(update_value) = value {
                            data.value = update_value.value;
                        }
                    }
                },
                DataTypes::SunspecI64(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()){
                        if let DataTypes::SunspecI64(update_value) = value {
                            data.value = update_value.value;
                        }
                    }
                },
            }
        }
    }

    fn update_data_by_index(&mut self, index: usize, value: &DataTypes) {
        match &mut self.data[index] {
            DataTypes::SunspecString(data) => {
                if let DataTypes::SunspecString(update_value) = value {
                    data.value = update_value.value.clone();
                }
            },
            DataTypes::SunspecF32(data) => {
                if let DataTypes::SunspecF32(update_value) = value {
                    data.value = update_value.value;
                }
            },
            DataTypes::SunspecU16(data) => {
                if let DataTypes::SunspecU16(update_value) = value {
                    data.value = update_value.value;
                }
            },
            DataTypes::SunspecU32(data) => {
                if let DataTypes::SunspecU32(update_value) = value {
                    data.value = update_value.value;
                }
            },
            DataTypes::SunspecU64(data) => {
                if let DataTypes::SunspecU64(update_value) = value {
                    data.value = update_value.value;
                }
            },
            DataTypes::SunspecU128(data) => {
                if let DataTypes::SunspecU128(update_value) = value {
                    data.value = update_value.value;
                }
            },
            DataTypes::SunspecI16(data) => {
                if let DataTypes::SunspecI16(update_value) = value {
                    data.value = update_value.value;
                }
            },
            DataTypes::SunspecI32(data) => {
                if let DataTypes::SunspecI32(update_value) = value {
                    data.value = update_value.value;
                }
            },
            DataTypes::SunspecI64(data) => {
                if let DataTypes::SunspecI64(update_value) = value {
                    data.value = update_value.value;
                }
            },
        }
    }

    fn get_data(&self, point: &str) -> DataTypes {
        for data_tmp in self.data.iter() {
            match data_tmp {
                DataTypes::SunspecString(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return data_tmp.clone();
                    }
                },
                DataTypes::SunspecU16(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return data_tmp.clone();
                    }
                },
                DataTypes::SunspecU32(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return data_tmp.clone();
                    }
                },
                DataTypes::SunspecU64(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return data_tmp.clone();
                    }
                },
                DataTypes::SunspecU128(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return data_tmp.clone();
                    }
                },
                DataTypes::SunspecI16(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return data_tmp.clone();
                    }
                },
                DataTypes::SunspecI32(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return data_tmp.clone();
                    }
                },
                DataTypes::SunspecI64(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return data_tmp.clone();
                    }
                },
                DataTypes::SunspecF32(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return data_tmp.clone();
                    }
                }
            };
        }
        DataTypes::SunspecU16(Point { name: "", offset: 0, length: 1, write_access: false, value: 0 } )
    }

    fn get_data_index(&self, point: &str) -> Option<usize> {
        for (idx, data_tmp) in self.data.iter().enumerate() {
            match data_tmp {
                DataTypes::SunspecString(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return Some(idx);
                    }
                },
                DataTypes::SunspecU16(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return Some(idx);
                    }
                },
                DataTypes::SunspecU32(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return Some(idx);
                    }
                },
                DataTypes::SunspecU64(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return Some(idx);
                    }
                },
                DataTypes::SunspecU128(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return Some(idx);
                    }
                },
                DataTypes::SunspecI16(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return Some(idx);
                    }
                },
                DataTypes::SunspecI32(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return Some(idx);
                    }
                },
                DataTypes::SunspecI64(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return Some(idx);
                    }
                },
                DataTypes::SunspecF32(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return Some(idx);
                    }
                }
            };
        }
        None
    }

    fn get_f32(&self, point: &str) -> Option<f32> {
        for data_tmp in self.data.iter() {
            if let DataTypes::SunspecF32(data) = data_tmp {
                if data.name.contains(point) && (data.name.len() == point.len()) {
                    return Some(data.value);
                }
            }
        }
        None
    }

    fn get_f32_by_index(&self, idx: usize) -> Option<f32> {
        match self.data[idx] {
            DataTypes::SunspecF32(data) => {
                Some(data.value)
            },
            _ => None
        }
    }

    fn get_string(&self, point: &str) -> Option<String> {
        for data_tmp in self.data.iter() {
            if let DataTypes::SunspecString(data) = data_tmp {
                if data.name.contains(point) && (data.name.len() == point.len()) {
                    return Some(data.value.clone());
                }
            }
        }
        None
    }

    fn get_string_by_index(&self, idx: usize) -> Option<String> {
        match &self.data[idx] {
            DataTypes::SunspecString(data) => {
                Some(data.value.clone())
            },
            _ => None
        }
    }

    fn get_u16(&self, point: &str) -> Option<u16> {
        for data_tmp in self.data.iter() {
            if let DataTypes::SunspecU16(data) = data_tmp {
                if data.name.contains(point) && (data.name.len() == point.len()) {
                    return Some(data.value);
                }
            }
        }
        None
    }

    fn get_u16_by_index(&self, idx: usize) -> Option<u16> {
        match self.data[idx] {
            DataTypes::SunspecU16(data) => {
                Some(data.value)
            },
            _ => None
        }
    }

    fn get_u32(&self, point: &str) -> Option<u32> {
        for data_tmp in self.data.iter() {
            if let DataTypes::SunspecU32(data) = data_tmp {
                if data.name.contains(point) && (data.name.len() == point.len()) {
                    return Some(data.value);
                }
            }
        }
        None
    }

    fn get_u32_by_index(&self, idx: usize) -> Option<u32> {
        match self.data[idx] {
            DataTypes::SunspecU32(data) => {
                Some(data.value)
            },
            _ => None
        }
    }

    fn get_u64(&self, point: &str) -> Option<u64> {
        for data_tmp in self.data.iter() {
            if let DataTypes::SunspecU64(data) = data_tmp {
                if data.name.contains(point) && (data.name.len() == point.len()) {
                    return Some(data.value);
                }
            }
        }
        None
    }

    fn get_u64_by_index(&self, idx: usize) -> Option<u64> {
        match self.data[idx] {
            DataTypes::SunspecU64(data) => {
                Some(data.value)
            },
            _ => None
        }
    }

    fn get_u128(&self, point: &str) -> Option<u128> {
        for data_tmp in self.data.iter() {
            if let DataTypes::SunspecU128(data) = data_tmp {
                if data.name.contains(point) && (data.name.len() == point.len()) {
                    return Some(data.value);
                }
            }
        }
        None
    }

    fn get_u128_by_index(&self, idx: usize) -> Option<u128> {
        match self.data[idx] {
            DataTypes::SunspecU128(data) => {
                Some(data.value)
            },
            _ => None
        }
    }

    fn get_i16(&self, point: &str) -> Option<i16> {
        for data_tmp in self.data.iter() {
            if let DataTypes::SunspecI16(data) = data_tmp {
                if data.name.contains(point) && (data.name.len() == point.len()) {
                    return Some(data.value);
                }
            }
        }
        None
    }

    fn get_i16_by_index(&self, idx: usize) -> Option<i16> {
        match self.data[idx] {
            DataTypes::SunspecI16(data) => {
                Some(data.value)
            },
            _ => None
        }
    }

    fn get_i32(&self, point: &str) -> Option<i32> {
        for data_tmp in self.data.iter() {
            if let DataTypes::SunspecI32(data) = data_tmp {
                if data.name.contains(point) && (data.name.len() == point.len()) {
                    return Some(data.value);
                }
            }
        }
        None
    }

    fn get_i32_by_index(&self, idx: usize) -> Option<i32> {
        match self.data[idx] {
            DataTypes::SunspecI32(data) => {
                Some(data.value)
            },
            _ => None
        }
    }

    fn get_i64(&self, point: &str) -> Option<i64> {
        for data_tmp in self.data.iter() {
            if let DataTypes::SunspecI64(data) = data_tmp {
                if data.name.contains(point) && (data.name.len() == point.len()) {
                    return Some(data.value);
                }
            }
        }
        None
    }

    fn get_i64_by_index(&self, idx: usize) -> Option<i64> {
        match self.data[idx] {
            DataTypes::SunspecI64(data) => {
                Some(data.value)
            },
            _ => None
        }
    }

    fn print(&self) {
        println!("Model {}:", self.model_number);
        for data in self.data.iter() {
            match data {
                DataTypes::SunspecF32(data) => println!("{}: {}", data.name, data.value),
                DataTypes::SunspecU16(data) => println!("{}: {}", data.name, data.value),
                DataTypes::SunspecU32(data) => println!("{}: {}", data.name, data.value),
                DataTypes::SunspecU64(data) => println!("{}: {}", data.name, data.value),
                DataTypes::SunspecU128(data) => println!("{}: {}", data.name, data.value),
                DataTypes::SunspecI16(data) => println!("{}: {}", data.name, data.value),
                DataTypes::SunspecI32(data) => println!("{}: {}", data.name, data.value),
                DataTypes::SunspecI64(data) => println!("{}: {}", data.name, data.value),
                DataTypes::SunspecString(data) => println!("{}: {}", data.name, data.value.clone()),
            }
        }
        println!(" ");
    }
}


fn vec_u8_to_vec_u16(src: &[u8], dst: &mut [u16], mut idx: usize, size: usize) -> usize {
    for i in (0..size).step_by(2) {
        dst[idx] = (src[i] as u16) << 8;
        dst[idx] += src[i+1] as u16;
        idx += 1;
    }
    idx
}

pub fn srt_to_vec_u8(src: &str, mut dst: &mut [u8]) -> Result<usize, std::io::Error> {
    dst.write(src.as_bytes())
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

        let mut regs = from.0.clone();

        while qtd > 0 {
            for data in model1.data.iter_mut() {
                match data {
                    DataTypes::SunspecString(data) => {
                        if (offset == data.offset) && (data.write_access) {
                            data.value = Point::<String>::decode(regs.clone()).value;
                            offset += data.length;
                            qtd -= data.length;
                            for _i in 0..data.length {
                                regs.remove(0);
                            }
                        }
                    },
                    DataTypes::SunspecU16(data) => {
                        if (offset == data.offset) && (data.write_access) {
                            data.value = u16::decode(regs.clone());
                            offset += data.length;
                            qtd -= data.length;
                            for _i in 0..data.length {
                                regs.remove(0);
                            }
                        }
                    },
                    DataTypes::SunspecU32(data) => {
                        if (offset == data.offset) && (data.write_access) {
                            data.value = u32::decode(regs.clone());
                            offset += data.length;
                            qtd -= data.length;
                            for _i in 0..data.length {
                                regs.remove(0);
                            }
                        }
                    },
                    DataTypes::SunspecU64(data) => {
                        if (offset == data.offset) && (data.write_access) {
                            data.value = u64::decode(regs.clone());
                            offset += data.length;
                            qtd -= data.length;
                            for _i in 0..data.length {
                                regs.remove(0);
                            }
                        }
                    },
                    DataTypes::SunspecU128(data) => {
                        if (offset == data.offset) && (data.write_access) {
                            data.value = u128::decode(regs.clone());
                            offset += data.length;
                            qtd -= data.length;
                            for _i in 0..data.length {
                                regs.remove(0);
                            }
                        }
                    },
                    DataTypes::SunspecI16(data) => {
                        if (offset == data.offset) && (data.write_access) {
                            data.value = i16::decode(regs.clone());
                            offset += data.length;
                            qtd -= data.length;
                            for _i in 0..data.length {
                                regs.remove(0);
                            }
                        }
                    },
                    DataTypes::SunspecI32(data) => {
                        if (offset == data.offset) && (data.write_access) {
                            data.value = i32::decode(regs.clone());
                            offset += data.length;
                            qtd -= data.length;
                            for _i in 0..data.length {
                                regs.remove(0);
                            }
                        }
                    },
                    DataTypes::SunspecI64(data) => {
                        if (offset == data.offset) && (data.write_access) {
                            data.value = i64::decode(regs.clone());
                            offset += data.length;
                            qtd -= data.length;
                            for _i in 0..data.length {
                                regs.remove(0);
                            }
                        }
                    },
                    _ => {}
                }
                if qtd == 0 {
                    break;
                }
            }
        }
        model1
    }
}

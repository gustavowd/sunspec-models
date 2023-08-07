use super::*;

pub fn model63001() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 63001,
        qtd: 152,
        update: false,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecI16(Point { name: "sunssf_1", offset: 2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "sunssf_2", offset: 1+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "sunssf_3", offset: 2+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "sunssf_4", offset: 3+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "int16_1", offset: 4+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "int16_2", offset: 5+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "int16_3", offset: 6+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "int16_4", offset: 7+2, length: 1, write_access: true, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "int16_5", offset: 8+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "int16_u", offset: 9+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "uint16_1", offset: 10+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "uint16_2", offset: 11+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "uint16_3", offset: 12+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "uint16_4", offset: 13+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "uint16_5", offset: 14+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "uint16_u", offset: 15+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "acc16", offset: 16+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "acc16_u", offset: 17+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "enum16", offset: 18+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "enum16_u", offset: 19+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "bitfield16", offset: 20+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "bitfield16_u", offset: 21+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecI32(Point { name: "int32_1", offset: 22+2, length: 2, write_access: false, value: -2147483648i32 } ));
    ret.data.push(DataTypes::SunspecI32(Point { name: "int32_2", offset: 24+2, length: 2, write_access: false, value: -2147483648i32 } ));
    ret.data.push(DataTypes::SunspecI32(Point { name: "int32_3", offset: 26+2, length: 2, write_access: true, value: -2147483648i32 } ));
    ret.data.push(DataTypes::SunspecI32(Point { name: "int32_4", offset: 28+2, length: 2, write_access: false, value: -2147483648i32 } ));
    ret.data.push(DataTypes::SunspecI32(Point { name: "int32_5", offset: 30+2, length: 2, write_access: false, value: -2147483648i32 } ));
    ret.data.push(DataTypes::SunspecI32(Point { name: "int32_u", offset: 32+2, length: 2, write_access: false, value: -2147483648i32 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "uint32_1", offset: 34+2, length: 2, write_access: false, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "uint32_2", offset: 36+2, length: 2, write_access: false, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "uint32_3", offset: 38+2, length: 2, write_access: true, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "uint32_4", offset: 40+2, length: 2, write_access: false, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "uint32_5", offset: 42+2, length: 2, write_access: false, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "uint32_u", offset: 44+2, length: 2, write_access: false, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "acc32", offset: 46+2, length: 2, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "acc32_u", offset: 48+2, length: 2, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "enum32", offset: 50+2, length: 2, write_access: false, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "enum32_u", offset: 52+2, length: 2, write_access: false, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "bitfield32", offset: 54+2, length: 2, write_access: false, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "bitfield32_u", offset: 56+2, length: 2, write_access: false, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "ipaddr", offset: 58+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "ipaddr_u", offset: 60+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI64(Point { name: "int64", offset: 62+2, length: 4, write_access: true, value: -9223372036854775808i64 } ));
    ret.data.push(DataTypes::SunspecI64(Point { name: "int64_u", offset: 66+2, length: 4, write_access: false, value: -9223372036854775808i64 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "acc64", offset: 70+2, length: 4, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU64(Point { name: "acc64_u", offset: 74+2, length: 4, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU128(Point { name: "ipv6addr", offset: 78+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU128(Point { name: "ipv6addr_u", offset: 86+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "float32", offset: 94+2, length: 2, write_access: true, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "float32_u", offset: 96+2, length: 2, write_access: false, value: f32::NAN } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "string", offset: 98+2, length: 16, write_access: true, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "string_u", offset: 114+2, length: 16, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "sunssf_5", offset: 130+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "sunssf_6", offset: 131+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "sunssf_7", offset: 132+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "pad_1", offset: 133+2, length: 1, write_access: false, value: 0x8000 } ));
    
    ret
}
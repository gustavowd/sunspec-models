use super::*;

pub fn model6() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 6,
        qtd: 91,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecU16(Point { name: "X", offset: 0+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Off", offset: 1+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val1", offset: 2+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val2", offset: 3+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val3", offset: 4+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val4", offset: 5+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val5", offset: 6+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val6", offset: 7+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val7", offset: 8+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val8", offset: 9+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val9", offset: 10+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val10", offset: 11+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val11", offset: 12+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val12", offset: 13+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val13", offset: 14+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val14", offset: 15+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val15", offset: 16+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val16", offset: 17+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val17", offset: 18+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val18", offset: 19+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val19", offset: 20+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val20", offset: 21+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val21", offset: 22+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val22", offset: 23+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val23", offset: 24+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val24", offset: 25+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val25", offset: 26+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val26", offset: 27+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val27", offset: 28+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val28", offset: 29+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val29", offset: 30+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val30", offset: 31+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val31", offset: 32+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val32", offset: 33+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val33", offset: 34+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val34", offset: 35+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val35", offset: 36+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val36", offset: 37+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val37", offset: 38+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val38", offset: 39+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val39", offset: 40+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val40", offset: 41+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val41", offset: 42+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val42", offset: 43+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val43", offset: 44+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val44", offset: 45+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val45", offset: 46+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val46", offset: 47+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val47", offset: 48+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val48", offset: 49+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val49", offset: 50+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val50", offset: 51+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val51", offset: 52+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val52", offset: 53+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val53", offset: 54+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val54", offset: 55+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val55", offset: 56+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val56", offset: 57+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val57", offset: 58+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val58", offset: 59+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val59", offset: 60+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val60", offset: 61+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val61", offset: 62+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val62", offset: 63+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val63", offset: 64+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val64", offset: 65+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val65", offset: 66+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val66", offset: 67+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val67", offset: 68+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val68", offset: 69+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val69", offset: 70+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val70", offset: 71+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val71", offset: 72+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val72", offset: 73+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val73", offset: 74+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val74", offset: 75+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val75", offset: 76+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val76", offset: 77+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val77", offset: 78+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val78", offset: 79+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val79", offset: 80+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Val80", offset: 81+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "Ts", offset: 82+2, length: 1, write_access: true, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Ms", offset: 84+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Seq", offset: 85+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Role", offset: 86+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Rsrvd", offset: 87+2, length: 1, write_access: true, value: 0x8000 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Alg", offset: 88+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "N", offset: 89+2, length: 1, write_access: true, value: 0xFFFF } ));
    
    ret
}
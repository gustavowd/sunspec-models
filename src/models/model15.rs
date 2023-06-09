use super::*;

pub fn model15() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 15,
        qtd: 24,
        update: false,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecU16(Point { name: "Clr", offset: 0+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "InCnt", offset: 1+2, length: 2, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "InUcCnt", offset: 3+2, length: 2, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "InNUcCnt", offset: 5+2, length: 2, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "InDscCnt", offset: 7+2, length: 2, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "InErrCnt", offset: 9+2, length: 2, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "InUnkCnt", offset: 11+2, length: 2, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "OutCnt", offset: 13+2, length: 2, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "OutUcCnt", offset: 15+2, length: 2, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "OutNUcCnt", offset: 17+2, length: 2, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "OutDscCnt", offset: 19+2, length: 2, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "OutErrCnt", offset: 21+2, length: 2, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Pad", offset: 23+2, length: 1, write_access: false, value: 0x8000 } ));
    
    ret
}
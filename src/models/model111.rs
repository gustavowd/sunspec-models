use super::*;

pub fn model111() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 111,
        qtd: 60,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecF32(Point { name: "A", offset: 0+2, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "AphA", offset: 2+2, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "AphB", offset: 4+2, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "AphC", offset: 6+2, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "PPVphAB", offset: 8+2, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "PPVphBC", offset: 10+2, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "PPVphCA", offset: 12+2, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "PhVphA", offset: 14+2, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "PhVphB", offset: 16+2, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "PhVphC", offset: 18+2, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "W", offset: 20+2, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "Hz", offset: 22+2, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "VA", offset: 24+2, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "VAr", offset: 26+2, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "PF", offset: 28+2, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "WH", offset: 30+2, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "DCA", offset: 32+2, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "DCV", offset: 34+2, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "DCW", offset: 36+2, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TmpCab", offset: 38+2, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TmpSnk", offset: 40+2, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TmpTrns", offset: 42+2, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecF32(Point { name: "TmpOt", offset: 44+2, length: 2, write_access: false, value: 0.0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "St", offset: 46+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "StVnd", offset: 47+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "Evt1", offset: 48+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "Evt2", offset: 50+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "EvtVnd1", offset: 52+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "EvtVnd2", offset: 54+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "EvtVnd3", offset: 56+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "EvtVnd4", offset: 58+2, length: 1, write_access: false, value: 0 } ));
    
    ret
}
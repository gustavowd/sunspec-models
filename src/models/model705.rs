use super::*;

pub fn model705() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 705,
        qtd: 43,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecU16(Point { name: "Ena", offset: 0+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "AdptCrvReq", offset: 1+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "AdptCrvRslt", offset: 2+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "NPt", offset: 3+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "NCrv", offset: 4+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "RvrtTms", offset: 5+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "RvrtRem", offset: 7+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "RvrtCrv", offset: 9+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "V_SF", offset: 10+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DeptRef_SF", offset: 11+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "RspTms_SF", offset: 12+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DERVoltVar.Crv.Pt.ActPt", offset: 13+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DERVoltVar.Crv.Pt.DeptRef", offset: 14+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DERVoltVar.Crv.Pt.Pri", offset: 15+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DERVoltVar.Crv.Pt.VRef", offset: 16+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DERVoltVar.Crv.Pt.VRefAuto", offset: 17+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DERVoltVar.Crv.Pt.VRefAutoEna", offset: 18+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DERVoltVar.Crv.Pt.VRefAutoTms", offset: 19+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "DERVoltVar.Crv.Pt.RspTms", offset: 20+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DERVoltVar.Crv.Pt.ReadOnly", offset: 22+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DERVoltVar.Crv.Pt.V_P1", offset: 23+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "DERVoltVar.Crv.Pt.Var_P1", offset: 24+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DERVoltVar.Crv.Pt.V_P2", offset: 25+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "DERVoltVar.Crv.Pt.Var_P2", offset: 26+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DERVoltVar.Crv.Pt.V_P3", offset: 27+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "DERVoltVar.Crv.Pt.Var_P3", offset: 28+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DERVoltVar.Crv.Pt.V_P4", offset: 29+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "DERVoltVar.Crv.Pt.Var_P4", offset: 30+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DERVoltVar.Crv.Pt.V_P5", offset: 31+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "DERVoltVar.Crv.Pt.Var_P5", offset: 32+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DERVoltVar.Crv.Pt.V_P6", offset: 33+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "DERVoltVar.Crv.Pt.Var_P6", offset: 34+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DERVoltVar.Crv.Pt.V_P7", offset: 35+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "DERVoltVar.Crv.Pt.Var_P7", offset: 36+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DERVoltVar.Crv.Pt.V_P8", offset: 37+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "DERVoltVar.Crv.Pt.Var_P8", offset: 38+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DERVoltVar.Crv.Pt.V_P9", offset: 39+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "DERVoltVar.Crv.Pt.Var_P9", offset: 40+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DERVoltVar.Crv.Pt.V_P10", offset: 41+2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "DERVoltVar.Crv.Pt.Var_P10", offset: 42+2, length: 1, write_access: true, value: 0 } ));
    
    ret
}
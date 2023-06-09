use super::*;

pub fn model710() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 710,
        qtd: 23,
        update: false,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecU16(Point { name: "Ena", offset: 0+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "AdptCrvReq", offset: 1+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "AdptCrvRslt", offset: 2+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "NPt", offset: 3+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "NCrvSet", offset: 4+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "Hz_SF", offset: 5+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "Tms_SF", offset: 6+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DERTripHF.Crv.ReadOnly", offset: 7+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DERTripHF.Crv.MustTrip.ActPt", offset: 8+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "DERTripHF.Crv.MustTrip.Pt.Hz", offset: 9+2, length: 2, write_access: true, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "DERTripHF.Crv.MustTrip.Pt.Tms", offset: 11+2, length: 2, write_access: true, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DERTripHF.Crv.MayTrip.ActPt", offset: 13+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "DERTripHF.Crv.MayTrip.Pt.Hz", offset: 14+2, length: 2, write_access: true, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "DERTripHF.Crv.MayTrip.Pt.Tms", offset: 16+2, length: 2, write_access: true, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "DERTripHF.Crv.MomCess.ActPt", offset: 18+2, length: 1, write_access: true, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "DERTripHF.Crv.MomCess.Pt.Hz", offset: 19+2, length: 2, write_access: true, value: 0xFFFFFFFF } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "DERTripHF.Crv.MomCess.Pt.Tms", offset: 21+2, length: 2, write_access: true, value: 0xFFFFFFFF } ));
    
    ret
}
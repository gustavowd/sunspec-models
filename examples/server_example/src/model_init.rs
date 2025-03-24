use sunspec_models::models::{Model, SunspecModels};
use sunspec_models::types::{DataTypes, SunspecTypes};

#[allow(unused)]
pub fn model_701_create_data(model701: &mut Model) {
	// update model data
	model701.update_data("A_SF", &DataTypes::new_i16(-1));
	model701.update_data("V_SF", &DataTypes::new_i16(-1));
	model701.update_data("Hz_SF", &DataTypes::new_i16(-2));
	model701.update_data("PF_SF", &DataTypes::new_i16(-3));
	model701.update_data("TotWh_SF", &DataTypes::new_i16(0));
	model701.update_data("TotVarh_SF", &DataTypes::new_i16(0));
	model701.update_data("Tmp_SF", &DataTypes::new_i16(-1));
	model701.update_data("W_SF", &DataTypes::new_i16(1));
	model701.update_data("VA_SF", &DataTypes::new_i16(1));
	model701.update_data("Var_SF", &DataTypes::new_i16(1));

	// Zero phase voltages
	model701.update_data("VL1", &DataTypes::new_u16(0));
	model701.update_data("VL2", &DataTypes::new_u16(0));
	model701.update_data("VL3", &DataTypes::new_u16(0));

	// define three phase inverter
	model701.update_data("ACType", &DataTypes::new_u16(2));
}


#[allow(unused)]
pub fn model_702_create_data(model702: &mut Model) {
	// update scale factor
	model702.update_data("W_SF", &DataTypes::new_i16(1));
	model702.update_data("VA_SF", &DataTypes::new_i16(1));
	model702.update_data("Var_SF", &DataTypes::new_i16(1));
	model702.update_data("PF_SF", &DataTypes::new_i16(-3));
	model702.update_data("V_SF", &DataTypes::new_i16(0));
	model702.update_data("A_SF", &DataTypes::new_i16(0));  
}

pub fn model_704_create_data(model704: &mut Model) {
	model704.update_data("WMaxLimPctEna",&DataTypes::new_u16(0));
	model704.update_data("WSetEna",&DataTypes::new_u16(0));
	model704.update_data("PFWInjEna",&DataTypes::new_u16(0));
	model704.update_data("VarSetEna",&DataTypes::new_u16(0));
	
	model704.update_data("WMaxLimPct", &DataTypes::new_u16(1100));
	model704.update_data("WSet", &DataTypes::new_i32(0));
	model704.update_data("WSetPct", &DataTypes::new_i16(0));
	model704.update_data("WSetMod", &DataTypes::new_u16(0));	// default mode: Active Power As Max Percent
	model704.update_data("DERCtlAC.PFWInj.PF", &DataTypes::new_u16(1000));
	model704.update_data("DERCtlAC.PFWInj.Ext", &DataTypes::new_u16(0));
	model704.update_data("VarSet", &DataTypes::new_i32(0));
	model704.update_data("VarSetPct", &DataTypes::new_i16(0));
	model704.update_data("VarSetMod", &DataTypes::new_u16(0)); // default mode: Reactive Power As Watt Max Pct

	model704.update_data("WSet_SF", &DataTypes::new_i16(1));
	model704.update_data("VarSet_SF", &DataTypes::new_i16(1));
	model704.update_data("PF_SF", &DataTypes::new_i16(-3));
	model704.update_data("WMaxLimPct_SF", &DataTypes::new_i16(-1));
	model704.update_data("WSetPct_SF", &DataTypes::new_i16(-1));
	model704.update_data("VarSetPct_SF", &DataTypes::new_i16(-1));
}

pub fn model_705_create_data(model705: &mut Model){
	model705.update_data("Ena", &DataTypes::new_u16(0));
	model705.update_data("NPt",&DataTypes::new_u16(10));
	model705.update_data("NCrv",&DataTypes::new_u16(1));
	model705.update_data("DERVoltVar.Crv.Pt.ActPt",&DataTypes::new_u16(10));
	model705.update_data("DERVoltVar.Crv.Pt.DeptRef", &DataTypes::new_u16(3));
	model705.update_data("DERVoltVar.Crv.Pt.V_P1", &DataTypes::new_u16(1000) );
	model705.update_data("DERVoltVar.Crv.Pt.Var_P1",&DataTypes::new_i16(0));
	model705.update_data("DERVoltVar.Crv.Pt.V_P2" , &DataTypes::new_u16(1000));
	model705.update_data("DERVoltVar.Crv.Pt.Var_P2",&DataTypes::new_i16(0));
	model705.update_data("DERVoltVar.Crv.Pt.V_P3", &DataTypes::new_u16(1000));
	model705.update_data("DERVoltVar.Crv.Pt.Var_P3",&DataTypes::new_i16(0));
	model705.update_data("DERVoltVar.Crv.Pt.V_P4" , &DataTypes::new_u16(1000));
	model705.update_data("DERVoltVar.Crv.Pt.Var_P4",&DataTypes::new_i16(0));
	model705.update_data("DERVoltVar.Crv.Pt.V_P5" , &DataTypes::new_u16(1000));
	model705.update_data("DERVoltVar.Crv.Pt.Var_P5",&DataTypes::new_i16(0));
	model705.update_data("DERVoltVar.Crv.Pt.V_P6" , &DataTypes::new_u16(1000));
	model705.update_data("DERVoltVar.Crv.Pt.Var_P6",&DataTypes::new_i16(0));
	model705.update_data("DERVoltVar.Crv.Pt.V_P7" , &DataTypes::new_u16(1000));
	model705.update_data("DERVoltVar.Crv.Pt.Var_P7",&DataTypes::new_i16(0));
	model705.update_data("DERVoltVar.Crv.Pt.V_P8" , &DataTypes::new_u16(1000));
	model705.update_data("DERVoltVar.Crv.Pt.Var_P8",&DataTypes::new_i16(0));
	model705.update_data("DERVoltVar.Crv.Pt.V_P9" , &DataTypes::new_u16(1000));
	model705.update_data("DERVoltVar.Crv.Pt.Var_P9",&DataTypes::new_i16(0));
	model705.update_data("DERVoltVar.Crv.Pt.V_P10", &DataTypes::new_u16(1000));
	model705.update_data("DERVoltVar.Crv.Pt.Var_P10",&DataTypes::new_i16(0));

	model705.update_data("V_SF", &DataTypes::new_i16(0));
	model705.update_data("DeptRef_SF", &DataTypes::new_i16(0));
}

use super::*;

pub fn model64112() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 64112,
        qtd: 64,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecU16(Point { name: "Port", offset: 0+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "V_SF", offset: 1+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "C_SF", offset: 2+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "H_SF", offset: 3+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "P_SF", offset: 4+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "AH_SF", offset: 5+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "KWH_SF", offset: 6+2, length: 1, write_access: false, value: -32768i16 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_fault", offset: 7+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_absorb_V", offset: 8+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_absorb_Hr", offset: 9+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_absorb_End_A", offset: 10+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_rebulk_V", offset: 11+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_float_V", offset: 12+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_max_Chg_A", offset: 13+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_equalize_V", offset: 14+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_equalize_Hr", offset: 15+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_auto_equalize", offset: 16+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_MPPT_mode", offset: 17+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_sweep_width", offset: 18+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_sweep_max", offset: 19+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_U_Pick_Duty_cyc", offset: 20+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_grid_tie", offset: 21+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_temp_comp", offset: 22+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_temp_comp_llimt", offset: 23+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_temp_comp_hlimt", offset: 24+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_auto_restart", offset: 25+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_wakeup_VOC", offset: 26+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_snooze_mode_A", offset: 27+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_wakeup_interval", offset: 28+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_AUX_mode", offset: 29+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_AUX_control", offset: 30+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_AUX_state", offset: 31+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_AUX_polarity", offset: 32+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_AUX_L_Batt_disc", offset: 33+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_AUX_L_Batt_rcon", offset: 34+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_AUX_L_Batt_dly", offset: 35+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_AUX_Vent_fan_V", offset: 36+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_AUX_PV_triggerV", offset: 37+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_AUX_PV_trg_h_tm", offset: 38+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_AUX_Nlite_ThrsV", offset: 39+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_AUX_Nlite_On_tm", offset: 40+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_AUX_Nlite_On_hist", offset: 41+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_AUX_Nlite_Off_hist", offset: 42+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_AUX_Error_batt_V", offset: 43+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_AUX_Divert_h_time", offset: 44+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_AUX_Divert_dly_time", offset: 45+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_AUX_Divert_Rel_V", offset: 46+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_AUX_Divert_Hyst_V", offset: 47+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_MajorFWRev", offset: 48+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_MidFWRev", offset: 49+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_MinorFWRev", offset: 50+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_DataLog_Day_offset", offset: 51+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_DataLog_Cur_Day_off", offset: 52+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_DataLog_Daily_AH", offset: 53+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_DataLog_Daily_KWH", offset: 54+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_DataLog_Max_Out_A", offset: 55+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_DataLog_Max_Out_W", offset: 56+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_DataLog_Absorb_T", offset: 57+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_DataLog_Float_T", offset: 58+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_DataLog_Min_Batt_V", offset: 59+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_DataLog_Max_Batt_V", offset: 60+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_DataLog_Max_Input_V", offset: 61+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_DataLog_Clear", offset: 62+2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "CC_Config_DataLog_Clr_Comp", offset: 63+2, length: 1, write_access: false, value: 0xFFFF } ));
    
    ret
}
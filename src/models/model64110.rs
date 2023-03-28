use super::*;

pub fn model64110() -> Model {
    let mut ret = Model {
        start_addr: 0,
        end_addr: 0,
        model_number: 64110,
        qtd: 282,
        data: Vec::new(),
    };
    ret.data.push(DataTypes::SunspecU16(Point { name: "MajorFWRev", offset: 0+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "MidFWRev", offset: 1+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "MinorFWRev", offset: 2+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "EncrypKey", offset: 3+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "MAC_Address", offset: 4+2, length: 7, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "WritePassword", offset: 11+2, length: 8, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "EnableDHCP", offset: 19+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TCPIP_address", offset: 20+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "Gateway_address", offset: 22+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "TCPIP_Netmask", offset: 24+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "DNS1_address", offset: 26+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU32(Point { name: "DNS2_address", offset: 28+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Modbus_port", offset: 30+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "SMTP_server_nm", offset: 31+2, length: 20, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "SMTP_account_nm", offset: 51+2, length: 16, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "SMTP_enable_SSL", offset: 67+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "SMTP_password", offset: 68+2, length: 8, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "SMTP_user_nm", offset: 76+2, length: 20, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Stat_email_int", offset: 96+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Stat_start_HR", offset: 97+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "Stat_email_sub", offset: 98+2, length: 25, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "Stat_email_addr1", offset: 123+2, length: 20, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "Stat_email_addr2", offset: 143+2, length: 20, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Alarm_email_en", offset: 163+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "Alarm_email_sub", offset: 164+2, length: 25, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "Alarm_email_addr1", offset: 189+2, length: 20, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "Alarm_email_addr2", offset: 209+2, length: 20, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "FTP_password", offset: 229+2, length: 8, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "TELNET_password", offset: 237+2, length: 8, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Log_write_int", offset: 245+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Log_retain", offset: 246+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Log_mode", offset: 247+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecString(Point { name: "NTP_server_nm", offset: 248+2, length: 20, write_access: false, value: String::new() } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "NTP_enable", offset: 268+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "TimeZone", offset: 269+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Date_year", offset: 270+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Date_month", offset: 271+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Date_Day", offset: 272+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Time_hour", offset: 273+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Time_minute", offset: 274+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Time_second", offset: 275+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "Battery_temp", offset: 276+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecI16(Point { name: "Ambient_temp", offset: 277+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "Temp_SF", offset: 278+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "AXS_Error", offset: 279+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "AXS_Status", offset: 280+2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(DataTypes::SunspecU16(Point { name: "AXS_Spare", offset: 281+2, length: 1, write_access: false, value: 0 } ));
    
    ret
}
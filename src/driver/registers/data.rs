use bitfield::bitfield;

bitfield! {
    pub struct MainConfig(u8);
    impl Debug;
    bool;
    power_down, set_power_down: 2;
    standby, set_standby: 1;
    start_conversion, set_start_conversion: 0;
}

bitfield! {
    pub struct LoopReadBackConfig(u8);
    impl Debug;
    bool;
    e3_en, set_e3_en: 6;
    e2_en, set_e2_en: 5;
    e1_en, set_e1_en: 4;
    p3_en, set_p3_en: 3;
    p2_en, set_p2_en: 2;
    p1_en, set_p1_en: 1;
    sts_en, set_sts_en: 0;
}

bitfield! {
    pub struct ErrorStatus(u8);
    impl Debug;
    bool;
    sync_error, set_sync_error: 7;
    ch3_error, set_ch3_error: 6;
    ch2_error, set_ch2_error: 5;
    ch1_error, set_ch1_error: 4;
    lead_off, set_lead_off: 3;
    low_battery, set_low_battery: 2;
    rld_near_rail, set_rld_near_rail: 1;
    common_mode_level_out_of_range, set_common_mode_level_out_of_range: 0;
}

bitfield! {
    #[derive(Clone, Copy)]
    pub struct DataStatus(u8);
    impl Debug;
    bool;
    pub e3_data_ready, set_e3_data_ready: 7;
    pub e2_data_ready, set_e2_data_ready: 6;
    pub e1_data_ready, set_e1_data_ready: 5;
    pub p3_data_ready, set_p3_data_ready: 4;
    pub p2_data_ready, set_p2_data_ready: 3;
    pub p1_data_ready, set_p1_data_ready: 2;
    pub is_alarm_active, set_alarm_active: 1;
}

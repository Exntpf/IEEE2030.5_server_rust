enum InverterStatusType {
    NA = 0,
    Off = 1,
    Sleeping = 2,
    Start_up = 3,
    Tracking_mppt = 4,
    Forced_power_reduction = 15,
    Shutting_down = 6,
    One_or_more_faults = 7,
    Standby = 8,
    Test_mode = 9,
    defined_in_manufacturer_status = 10,
}

enum LocalControlModeStatusType {
    Local_control = 0,
    Remote_control = 1,
}


enum OperationalModeStatusType {
    NA = 0,
    Off = 1,
    Operational_mode = 2,
    Test_mode = 3,
}


enum StorageModeStatusType {
    Storage_charging = 0,
    Storage_discharging = 1,
    Storage_holding = 2,
}

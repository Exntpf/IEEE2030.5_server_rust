
#[repr(UInt8)]
enum DERCurveType {
    OpModFreqWatt = 0,
    OpModHFRTMayTrip = 1,
    OpModHFRTMustTrip = 2,
    OpModHVRTMayTrip = 3,
    OpModHVRTMomentaryCessation = 4,
    OpModHVRTMustTrip = 5,
    OpModLFRTMayTrip = 6,
    OpModLFRTMustTrip = 7,
    OpModLVRTMayTrip = 8,
    OpModLVRTMomentaryCessation = 9,
    OpModLVRTMustTrip = 10,
    OpModVoltVar = 11,
    OpModVoltWatt = 12,
    OpModWattPF = 13,
    OpModWattVar = 14,
}


enum DERType {
    NaUnknown = 0,
    VirtualOrMixedDER = 1,
    ReciprocatingEngine = 2,
    FuelCell = 3,
    PvSystem = 4,
    CombinedHeatPower = 5,
    OtherPeneration = 6,
    OtherStorage = 80,
    ElectricVehicle = 81,
    EVSE = 82,
    CombinedPvStorage = 83,
}

enum InverterStatusType {
    NA = 0,
    Off = 1,
    Sleeping = 2,
    StartUp = 3,
    TrackingMppt = 4,
    ForcedPowerReduction = 15,
    ShuttingDown = 6,
    OneOrMoreFaults = 7,
    Standby = 8,
    TestMode = 9,
    DefinedInManufacturerStatus = 10,
}

enum LocalControlModeStatusType {
    LocalControl = 0,
    RemoteControl = 1,
}


enum OperationalModeStatusType {
    NA = 0,
    Off = 1,
    OperationalMode = 2,
    TestMode = 3,
}


enum StorageModeStatusType {
    StorageCharging = 0,
    StorageDischarging = 1,
    StorageHolding = 2,
}

# all python flags
AlarmStatusType
ConnectStatusType
DERControlType
DeviceCategoryType
FunctionsImplementedType
QualityFlagsType
ResponseRequiredType
RoleFlagsType

# all python enums
AccumlationBehaviourType
CommodityType
ConsumptionBlockType
CurrentStatusType
DataQualifierType
DERCurveType
DERType
DERUnitRefType
FlowDirectionType
InverterStatusType
KindType
OperationalModeStatusType
PhaseCode
PowerSourceType
ResponseStatusType
RtgAbnormalCategoryType
RtgNormalCategoryType
ServiceKind
StorageModeStatusType
SubscribableType
TimeQualityType
TOUType
UomType

# types we have to implement
ApplianceLoadReductionType
ChargeKind
CurrencyCode
PriorityType
UnitType

# Python flags:

class DeviceCategoryType(enum.Flag):
    Programmable_communicating_thermostat = 1
    Strip_heaters = 2
    Baseboard_heaters = 4
    Water_heater = 8
    Pool_pump = 16
    Sauna = 32
    Hot_tub = 64
    Smart_appliance = 128
    Irrigation_pump = 256
    Managed_commercial_and_industrial_loads = 512
    Simple_misc_loads = 1024
    Exterior_lighting = 2048
    Interior_lighting = 4096
    Load_control_switch = 8192
    Energy_management_system = 16384
    Smart_energy_module = 65536
    Electric_vehicle = 262144
    Virutal_or_mixed_der = 524288
    Reciprocating_engine = 2097152
    Photovoltaic_system = 8388608
    Combined_pv_and_storage = 16777216
    Other_generation_system = 33554432
    Other_storage_system = 67108864




class RoleFlagsType(enum.Flag)
    IsMirror = 1
    IsPremiseAggregationPoint = 2
    IsPEV = 4
    IsDER = 8
    IsRevenueQuality = 16
    IsDC = 32
    IsSubmeter = 64


class DERControlType(enum.Flag)
    Charge_mode = 1
    Discharge_mode = 2
    opModConnect = 4
    opModEnergize = 8
    opModFixedPFAbsorbW = 16
    opModFixedPFInjectW = 32
    opModFixedVar = 64
    opModFixedW = 128
    opModFreqDroop = 256
    opModFreqWatt = 512
    opModHFRTMayTrip = 1024
    opModHFRTMustTrip = 2048
    opModHVRTMayTrip = 4096
    opModHVRTMomentaryCessation = 8192
    opModHVRTMustTrip = 16384
    opModLFRTMayTrip = 32768

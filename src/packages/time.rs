/*
 * Time function set (B.2.8)
 */

struct Time {
    resource_data: ResourceData,
    current_time: TimeType,
    current_time: TimeType,
    dst_end_time: TimeType,
    dst_offset: TimeOffsetType,
    dst_start_time: TimeType,
    local_time: Option<TimeType>,
    quality: UInt8,
    tz_offset: TimeOffsetType,
    pollRate: Option<UInt32>, // default value = 900
}

impl Time {

}

impl Resource for Time {
    fn get_href() -> AnyURI {

    }
}

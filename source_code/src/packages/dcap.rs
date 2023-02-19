/*
 * Device Capability Resources
 */

use super::common::*;
pub fn handle_request(path: &str, method: &str, body: Option<&str>) -> (u32, Vec<u8>){
    let output = "<DeviceCapability href=\"/dcap\" xmlns=\"urn:ieee:std:2030.5:ns\">
    <DemandResponseProgramListLink all=\"1\" href=\"/drp\"/>
    <MessagingProgramListLink all=\"2\" href=\"/msg\"/>
    <EndDeviceListLink all=\"1\" href=\"/edev\"/>
    <SelfDeviceLink href=\"/sdev\"/>
</DeviceCapability>";
    return (200, output.as_bytes().to_vec());
}
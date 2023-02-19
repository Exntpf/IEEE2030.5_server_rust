/**
 * EndDevice Resource function set
 */ 

 /* 
 this is the function set we're going to implement first.
 all function set modules must have a function that takes the path and
 method as arguments and spits out a Vec<u8> which can be easily converted
 and sent via the tcp connection

fn edev_handle_connection(path: &str, method: &str) -> Vec<u8>{
match path{
    path1 => path1_handle( arguments ),
    path2 => path2_handle( arguments ),
    path3 => path3_handle( arguments ),
    _ => 400/404/405 message as Vec<u8>,
}
any common functionality that emerges from each path\d_handle
function goes here.
}

edev_handle( arguments ) -> Vec<u8>{
    // backend.rs guarantees that method here is valid. Not that that is 
    // actually required, since it is this file's job to ensure that every
    // path-method combo is accounted for in keeping with what has been implemented.
    // I guess it ensures that only wadl approved requests make it this far,
    // so if a dev implements a path-method combo they're not supposed to, 
    // that is stopped. Doesn't stop them from implementing supported combos
    // poorly though. Food for programmer thought.
}

 */
pub fn handle_request(path: &str, method: &str, body: Option<&str>) -> (u32, Vec<u8>){
    let output = "<EndDeviceList all=\"1\" href=\"/edev\" results=\"1\" subscribable=\"0\"
xmlns=\"urn:ieee:std:2030.5:ns\">
    <EndDevice href=\"/edev/3\" subscribable=\"0\">
        <ConfigurationLink href=\"/edev/3/cfg\"/>
        <DeviceInformationLink href=\"/edev/3/di\"/>
        <DeviceStatusLink href=\"/edev/3/ds\"/>
        <FileStatusLink href=\"/edev/3/fs\"/>
        <PowerStatusLink href=\"/edev/3/ps\"/>
        <sFDI>987654321005</sFDI>
        <changedTime>1379905200</changedTime>
        <FunctionSetAssignmentsListLink all=\"3\" href=\"/edev/3/fsal\"/>
        <RegistrationLink href=\"/edev/3/reg\"/>
        <SubscriptionListLink all=\"0\" href=\"/edev/3/subl\"/>
    </EndDevice>
</EndDeviceList>";
    return (200, output.as_bytes().to_vec());
}
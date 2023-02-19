/*
 * Function Set Assignmnets function set (B.2.5)
 */

use common::*;

struct FunctionSetAssignmentsBase {
    resource_data: ResourceData,
}

impl FunctionSetAssignmentsBase {
    fn new() -> FunctionSetAssignmentsBase{
        FunctionSetAssignmentsBase { 
            reource_data: ResourceData{ 
                href: AnyURI::new("/fsa") 
            }
        }
    }
}

// that's our macro right there. if we can check if there's a 
// resource_data field in the struct, or there's a field that 
// implements the Resource Trait, we can write #derive for this trait
impl Resource for FunctionSetAssignmentsBase {
    fn get_href(&self) -> AnyURI {
        self.resource_data.href
    }
}
/*
 * Function Set Assignmnets function set (B.2.5)
 */


use super::common::{
    identification::*,
    primitives::*,
    types::*,
    objects::*,
};

struct FunctionSetAssignmentsBase {
    super_class: ResourceObj,
}

impl FunctionSetAssignmentsBase {
    fn new() -> FunctionSetAssignmentsBase{
        FunctionSetAssignmentsBase { 
            super_class: ResourceObj::new(Some(String::from("/fsa"))),
        }
    }
}

// that's our macro right there. if we can check if there's a 
// resource_data field in the struct, or there's a field that 
// implements the Resource Trait, we can write #derive for this trait
impl Resource for FunctionSetAssignmentsBase {
    fn get_href(&self) -> Option<String> {
        self.super_class.get_href()
    }
}
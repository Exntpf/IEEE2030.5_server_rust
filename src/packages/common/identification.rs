/*
 * Defines traits and structs described generally in section 8.2 
 * and in detail in section B.2.3.2. specifically, describes the
 *  Resource trait and traits that extend it (e.g. List, IdentifiedObject)
 * 
 */
// Attribute data
#[derive(Default)]
struct AnyURI {
    elements: <String6>,
}

impl AnyURI {
    pub fn get_full_uri(&self) -> String {
        let mut output = String::new();
        for elem in &self.path {
            output.push_str("\\");
            output.push_str(&elem);
        }
        output
    }
}


// Traits
#[derive(Default)]
trait Resource {
    pub fn get_href(&self) -> AnyURI;
}

trait ListTrait {
    // TODO: ListTrait
}

trait LinkTrait {
    // TODO: LinkTrait
}

trait Respondable {
    pub fn get_replyTo(&self) -> Option<AnyURI>;
    pub fn get_responseRequired(&self) -> Option<HexBinary8>;
}

trait Subscribable {
    // TODO: Subscribable
}

trait Identified {
    // TODO: Identified
}

// Data Containers
#[derive(Default)]
struct ResourceData {
    href: AnyURI,
}

#[derive(Default)]
struct ListData {
    all: UInt32,
    result: UInt32,
}

// «XSDattribute»
#[derive(Default)]
struct LinkData {
    href: AnyURI,
}

#[derive(Default)]
struct RespondableData{
    reply_to: Option<AnyURI>,
    response_required: HexBinary8,
}

#[derive(Default)]
struct SubscribableData{
    subscribable: Option<SubscribableType>,
}

#[derive(Default)]
struct IdentifiedData {
    description: String32,
    mrid_type: mRIDType,
    version: VersionType,
}

// Resources
struct List {
    resource_data: ResourceData,
    list_data: ListData,
}

// impl ListTrait and LinkTrait
// TODO: (optional) could delve into macros and implement "derive(Link, List, Resource)"
// by using the fact that these objects all have an instance of something 
// that implements these traits (LinkData, ListData, ResourceData)
// would make this whole implementaiton process a lot faster and less verbose.
struct ListLink {
    link_data: LinkData,
    list_data: ListData,
}

struct RespondableResource {
    resource_data: ResourceData,
    respondable_data: RespondableData,
}

struct SubscribableResource {
    resource_data: ResourceData,
    subscribable_data: SubscribableData,
}

struct IdentifiedObject {
    resource_data: ResourceData,
    identified_data: IdentifiedData,
}

struct SubscribableIdentifiedObject {
    subscribable_resource: SubscribableResource,
    identified_data: IdentifiedData,
}

struct SubscribableList {
    subscribable_resource: SubscribableResource,
    list_data: ListData,
}

struct RespondableSubscribableIdentifiedObject {
    respondable_resource: RespondableResource,
    subscribable_data: SubscribableData,
    identified_data: IdentifiedData,
}

struct RespondableIdentifiedObject {
    respondable_resource: RespondableResource,
    identified_data: IdentifiedData,    
}


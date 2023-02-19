/*
 * Defines traits and structs described generally in section 8.2 
 * and in detail in section B.2.3.2. Specifically, it describes the
 *  Resource trait and traits that extend it (e.g. List, IdentifiedObject)
 */
/// Attribute data
/// was defined as a struct to allow for storage format flexibility 
/// but that was later decided to be pointless flexibility at the cost
/// of simplicty

/// anyURI mentioned in the specification can either be a relative 
/// address or an absolute reference (in the subs/notf function set)
/// there is no mention of how it should be implemented, so for the moment
/// it will be an owned String type.

// Traits
trait Resource {
    pub fn get_href(&self) -> Option<String>{
        None;
    }
}

trait List {
    type Inner; // every struct can only implement this trait for 1 type.
    pub fn get_values(s: UInt16, a: Option<TimeType>, l: UInt32) -> Vec<&Inner>; // need query parameters.
}

trait LinkTrait {
    pub fn get_href(&self) -> String{
        None;
    }
}

trait Respondable {
    pub fn get_replyTo(&self) -> Option<String>;
    pub fn get_responseRequired(&self) -> Option<HexBinary8>;
}

trait Subscribable {
    todo!("define Subscribable trait functions");
}

trait Identified {
    todo!("define Identified trait functions");
}

// Data Containers
#[derive(Default, Debug, Serialize, Deserialize)]
struct ResourceObj {
    href: Option<String>,
}

impl ResourceObj {
    /// input's that do not start with '/' or are >255 characters
    /// are ignored and href value will be None.
    fn new(href: Option<String>) -> ResourceObj{
        if let Some(ref href) = href{
            if !href.starts_with("/") || href.len() > 255{ 
                return ResourceObj{href: None} 
            }
        }
        ResourceObj{ href }
    }
}

impl Resource for ResourceObj {
    fn get_href(&self) -> Option<String>{
        if let Some(output) = &self.href{
            return Some(output.to_owned());
        }
        None
    }
}

/// List fields as per IEEE 2030.5 specification.
#[derive(Default, Serialize, Deserialize)]
struct ListData<T: Resource>{
    all: u32,
    result: u32,
    items: Vec<T>,
}

impl<T: Resource> ListData<T>{
    fn new() -> ListData<T>{
        ListData{ all: 0, result: 0, items: Vec::<T>::new() }
    }

    /// adds item onto end of ListData collection of `T`
    /// if `all` is greater than `result`, it's value is not affected.
    fn push(&mut self, item: T){
        self.items.push(item);
        self.result += 1;
        if self.all == self.result { self.all += 1 }
    }
    /// adds item onto end of ListData collection of `T`
    /// incremenets `all` and `result` values. If 
    fn push_and_increment(&mut self, item: T){
        self.items.push(item);
        self.result += 1;
        self.all += 1;
    }
    /// returns `Some(ListData[index])` if it exists, else `None`
    /// Decrements `result`. DOES NOT decrement `all`.
    /// checks for index out of bounds based on `result` number.
    fn remove(&mut self, index: u32) -> Option<T>{
        if index < self.result{
            let output = Some(self.items.remove(index.try_into().unwrap()));
            self.result -= 1;
            output
        } else {
            None
        }
    }
    /// returns `Some(ListData[index])` if it exists, else `None`
    /// Decrements `result`. DOES decrement `all`.
    /// checks for index out of bounds based on `result` number.
    fn remove_and_decrement(&mut self, index: u32) -> Option<T>{
        if index < self.result{
            let output = Some(self.items.remove(index.try_into().unwrap()));
            self.result -= 1;
            self.all -= 1;
            output
        } else {
            None
        }
    }
    /// removes an item from the ListData if it exists within the 
    fn remove_href(&mut self, href: &str) -> bool{
        
    }
    /// returns `Some(ListData[index])` if it exists, else `None`
    /// Decrements `result`. DOES NOT decrement `all`.
    /// checks for index out of bounds based on `result` number.
    fn pop(&mut self) -> Option<T>{
        let output = self.items.pop();
        if let Some(_) = output {
            self.result -= 1;
        }
        output
    }
    /// returns `Some(ListData[index])` if it exists, else `None`
    /// Decrements `result`. DOES decrement `all`.
    /// checks for index out of bounds based on `result` number.
    fn pop_and_decrement(&mut self) -> Option<T>{
        // storing in output because I dunno if this fails or not.
        let output = self.items.pop();
        if let Some(_) = output {
            self.result -= 1;
            self.all -= 1;
        }
        output
    }
    /// sets `self.all` to `all` if `all >= self.result`
    fn set_all_value(&mut self, all: u32){
        if all >= self.result{ self.all = all; }
        else { self.all = self.result }
    }
    fn get_result_value(&self) -> u32{
        self.result
    }
}

#[derive(Resource, Default, Serialize, Deserialize)]
struct ListObj<T: Resource> {
    super_class: ResourceObj,
    list_data: ListData<T>,
}

impl<T: Resource> ListObj<T>{
    fn new(href: &str) -> ListObj<T>{
        ListObj{ 
            super_class: ResourceObj::new(Some(href.to_owned())), 
            list_data: ListData::new(),
        }
    }
}

// «XSDattribute»
#[derive(Default, Serialize, Deserialize)]
struct LinkObj {
    href: String,
}

#[derive(Default, Serialize, Deserialize)]
struct RespondableData{
    reply_to: Option<String>,
    response_required: HexBinary8,
}

#[derive(Default, Serialize, Deserialize)]
struct SubscribableData{
    subscribable: Option<SubscribableType>,
}

#[derive(Default, Serialize, Deserialize)]
struct IdentifiedData {
    description: String32,
    mrid_type: mRIDType,
    version: VersionType,
}

#[derive(Link)]
struct ListLink {
    super_class: LinkObj,
    all: Option<UInt32>,
}

#[derive(Resource, Default, Debug, Serialize, Deserialize)]
struct RespondableResource {
    super_class: ResourceObj,
    respondable_data: RespondableData,
}

#[derive(Resource, Default, Debug, Serialize, Deserialize)]
struct SubscribableResource {
    super_class: ResourceObj,
    subscribable_data: SubscribableData,
}

#[derive(Resource, Default, Debug, Serialize, Deserialize)]
struct IdentifiedObject {
    super_class: ResourceObj,
    identified_data: IdentifiedData,
}

#[derive(Resource, Default, Debug, Serialize, Deserialize)]
struct SubscribableIdentifiedObject {
    super_class: SubscribableResource,
    identified_data: IdentifiedData,
}

#[derive(Resource, Default, Debug, Serialize, Deserialize)]
struct SubscribableList {
    super_class: SubscribableResource,
    list_data: ListData,
}

#[derive(Resource, Default, Debug, Serialize, Deserialize)]
struct RespondableSubscribableIdentifiedObject {
    super_class: RespondableResource,
    subscribable_data: SubscribableData,
    identified_data: IdentifiedData,
}

#[derive(Resource, Default, Debug, Serialize, Deserialize)]
struct RespondableIdentifiedObject {
    super_class: RespondableResource,
    identified_data: IdentifiedData,    
}


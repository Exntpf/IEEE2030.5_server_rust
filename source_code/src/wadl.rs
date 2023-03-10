/*
 * WADL checker for the 2030.5 protocol
 * sits between the rest of the server and the sep_wadl storage
 * supplied by 2030.5.
 * 
 * This allows the format of the wadl to be de-coupled from the rest
 * of the implementation
 */
use std::str;
use crate::xml::*;

const WADL_PATH: &str = "IEEE2030.5_server_rust/source_code/sep_wadl.xml";
const VALID_METHODS: [&str; 5]= ["GET", "HEAD", "PUT", "POST", "DELETE"];

pub enum Mode{
    Mandatory,
    Optional,
    Discouraged,
    Error,
}

/// given a path and method, returns the Mode of the request, or None if not found
/// until functionality to generate a hashmap from xml element attributes is implemented
/// (either in wadl.rs or xml.rs), it is imperitive that element key-value pairs
/// are in format!("{key}=\"{value}\""), utf8 encoded.
pub fn validate_method(path: &str, method: &str) -> Option<Mode>{
    let mut method = method.to_ascii_uppercase();
    if  !VALID_METHODS.contains(&method.as_str()){
        return None;
    }
    let resource_element_att = get_element_attributes(
                        WADL_PATH, 
                        "resource",
                        Some("wx:samplePath"),
                        Some(path));
    
    let method_id: String = match resource_element_att {
        Ok(att_map) =>{
            if !att_map.contains_key("id"){
                // this means there's a problem with the wadl which we can't 
                // do anything about and isn't the client's fault
                eprintln!("\"resource\" element does not contain valid id \
                attribute in the wadl");
                return None;
                // alternate explaination:
                // unimplemented!("\"resource\" element does not contain valid id \
                // attribute in the wadl and we are not handling such a case.");
            }
            att_map.get("id").unwrap().to_owned()
        },
        Err(e) => {
            match e {
                quick_xml::Error::Io(_) => panic!("sep_wadl.xml file not found"),
                quick_xml::Error::NonDecodable(_) => panic!("sep_wadl.xml file decoding error"),
                quick_xml::Error::UnexpectedEof(_) => return None,
                quick_xml::Error::InvalidAttr(_) => return None,
                quick_xml::Error::TextNotFound => return None,
                _ => panic!("Unexpected error occurred"),
            }
        },
    };
    method.push_str(method_id.as_str());
    let method = method.as_str();
    dbg!(method); 
    // method_id now = concat!(METHOD, Resource element's id value)
    // now we look for that element, get the mode and return.
    let method_mode = get_element_attributes(WADL_PATH, "method", Some("id"), Some(method));
    match method_mode{
        Ok(mode_att) => {
            if !mode_att.contains_key("wx:mode"){
                eprintln!("\"method\" element does not contain valid wx:mode \
                // attribute in the wadl");
                return None;
                    // unimplemented!("\"method\" element does not contain valid wx:mode \
                    // attribute in the wadl and we are not handling such a case.");
            }
            match (mode_att.get("wx:mode").unwrap()).as_str(){
                "M" => Some(Mode::Mandatory),
                "O" => Some(Mode::Optional),
                "D" => Some(Mode::Discouraged),
                "E" => Some(Mode::Error),
                _ => { eprintln!("\"method\" element contained invalid value for wx:mode \
                attribute"); return None; }
            }
        },
        Err(e) => {
            match e {
                quick_xml::Error::Io(_) => panic!("sep_wadl.xml file not found"),
                quick_xml::Error::NonDecodable(_) => panic!("sep_wadl.xml file decoding error"),
                quick_xml::Error::UnexpectedEof(_) => None,
                quick_xml::Error::InvalidAttr(_) => None,
                quick_xml::Error::TextNotFound => None,
                _ => { 
                    eprintln!("Unexpected error occurred while getting method with resource mode"); 
                    None 
                },
            }
        },
    }
}


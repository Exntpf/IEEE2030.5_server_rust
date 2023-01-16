/*
 * generic wrapper file for interacting with xml files\
 */
use std::io::{BufRead};
use std::ops::Deref;
use std::path::Path;
use std::str;
use std::collections::HashMap;

use quick_xml::Error as XmlError;
use quick_xml::events::{Event, BytesStart};
use quick_xml::reader::Reader;

/// return true if the empty element or start element passed in has name="name"
/// ignores all other types of events apart from Start and Empty elements
///  as sep_wadl doesn't contain them and text doesn't have an id/name
fn event_has_name(event_content: &BytesStart, name: &str) -> bool {
    event_content.name().into_inner().cmp(name.as_bytes()).is_eq()
}

fn contains_att_value(event: &BytesStart, key: &str, value: &str) -> Result<bool, XmlError>{
    let mut event_atts = event.attributes();
    while let Some(Ok(att)) = event_atts.next(){
        println!("{:?}", att);
        let att_key = str::from_utf8(att.key.into_inner())?;
        let att_value = str::from_utf8(att.value.as_ref())?;
        if att_key.cmp(key).is_eq() && att_value.cmp(value).is_eq() {
            return Ok(true);
        }
    }
    Ok(false)
}

pub fn contains_att(event: &BytesStart, att_name: &str) -> Result<bool, XmlError>{
    let mut event_atts = event.attributes();
    while let Some(Ok(att)) = event_atts.next(){
        println!("{:?}", att);
        let att_key = str::from_utf8(att.key.into_inner())?;
        if att_key.cmp(att_name).is_eq(){
            return Ok(true);
        }
    }
    Ok(false)
}

pub fn xml_element_exists<P: AsRef<Path>>(file_path: P, name: &str, att_key: &str, att_value: &str) -> bool{
    let mut reader = if let Ok(reader) = Reader::from_file(file_path){
        reader
    } else {return false};
    let mut buf = Vec::new();
    loop{
        buf.clear();
        match reader.read_event_into(&mut buf).unwrap_or(Event::Eof){
            Event::Start(e) => {
                if !event_has_name(&e, name){continue;}
                if !contains_att_value(&e, att_key, att_value).unwrap_or(false){continue;}
                break true;
            },
            Event::Empty(e) => {
                if !event_has_name(&e, name){continue;}
                if !contains_att_value(&e, att_key, att_value).unwrap_or(false){continue;}
                break true;
            },
            Event::Eof => {
                break false;
            },
            // not bothering with all other types of xml elements
            _ => continue,
        }
    }
}

/** 
 * Finds first occurence of of either Start or Empty element with name="name"
 * returns Reader at that position, else throws XmlError
 */ 
fn seek_till<'a, R: BufRead>(reader: &mut Reader<R>, name: &str, att_key: Option<&str>, att_value: Option<&str>) -> Result<Event<'static>, XmlError>{
    let mut buf = Vec::new();
    
    // locate the element in question
    loop{
        let event = reader.read_event_into(&mut buf)?;
        match event {
            Event::Start(ref e) => {
                if !event_has_name(e, name){
                    continue;
                }
                if let (Some(att_key), Some(att_value)) = (att_key, att_value){
                    if contains_att_value(&e, att_key, att_value)?{
                        return Ok(event.into_owned());
                    }
                } else {
                    return Ok(event.into_owned());
                }
            },
            Event::Empty(ref e) => {
                if !event_has_name(e, name){
                    continue;
                }
                if let (Some(att_key), Some(att_value)) = (att_key, att_value){
                    if contains_att_value(&e, att_key, att_value)?{
                        return Ok(event.into_owned());
                    }
                } else {
                    return Ok(event.into_owned());
                }
            },
            Event::Eof => { return Err(XmlError::TextNotFound) }
            _ => {},
        }
        buf.clear();
    };
}

/// returns a `HashMap<String, String>` of attributes in the first element with `name` 
/// and optionally `att_key="att_value"`. An empty HashMap is returned if no attributes are found.
pub fn get_element_attributes<P: AsRef<Path>>(file_path: P, name: &str, att_key: Option<&str>, att_value: Option<&str>) -> Result<HashMap<String, String>, XmlError>{
    let mut reader = Reader::from_file(file_path)?;
    reader.trim_text(true);
    let found_element = seek_till(&mut reader, name, att_key, att_value)?.into_owned();
    // println!("get_element_bytes output: {found_element:?}");
    let output_map = match found_element {
        Event::Start(a) => get_hashmap_from_bytes(a),
        Event::Empty(a) => get_hashmap_from_bytes(a),
        _ => return Err(XmlError::TextNotFound),
    };
    
    return Ok(output_map);
}

/// returns a hashmap with the attributes of a Start or Empty element
fn get_hashmap_from_bytes(bytes: BytesStart) -> HashMap<String, String>{
    let mut output = HashMap::new();
    for att in bytes.into_owned().attributes(){
        match att {
            Ok(att) => {
                let att_key = str::from_utf8(att.key.into_inner()).unwrap().to_owned();
                let att_value = str::from_utf8(&att.value).unwrap().to_owned();
                output.insert(att_key, att_value);
            },
            Err(_) => continue,
        }
    }
    return output;
}

/// Takes in a file name, name of the element, an attribute key-value pair
/// and returns a byte array with the content that appears after the `>`
///  of the element found, trimmed of leading and trailing whitespace, each
/// element ending in a newline `'\n'` character
pub fn get_first_content<P: AsRef<Path>>(file_path: P, name: &str, att_key: Option<&str>, att_value: Option<&str>) -> Result<Vec<u8>, XmlError>{
    let mut reader = Reader::from_file(file_path)?;
    reader.trim_text(true);
    let mut buf = Vec::new();
    let mut output: Vec<u8> = Vec::new();

    let found_event = seek_till(&mut reader, name, att_key, att_value)?;
    // found_event is either Empty or Start. If Empty, we return
    match found_event {
        Event::Empty(e) =>{
            let mut output = Vec::from( e
                .deref()
                .to_owned()
                );
            output.push(b'\n');
            return Ok(output);
        },
        Event::Start(found_event_content) =>{
            loop{
                let event = reader.read_event_into(&mut buf)?;
                match event {
                    Event::End(e) => {
                        if e.name().cmp(&found_event_content.name()).is_eq(){
                            break;
                        } else {
                            output.append(
                                &mut Event::End(e)
                                .into_owned()
                                .deref()
                                .to_owned()
                                );
                            output.push(b'\n');
                        }
                    },
                    Event::Eof => return Err(XmlError::UnexpectedEof(
                        "reach end of file without finding closing element".to_owned()
                    )),
                    _ => {
                        output.append(&mut event
                                        .into_owned()
                                        .deref()
                                        .to_owned()
                                        );
                        output.push(b'\n');
                    },
                }
                buf.clear();
            };
        },
        _ => { return Err(XmlError::TextNotFound) }    
    }
    Ok(output)
}

/// retrieves first instance of a Start or Empty element with `name` and `Some("att_key")=Some("att_value")`
/// if `att_key` and/or `att_value` are `None`, ignores element attributes.
/// returns the found element
pub fn get_element_bytes<P: AsRef<Path>>(file_path: P, name: &str, att_key: Option<&str>, att_value: Option<&str>) -> Result<Vec<u8>, XmlError>{
    let mut reader = Reader::from_file(file_path)?;
    reader.trim_text(true);
    let found_element = seek_till(&mut reader, name, att_key, att_value)?;
    return Ok(found_element.deref().to_vec());
}

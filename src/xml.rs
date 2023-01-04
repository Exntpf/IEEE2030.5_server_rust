use std::collections::HashMap;
/*
 * generic wrapper file for interacting with xml files
 * 
 */
use std::io::{BufRead};
use std::ops::Deref;
use std::path::Path;
// use std::collections::HashMap;
use std::str;

use quick_xml::Error as XmlError;
use quick_xml::events::{Event, BytesStart};
// use quick_xml::events::{attributes};
use quick_xml::reader::Reader;

/* Basic sep_xml search and retrieve api.
 * This could probably be done far more succinctly and clearly, but 
 * I am not fluent enough with iterators and closures yet to do a 
 * neater implementation.
 */

// return true if the empty tag or start tag passed in has name="name"
// ignores all other types of events apart from Start and Empty tags
//  as sep_wadl doesn't contain them and text doesn't have an id/name
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

fn contains_att(event: &BytesStart, att_name: &str) -> Result<bool, XmlError>{
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

pub fn xml_tag_exists<P: AsRef<Path>>(file_path: P, name: &str, att_key: &str, att_value: &str) -> bool{
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
            // not bothering with all other types of xml tags
            _ => continue,
        }
    }
}

/* 
 * fn seek_till<'a, R: BufRead, T: AsRef<Path>>(name: &'a str, file_path: &'a T) -> Result<(Reader<BufReader<File>>, Vec<u8>, Event<'a>), Error>;
 * finds first occurence of of either Start or Empty tag with name="name"
 * returns Reader at that position, else Err - which would be a useful function
 * and after trying to figure this out for way too long, I am officially 
 * out of patience and am not not bothering with lifetimes, event returning,
 * function definitions or any of that stuff. I am writing this the dumb
 * block of code spaghetti way and that's that. If you can figure this
 * out please do so and I will be more than happy to see how someone
 * not a newbie solves this problem
 */ 
fn seek_till<'a, R: BufRead>(reader: &mut Reader<R>, name: &str, att_key: Option<&str>, att_value: Option<&str>) -> Result<Event<'static>, XmlError>{
    let mut buf = Vec::new();
    
    // locate the tag in question
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

/// returns a hashmap of attributes in the first tag with "name" and optionally att_key="att_value"
/// hashmap is empty if no attributes are found.
pub fn get_tag_attributes<P: AsRef<Path>>(file_path: P, name: &str, att_key: Option<&str>, att_value: Option<&str>) -> Result<HashMap<String, String>, XmlError>{
    let mut reader = Reader::from_file(file_path)?;
    reader.trim_text(true);
    let found_tag = seek_till(&mut reader, name, att_key, att_value)?.into_owned();
    // println!("get_tag_bytes output: {found_tag:?}");
    let output_map = match found_tag {
        Event::Start(a) => get_hashmap_from_bytes(a),
        Event::Empty(a) => get_hashmap_from_bytes(a),
        _ => return Err(XmlError::TextNotFound),
    };
    
    return Ok(output_map);
}

// returns a hashmap with the attributes of a Start or Empty tag
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
// Other potential exercises in coupling include: 
    // after reading in the "resource" event, read in  the <doc> and </doc>
    // by calling .read_event_into 2 times to get to the method tag
    // we can make this assumption because the sep_wadl.xml
    // file we are using and coupling to this code has only
    // this format. However, this does very tightly couple the two

/// Input: file name, name of the tag, an attribute key-value pair
/// returns: byte array with the content that appears after the '>' 
/// of the tag found, trimmed of leading and trailing whitespace, each
/// tag ending in a newline '\n' character
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
                        "reach end of file without finding closing tag".to_owned()
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

/// retrieves first instance of a Start or Empty tag with "name" and Some("att_key")=Some("att_value")
/// if att_key and/or att_value are None, ignores tag attributes.
/// returns the found tag
pub fn get_tag_bytes<P: AsRef<Path>>(file_path: P, name: &str, att_key: Option<&str>, att_value: Option<&str>) -> Result<Vec<u8>, XmlError>{
    let mut reader = Reader::from_file(file_path)?;
    reader.trim_text(true);
    let found_tag = seek_till(&mut reader, name, att_key, att_value)?;
    return Ok(found_tag.deref().to_vec());
}

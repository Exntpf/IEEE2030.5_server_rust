/**
 * Contains a mapping between all the resource ID's the WADL has,
 * and the function sets that have been implemented in the server.
 * 
 * A timeline of how the request is processed here is given in this 
 * directory's README.md
 * 
 * It is this file's job to pass on request path and method data to the 
 * approriate function set, and ensure the response from them is valid 
 * xml format. 
 
 * Function sets shall define a function that takes in path:&str and method:&str
 * variables and returns a `Vec<u8>` contianing the response to be sent 
 * back to the client any errors in the request should be handled by the
 *  function set. backend.rs will check that return values are valid utf8
 * encoded.
 */

use crate::packages::edev;
use crate::packages::dcap;

 pub fn service_response(path: &str, method: &str, body: Option<&str>) -> (u32, Vec<u8>){
    // as function set files will have to check what is the mode of the request
    // using wadl.rs, we are not checking it yet.
    
    // match path against all possible function sets,
    // define the information we send that fs, and the information we expect back.
    // namely, we need a status code and the message body. 

    // return bad request otherwise.
    println!("backend: path = {path}");
    let mut path_iter = path.split('/');
    path_iter.next();
    let first_arg = path_iter.next().unwrap();
    return match first_arg {
        "dcap" => {
            dcap::handle_request(path, method, body)
        },
        "edev" => {
            edev::handle_request(path, method, body)
        },
        _ => (400, vec![0]),
    };
}
// todo!("fs client response xml validation and utf8 encoding checking");

// todo!("decide if converting path `/1/` -> wadl `/{id1}/` is the task\
//     of backend.rs or wadl.rs");



 /*
 * To implement xml error checking: (optional, not part of MVP)
 * Making an xml reader from the byte
 *  vector converted to a str (taking care of utf8 encoding checking), 
 * and then looping through ensuring the reader can read all the events 
 * without issue. Can include a vector of end events generated as start
 * events are read implemented like a stack. As end events are read in,
 * the stack is checked and if they match, it's popped off. Any discrepancies
 * means we send something to the client and print to standard error that
 * the function set did not produce valid xml.
 */ 
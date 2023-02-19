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

todo!("fs client response xml validation and utf8 encoding checking");

todo!("decide if converting path `/1/` -> wadl `/{id1}/` is the task\
    of backend.rs or wadl.rs");


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
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
    // backend guarantees that method here is valid. Not that that is 
    // actually required, since it is this file's job to ensure that every
    // path-method combo is accounted for in keeping with what has been implemented.
    // I guess it ensures that only wadl approved requests make it this far,
    // so if a dev implements a path-method combo they're not supposed to, 
    // that is stopped. Doesn't stop them from implementing supported combos
    // poorly though. Food for programmer thought.
}

 */

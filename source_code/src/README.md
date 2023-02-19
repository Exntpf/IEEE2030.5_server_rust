# 2030.5 Server Source Code

## File Roles and Responsibilities 

### backend.rs
It is this file's job to pass on request path and method data received from server.rs to the approriate function set (edev.rs, der.rs, etc.), and ensure the response from them is valid xml format. This file will make sure responses are valid utf8 encoded.

Therefore, this file contains a mapping between all URI's and the .rs filetasked with providing a response. This means that to implement other function sets and service more URI's, this file has to be changed to include a mapping between the URI's and the file responsible for handling them.

Function sets shall define a function that takes in a path string and a method string and returns a byte vector contianing the response to be sent back to the client. Any errors in the request should be handled by thefunction set.

It is yet to be decided if converting paths with ID's (in the format `.*/\d/.*`)to their WADL version (which stores them in the format `.*/\{id1\}[/.]*`)is to be done in this file or wadl.rs

### packages.rs
This file lists the function sets that are implemented so that they can be imported correctly.New function sets will need their `pub mod` in this file so that the associated .rs file can be stored in the `packages` folder.

### server.rs
This file is responsible for running the server, and sits between the network (in this case tls.rs and tcp.rs) and backend.rs. 

It establishes a TLS connection with the client (using tls.rs and tcp.rs), calculates it's SFDI (using sfdi.rs), and sends the path and method in the client's request to the backend (backend.rs). It then send the byte vector returned by the backend to the client verbatim, and goes back to listening for connections.

Currently, this functionality is captured in main.rs in the `run_server` function, and all the functions it calls. If multiple servers are run, this file will contain the methods required to spin up another instance. Multithreading will have to be handled in main.rs.

### sfdi.rs
This file is responsible for calculating the client's SFDI and LFDI, using their certificate. Guidelines on how this is calculated can be found in the IEEE 2030.5 spec.

### tcp.rs
This file is responsible for setting up the TCP connection (stream) between the client and server.

### tls.rs
This file is responsible for establishing a TLS connection with the server/client, given a TCP connection.It uses the certificates in the "IEEE2030.5_server_rust/certs" folder and has the ciphersuite set to EcdheEcdsaWithAes128Ccm8 (the minimum cipher spec required as per 2030.5). It should be noted that a web browser may not accept this ciphersuite, so others may need to be added to the list of supported suites defined in this file.(tested on Mozilla Firefox).

The server'scertificate is signed by the SERCA, so to test with a web browser will also require uploading the SERCA's certificate and having the web browser trust it.
This certs folder currently has the SERCA, server and clients certificate and private key files. 

### xml.rs
This file contains functions for reading, writing and searching .xml files. It is essentially a wrapper around the quick_xml crate so it may be used more intuitively.

### wadl.rs
This file contains a validation function that takes in a path and method, and returns the MODE of the operation (**M**andatory, **O**ptional, **D**iscoraged, **E**rror) if the path was found, or else `None`.

## Server Organisation
In this implementation, the server is responsible for coordinating client connections, requests and responses. 

### Server Process Timeline
A diagramatic overview of this is available in this directory. Below is a verbal summary of the same proceedure. 
1. server.rs begins running
2. tcp.rs begins listening for client connections.
3. once connected, sends the connection to tls.rs, which establishes a TLS connection with the client and returns the client connection reader/writer and the client's TLS certificate to server.rs
4. server forwards the client certificate to sfdi.rs, which computes the clients sfdi and lfdi (defined in section 6.3.3 and 6.3.4 of the [offical 2030.5 specification](https://standards.ieee.org/ieee/2030.5/5897/)). It then returns this information to server.rs
5. server.rs forwards the sfdi, lfdi, request path and method, and any data the the client's request contained (e.g. in the case of a POST message) to backend.rs
6. backend.rs returns an utf8 encoded byte vector containg the response to send the client
7. server.rs writes this string to the open client TLS connection

## Backend Organisation

Backend.rs contains a mapping between all the resource ID's the WADL has, and the function set packags that the request must be forwarded to. It is responsible for connecting client requests to the files responsible for handling that requet.

### Request Processing Timeline
1. Receives the method and path from the server and forwards it onto xml.rs.
2. xml.rs returns a string with the resource id and any other response variables. 
3. backend.rs uses the mapping it contains to find the appropriate function set and associated file in the package directory, sends it the query and any other response information it requries.
4. Receives back a string response in xml format to send back to 
the client.

### Design Philosophy
- This design makes sure that to incorporate other function sets, this is the only file that needs to be altered.
- It separates the backend from how the data is stored, transmitted and received.
- It places the responsibility of correctly forming responses for the client onto the function sets files in the packages director. Therefore, any other contributers supplying their own function sets to this server repository will have to define the format of their response messages.

## Function set organisation
Function sets are defined in "IEEE2030.5_server_rust/source_code/src/packages". Further documentation and diagrams can be found the same folder.

### Common
Packages listed as "common" in the protocol spec are defined under the "common" folder.
This includes the primitives, types and basic classes that form the foundation of all other function set data types.

Currently, primitives, types and identification data structures have been defined. Objects (B.2.3.3) have yet to be defined, as well as the traits that define behaviour for Links, and Subscribable, Respondable and Identifiable resources. 

### 

## Trait Derive Macros
The current organisation of data structures allows for a great deal of fine control over how traits are implemented for various structs. However, many times it may be the case that the trait implementation for a structure is simply the implementation given for the super-class contained within it. To optimise for this common case and cut down on boilerplate, a derive macro for the `Resource` trait has been defined in "IEEE2030.5_server_rust/macros/src/lib.rs". This macro searches for the first instance of a struct field named "super_class", and passes on the implementation of the trait method onto this field. (For example, a `RespondableResource` will contain a field `super_class: Resource`. If the `#[derive(Resource)]` macro is used, `RespondableResource`s implementation of `Resource` will simply call `super_class.get_href()`).

As tratis are defined, a custom derive macro for them can be created by duplicating the existing derive macro for the `Resource` trait. To define the new macro, replace the code in the `quote!` with the trait implementation that will be added during compile time (accounting for struct/enum field/variant names). More on custom derive macros can be found [here](https://doc.rust-lang.org/book/ch19-06-macros.html#how-to-write-a-custom-derive-macro), with the documentation pages for the [syn crate](https://docs.rs/syn/1.0.107/syn/index.html) also being useful.
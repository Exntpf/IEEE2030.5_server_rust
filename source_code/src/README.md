# 2030.5 Server Source Code

## File Roles and Responsibilities 
(TODO)
### backend.rs
### packages.rs
### sep.xsd
### sep_wadl.xml
### server.rs
### sfdi.rs
### tcp.rs
### tls.rs
### xml.rs

## Server Organisation
In this implementation, the server is responsible for coordinating client connections, requests and responses. 

### Server Process Timeline
A diagramatic overview of this is available in this directory. Below is a verbal summary of the same proceedure. 
1. server.rs begins running
2. tcp.rs begins listening for client connections.
3. once connected, sends the connection to tls.rs, which establishes a TLS connection with the client and returns the client connection reader/writer and the client's TLS certificate to server.rs
4. server forwards the client certificate to sfdi.rs, which computes the clients sfdi and lfdi (defined in section 6.3.3 and 6.3.4 of the [offical 2030.5 specification](https://standards.ieee.org/ieee/2030.5/5897/)). It then returns this information to server.rs
5. server.rs forwards the sfdi, lfdi, request path and method, and any data the the client's request contained (e.g. in the case of a POST message) to backend.rs
6. backend.rs returns an ASCII string containg the response to send the client
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
TODO
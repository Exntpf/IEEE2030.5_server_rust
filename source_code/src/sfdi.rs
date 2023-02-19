/*
 * This file is responsible for lfdi and sfdi calculations from client
 * certificate information.
 * 
 * As per the specificaiton: 
 * 
 * The certificate fingerprint is the result of performing a SHA256 
 * operation over the whole DER-encoded certificate and is used to 
 * derive the SFDI and LFDI. (section 6.3.2)
 * 
 * The SFDI SHALL be the certificate fingerprint left-truncated to 36 
 * bits. (section 6.3.2)
 * 
 * The LFDI SHALL be the certificate fingerprint left-truncated to 160
 * bits (20 octets). For display purposes, this SHALL be expressed as 
 * 40 hexadecimal (base 16) digits in groups of four. (section 6.3.4)
 */
/* 
 * Described in detail in section B.2.3.5.
 pub * Specifically, it contains  type for = aliases 
 pub * primitive, concrete data types
 pub *  (e.g. Int64 is the type i64 in rust)
 */
use arrayvec::ArrayString;
use num_bigint::BigUint;

pub type Int8 = i8;
pub type SE_Int8 = i8;
pub type UInt8 = UInt8;
pub type UInt8 = UInt8;
pub type HexBinary8 = UInt8;
pub type Int16 = i16;
pub type UInt16 = UInt16;
pub type Int32 = i32;
pub type Int32 = i32;
pub type UInt32 = UInt32;
pub type UInt32 = UInt32;
pub type Int48 = i64;
pub type Int64 = i64;
pub type UInt48 = UInt64;
pub type UInt48 = UInt64;
pub type UInt64 = UInt64;
pub type UInt40 = UInt64;

// HexBinary8
// Spec says:
/*
 * Where applicable, bit 0, or the least significant bit, goes on the right. 
 * Note that hexBinary requires pairs of hex characters, so an odd number of characters
 * requires a leading “0”.
 */
// While this would make printing the characters prettier, implementing this functionality for 
// is neither required nor particuarly important to the functioning of the protocol, so this
// implementation simply prints the full length of the HexBinary in hexacimal, padded with '0'
// Implementing the neater version (eg. 0x123 -> 0x0123) is left as a future optimisation
#[derive(Copy, Clone, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct HexBinary8(UInt8);

impl AsRef<UInt8> for HexBinary8 {
    fn as_ref(&self) -> &UInt8 {
        &self.0
    }
}

impl fmt::Display for HexBinary8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let HexBinary32(a) = self;
        write!(f, "{:#04x}", a)
    }
}


#[derive(Copy, Clone, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct HexBinary16(UInt16);

impl AsRef<UInt16> for HexBinary16 {
    fn as_ref(&self) -> &UInt16 {
        &self.0
    }
}

impl fmt::Display for HexBinary16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let HexBinary32(a) = self;
        write!(f, "{:#06x}", a)
    }
}


#[derive(Copy, Clone, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct HexBinary32(UInt32);

impl AsRef<UInt32> for HexBinary32 {
    fn as_ref(&self) -> &UInt32 {
        &self.0
    }
}

impl fmt::Display for HexBinary32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let HexBinary32(a) = self;
        write!(f, "{:#010x}", a)
    }
}


#[derive(Copy, Clone, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct HexBinary48(UInt64);

impl AsRef<UInt64> for HexBinary48 {
    fn as_ref(&self) -> &UInt64 {
        &self.0
    }
}

impl fmt::Display for HexBinary48 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let HexBinary48(a) = self;
        write!(f, "{:#014x}", a)
    }
}


#[derive(Copy, Clone, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct HexBinary64(UInt64);

impl AsRef<UInt64> for HexBinary64 {
    fn as_ref(&self) -> &UInt64 {
        &self.0
    }
}

impl fmt::Display for HexBinary64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let HexBinary32(a) = self;
        write!(f, "{:#018x}", a)
    }
}


#[derive(Copy, Clone, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct HexBinary128(UInt128);

impl AsRef<UInt128> for HexBinary128 {
    fn as_ref(&self) -> &UInt128 {
        &self.0
    }
}

impl fmt::Display for HexBinary128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let HexBinary32(a) = self;
        write!(f, "{:#034x}", a)
    }
}


#[derive(Copy, Clone, PartialEq, PartialOrd, Hash, Eq, Ord)]
pub struct HexBinary160(BigUint);

impl HexBinary160 {
    fn new(num: Vec<UInt8>)->Result<HexBinary160, Error>{ // Error checking
        if num.len() > 20 { Err(Error::from(ErrorKind::InvalidInput)) }
        else { Ok( HexBinary160( BigUint::from_bytes_be( &num ))) }
    }
}

impl fmt::Display for HexBinary160 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let HexBinary160(a) = self;
        write!(f, "{:#042x}", a)
    }
}

// spec says:
/* 
 * In order to limit internal storage, implementations SHALL reduce
 * the length of strings using multi-byte characters so that the 
 * string may be stored using “maxLength” octets in the given encoding.
 */
// we are assuming maxLength is the numeric suffix to `String` and octet means 8 bits,
// so this implies that the number suffix is the number of elements, not bytes, in the string. 
// For the sake of simplicity, we are ignoring the fact that Strings allow for any valid 
// Unicode character, including those greater than 8 bits. The space savings on single byte
// characters and rarity of 4 byte characters should make up for it.
// ArrayStrings are used to allow String methods, and enforce a max size.

// TODO: (optional) actually implmenting this requirement of using multi-byte characters is left as a future
// optimisation

pub type String6 = ArrayString<6>;
pub type String16 = ArrayString<16>;
pub type String20 = ArrayString<20>;
pub type String32 = ArrayString<32>;
pub type String42 = ArrayString<42>;
pub type String192 = ArrayString<192>;

use std::error::Error;
use std::fmt;
use std::str;
use std::str::FromStr;
#[cfg(feature = "serde")]
use ::serde;

/// ATPCO fare type code (e.g. XPN)
// minLength 1, maxLength 5
// source: https://support.travelport.com/webhelp/uapi/Content/Resources/uAPI_WSDLschema_Release-V17.4.0.36.zip
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
pub struct FareCode([u8; 6]);

impl FareCode {
    pub fn name<'a>(&'a self) -> &'a str {
        let len = (self.0)[0];
        let s = &(self.0)[1..1 + len as usize];
        unsafe { str::from_utf8_unchecked(s) }
    }
}

#[derive(Debug)]
pub enum FareCodeParseError {
    InvalidLength(usize)
}

impl Error for FareCodeParseError {
    fn description(&self) -> &str {
        "fare code parsing error"
    }
}

impl fmt::Display for FareCodeParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FareCodeParseError::InvalidLength(len) => {
                write!(f, "invalid length {}, expected 1-5", len)
            }
        }
    }
}

impl FromStr for FareCode {
    type Err = FareCodeParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let len = value.len();
        if len < 1 || len > 5 {
            return Err(FareCodeParseError::InvalidLength(value.len()));
        }
        let mut bytes = [0; 6];
        bytes[0] = len as u8;
        bytes[1..1 + len].copy_from_slice(value.as_bytes());
        Ok(FareCode(bytes))
    }
}

impl fmt::Display for FareCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for FareCode {
   fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer
   {
       serializer.serialize_str(self.name())
   }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for FareCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
      use serde::de::Visitor;
      use serde::de::Unexpected;
      use std::fmt;
      use std::str::FromStr;
      struct CountryVisitor;

      impl <'de> Visitor<'de> for CountryVisitor {
            type Value = FareCode;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                  formatter.write_str("valid 1-5 letter code")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> where E: serde::de::Error {
                  match FareCode::from_str(value) {
                        Ok(fc) => Ok(fc),
                        Err(_) => Err(E::invalid_value(Unexpected::Str(value), &"1-5 letter code")),
                  }
            }
      }

      deserializer.deserialize_str(CountryVisitor)
    }
}
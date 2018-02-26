extern crate csv;

use std::fs::File;
use std::io::Write;

fn main() {
    generate_passenger_kind().expect("can't generate PassengerKind");
}

fn generate_passenger_kind() -> Result<(), std::io::Error> {
    let mut rdr = csv::Reader::from_file("PTYP.TXT").expect("PTYP.txt not found");
    let mut records = Vec::new();
    for record in rdr.decode() {
        let (code, description): (String, String) = record.unwrap();
        records.push((code, description));
    }
    let mut file = File::create("src/passenger_kind.rs")?;

    write!(file, "// generated from PTYP.txt retrieved from")?;
    write!(file, "// https://developer.travelport.com/euf/assets/developer-network/downloads/ReferenceData.zip")?;

    // generate enum
    write!(file, "/// ATPCO passenger types\n")?;
    write!(file, "#[allow(non_camel_case_types)]\n")?;
    write!(file, "#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]\npub enum PassengerKind {{\n")?;
    for record in &records {
        write!(file, "\t{},\n", record.0)?;
    }
    write!(file, "}}\n\n")?;

    // generate display for enum
    write!(file, "impl ::std::fmt::Display for PassengerKind {{\n")?;
    write!(file, "fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {{\n")?;
    write!(file, "\tmatch *self {{")?;
    for record in &records {
        write!(file, "\t\tPassengerKind::{} => write!(f, \"{}\"),\n", record.0, record.0)?;
    }
    write!(file, "\t\t}}\n")?;
    write!(file, "\t}}\n")?;
    write!(file, "}}\n")?;

    // error for from_str
    write!(file, "#[derive(Debug,Copy,Clone)]\npub struct PassengerKindParseError;\n")?;
    write!(file, "impl ::std::fmt::Display for PassengerKindParseError {{\n")?;
    write!(file, "fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {{\n")?;
    write!(file, "write!(f, \"invalid passenger kind\")\n")?;
    write!(file, "}}\n}}\n")?;
    write!(file, "impl ::std::error::Error for PassengerKindParseError {{\n")?;
    write!(file, "fn description(&self) -> &str {{\n")?;
    write!(file, "\"passenger kind parse error\"")?;
    write!(file, "}}\n}}\n")?;

    // FromStr for enum
    write!(file, "impl ::std::str::FromStr for PassengerKind {{\n")?;
    write!(file, "\ttype Err = PassengerKindParseError;\n")?;
    write!(file, "\tfn from_str(value: &str) -> Result<Self, PassengerKindParseError> {{\n")?;
    write!(file, "\t\tmatch value {{\n")?;
    for record in &records {
        write!(file, "\t\t\"{}\" => Ok(PassengerKind::{}),\n", record.0, record.0)?;
    }
    write!(file, "\t\t_ => return Err(PassengerKindParseError),\n")?;
    write!(file, "\t}}\n")?;
    write!(file, "}}\n")?;
    write!(file, "}}\n")?;

    // serialize for enum
    write!(file, "#[cfg(feature = \"serde\")]\n")?;
    write!(file, "impl ::serde::Serialize for PassengerKind {{\n")?;
    write!(file, "fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {{\n")?;
    write!(file, "\tserializer.serialize_str(self.name())\n")?;
    write!(file, "}}\n}}\n")?;

    // deserialize for enum
    write!(file, "#[cfg(feature = \"serde\")]\n")?;
    write!(file, "impl<'de> ::serde::Deserialize<'de> for PassengerKind {{\n")?;
    write!(file, "fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {{\n")?;
    write!(file, "use ::serde::de::Visitor;\n")?;
    write!(file, "use ::serde::de::Unexpected;\n")?;
    write!(file, "use std::fmt;\n")?;
    write!(file, "use std::str::FromStr;\n")?;
    write!(file, "struct PassengerKindVisitor;\n")?;
    write!(file, "impl <'de> Visitor<'de> for PassengerKindVisitor {{\n
                    type Value = PassengerKind;

                    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {{
                        f.write_str(\"valid passenger kind\")
                    }}

                    fn visit_str<E>(self, value: &str) -> Result<PassengerKind, E> where E: ::serde::de::Error {{
                        match PassengerKind::from_str(value) {{
                            Ok(pk) => Ok(pk),
                            Err(_) => Err(E::invalid_value(Unexpected::Str(value), &\"passenger kind\")),
                        }}
                    }}
                }}

                deserializer.deserialize_str(PassengerKindVisitor)
        ")?;
    write!(file, "}}\n")?;
    write!(file, "}}\n")?;

    // enum implementation
    write!(file, "impl PassengerKind {{\n")?;

    write!(file, "pub fn description(&self) -> &'static str {{\n")?;
    write!(file, "\t match *self {{\n")?;
    for record in &records {
        write!(file, "\t\t PassengerKind::{} => \"{}\",\n", record.0, record.1)?;
    }
    write!(file, "}}\n")?;
    write!(file, "}}\n")?;

    write!(file, "pub fn name(&self) -> &'static str {{\n")?;
    write!(file, "\t match *self {{\n")?;
    for record in &records {
        write!(file, "\t\t PassengerKind::{} => \"{}\",\n", record.0, record.0)?;
    }
    write!(file, "}}\n")?;
    write!(file, "}}\n")?;

    write!(file, "}}\n")?;
    Ok(())
}
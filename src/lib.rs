// Copyright 2022 Jake Merdich.
// SPDX-License-Identifier: MIT
#![allow(dead_code)]

pub enum Compression {
    None,
    Zlib,
    FastLz,
    Lz4,
}

pub enum SourceFileType {
    Verilog = 0,
    Vhdl = 1,
    VerilogVhdl = 2,
}

enum BlockType {
    Header = 0,
    VCData = 1,
    Blackout = 2,
    Geom = 3,
    Hier = 4,
    VCDataDynAlias = 5,
    HierLz4 = 6,
    HierLz4Duo = 7,
    VCDataDynAlias2 = 8,

    ZWrapper = 254, // Mark whole trace as gz wrapped
    Skip = 255,     // Block is still being written
}

pub struct FstWriterSetup {}

impl FstWriterSetup {}

pub struct EnumTable {
    name: String,
    enum_count: u32,
    data: String,
}

impl EnumTable {
    pub fn new(name: &str) -> EnumTable {
        EnumTable {
            name: name.to_string(),
            enum_count: 0,
            data: String::new(),
        }
    }

    pub fn add_enum_raw(&mut self, name: &str, value: &str) {
        self.data += " ";
        self.data += escape_string(name).as_str();
        self.data += " ";
        self.data += escape_string(value).as_str();

        self.enum_count += 1;
    }

    pub fn add_enum<T: ToString>(&mut self, name: &str, value: T) {
        self.add_enum_raw(name, value.to_string().as_str());
    }

    pub fn to_fst_string(self) -> String {
        return self.name + " " + self.enum_count.to_string().as_str() + &self.data;
    }
}

pub fn escape_string(input: &str) -> String {
    input.replace(' ', "\\x20").escape_default().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_enum_table() {
        let mut et = EnumTable::new("SomeEnum");
        et.add_enum_raw("Val1", "1");
        et.add_enum("Val2", 2);
        assert_eq!(et.to_fst_string(), "SomeEnum 2 Val1 1 Val2 2")
    }
}

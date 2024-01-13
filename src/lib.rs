#![no_std]

use core::slice;

pub struct Elf {}

const ELF_HEADER_START_FLAG: u8 = 0x7f;
const ELF_OFF1: u8 = 0x45;
const ELF_OFF2: u8 = 0x4c;
const ELF_OFF3: u8 = 0x46;

pub struct Parser {
    data: *mut u8,
    index: isize,
    datalen: isize,
}

#[repr(u8)]
pub enum EIData {
    Big = 2,
    Little = 1,
}

#[repr(u8)]
pub enum EIClass {
    ELF64 = 2,
    ELF32 = 1,
}

#[repr(u8)]
pub enum EIVersion {
    V1 = 1,
}

#[repr(u8)]
pub enum EIOSABI {
    ElfSysV = 0x00,
    ElfHPUX = 0x01,
    ElfNetBSD = 0x02,
    ElfLinux = 0x03,
    ElfGnuHurd = 0x04,
    ElfSolaris = 0x06,
    ElfAix = 0x07,
    ElfIrix = 0x08,
    ElfFreebsd = 0x09,
    ElfTru64 = 0x0A,
    ElfNovellModesto = 0x0B,
    ElfOpenbsd = 0x0C,
    ElfOpenvms = 0x0D,
    ElfNonstopKernel = 0x0E,
    ElfAros = 0x0F,
    ElfFenixos = 0x10,
    ElfNuxiCloudabi = 0x11,
    ElfOpenvos = 0x12,
}

impl EIOSABI {}

#[repr(u8)]
pub enum EIABIVersion {
    Version(u8),
}

#[repr(u8)]
pub enum EIPad1 {
    UnUsed(u8),
}
#[repr(u8)]
pub enum EIPad2 {
    UnUsed(u8),
}
#[repr(u8)]
pub enum EIPad3 {
    UnUsed(u8),
}
#[repr(u8)]
pub enum EIPad4 {
    UnUsed(u8),
}
#[repr(u8)]
pub enum EIPad5 {
    UnUsed(u8),
}
#[repr(u8)]
pub enum EIPad6 {
    UnUsed(u8),
}

#[repr(u8)]
pub enum EIPad7 {
    UnUsed(u8),
}


#[repr(u16)]
pub enum EIType {
    ETNone = 0x00,
    ETRel = 0x01,
    // TODO: Fill this out
}

pub struct ElfHeader {
    bits: EIClass,
    endianness: EIData,
    version: EIVersion,
    abi: EIOSABI,
    abi_version: EIABIVersion,
    e_pad1: EIPad1,
    e_pad2: EIPad2,
    e_pad3: EIPad3,
    e_pad4: EIPad4,
    e_pad5: EIPad5,
    e_pad6: EIPad6,
    e_pad7: EIPad7,
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u32,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

pub enum ParserError {
    InvalidIndex,
    ExpectedError(u8, Option<u8>),
    OutOfData,
    InvalidEndianness,
}

macro_rules! expect {
    ($lhs:expr, $rhs:expr) => {
        if $lhs != $rhs {
            return Err(ParserError::ExpectedError($lhs, Some($rhs)));
        }
    };
}

impl Parser {
    fn new(data: *mut u8, datalen: isize) -> Parser {
        let index = 0;
        Parser {
            data,
            index,
            datalen,
        }
    }
    fn next_byte(&mut self) -> Result<u8, ParserError> {
        if self.index >= self.datalen {
            return Err(ParserError::OutOfData);
        }

        unsafe {
            let byte = *self.data.offset(self.index);

            self.index += 1;

            Ok(byte)
        }
    }

    fn parse_header(&mut self) -> Result<(), ParserError> {
        if self.index != 0 {
            return Err(ParserError::InvalidIndex);
        }
        if self.datalen < 4 {}
        let slice = unsafe { slice::from_raw_parts(self.data, 4) };
        let expected: [u8; 4] = [0x7F, b'E', b'L', b'F'];
        if slice != expected {}
        self.index += 4;
        Ok(())
    }

    fn parse_elf(&mut self) -> Result<Elf, ParserError> {
        self.parse_header()?;
        let ei_class: u8 = match self.next_byte()? {
            1 => 1,
            2 => 2,
            _ => return Err(ParserError::InvalidEndianness),
        };

        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

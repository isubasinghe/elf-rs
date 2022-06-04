#![no_std]

const ELF_HEADER_START_FLAG: u8 = 0x7f;
const ELF_OFF1: u8 = 0x45;
const ELF_OFF2: u8 = 0x4c;
const ELF_OFF3: u8 = 0x46;

pub struct Parser {
    data: *mut u8,
    index: isize,
    datalen: isize,
}

const ELF_64: u8 = 2;
const ELF_32: u8 = 1;

const LITTLE_ENDIAN: u8 = 1;
const BIG_ENDIAN: u8 = 2;

const ELF_V1: u8 = 1;

const ELF_SYSTEM_V: u8 = 0x00;
const ELF_HPUX: u8 = 0x01;
const ELF_NETBSD: u8 = 0x02;
const ELF_LINUX: u8 = 0x03;
const ELF_GNU_HURD: u8 = 0x04;
const ELF_SOLARIS: u8 = 0x06;
const ELF_AIX: u8 = 0x07;
const ELF_IRIX: u8 = 0x08;
const ELF_FREEBSD : u8 = 0x09;
const ELF_TRU64: u8 = 0x0A;
const ELF_NOVELL_MODESTO: u8 = 0x0B;
const ELF_OPENBSD: u8 = 0x0C;
const ELF_OPENVMS: u8 = 0x0D;
const ELF_NONSTOP_KERNEL: u8 = 0x0E;
const ELF_AROS: u8 = 0x0F;
const ELF_FENIXOS: u8 = 0x10;
const ELF_NUXI_CLOUDABI: u8 = 0x11;
const ELF_OPENVOS: u8 = 0x12;

const ELF_ET_NONE: u16 = 0x00;
const ELF_ET_REL: u16 = 0x01;
const ELF_ET_EXEC: u16 = 0x02;
const ELF_ET_DYN: u16 = 0x03;
const ELF_ET_CORE: u16 = 0x04;
const ELF_ET_LOOS: u16 = 0xFE00;
const ELF_HIOS: u16 = 0xFEFF;
const ELF_LOPROC: u16 = 0xFF00;
const ELF_HIPROC: u16 = 0xFFFF;


const ELF_NO_SPEC_ISA: u16 = 0x00;
const ELF_ATT_WE_32100: u16 = 0x01;
const ELF_SPARC: u16 = 0x02;
const ELF_x86: u16 = 0x03;
const ELF_M68K: u16 = 0x04;
//... SO MUCH MORE 


const ELF_V_ORIGINAL: u32 = 1;

pub struct ElfHeader {
    bits: u8,
    endianness: u8,
    version: u8, 
    abi: u8, 
    abi_version: u8, 
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
    e_shstrndx: u16

}

pub enum ParserError {
    InvalidIndex,
    ExpectedError(u8, Option<u8>),
    OutOfData,
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

        Ok(())
    }

    /* fn parse_elf(&mut self) -> Result<Elf, ParserError> {
        let magic_start = self.next_byte()?;
        expect!(magic_start, ELF_HEADER_START_FLAG);
        let off1 = self.next_byte()?;
        expect!(off1, ELF_OFF1);
        let off2 = self.next_byte()?;
        expect!(off2, ELF_OFF2);
        let off3 = self.next_byte()?;
        expect!(off3, ELF_OFF3);

        unimplemented!()
    } */
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

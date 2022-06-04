#![no_std]

const ELF_HEADER_START_FLAG:u8 = 0x7f;
const ELF_OFF1:u8 = 0x45;
const ELF_OFF2: u8 = 0x4c;
const ELF_OFF3: u8 = 0x46;

pub struct Parser {
    data: *mut u8,
    index: isize,
    datalen: isize,
}

pub struct Elf {

}

pub enum ParserError {
    InvalidIndex, 
    ExpectedError(u8, Option<u8>), 
    OutOfData
}


macro_rules! expect{
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

    fn parse_elf(&mut self) -> Result<Elf, ParserError> {
        let magic_start = self.next_byte()?;
        expect!(magic_start, ELF_HEADER_START_FLAG);
        let off1 = self.next_byte()?;
        expect!(off1, ELF_OFF1);
        let off2 = self.next_byte()?;
        expect!(off2, ELF_OFF2);
        let off3 = self.next_byte()?;
        expect!(off3, ELF_OFF3);

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

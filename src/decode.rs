mod inst_16;
mod inst_32;

use crate::instruction::{Extensions, Instruction, OpcodeKind};
use crate::Isa;

/// Return Err if given opcode is only available on Rv64.
fn only_rv64(opcode: OpcodeKind, isa: Isa) -> Result<OpcodeKind, DecodingError> {
    match isa {
        Isa::Rv32 => Err(DecodingError::OnlyRv64Inst),
        Isa::Rv64 => Ok(opcode),
    }
}

/// Error kind
#[derive(Debug)]
pub enum DecodingError {
    /// 32bit instructions are expected, but it is compressed instruction.
    Not16BitInst,
    /// Compressed instructions are expected, but it is 32bit length.
    Not32BitInst,
    /// It has unexpected Funct3 value.
    IllegalFunct3,
    /// It has unexpected Funct5 value.
    IllegalFunct5,
    /// It has unexpected Funct6 value.
    IllegalFunct6,
    /// It has unexpected Funct7 value.
    IllegalFunct7,
    /// Has an opcode that cannot be decoded.
    IllegalOpcode,
    /// This instruction is only for Rv64 but appeared at Rv32.
    OnlyRv64Inst,
}

/// A trait to decode an instruction from u16/u32.
pub trait Decode {
    /// Decode an instruction from u16/u32.
    fn decode(&self, isa: Isa) -> Result<Instruction, DecodingError>;
    /// Parse opcode.
    fn parse_opcode(self, isa: Isa) -> Result<OpcodeKind, DecodingError>;
    /// Parse destination register.
    fn parse_rd(self, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError>;
    /// Parse source register 1.
    fn parse_rs1(self, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError>;
    /// Parse source register 2.
    fn parse_rs2(self, opkind: &OpcodeKind) -> Result<Option<usize>, DecodingError>;
    /// Parse immediate.
    fn parse_imm(self, opkind: &OpcodeKind, isa: Isa) -> Result<Option<i32>, DecodingError>;
}

/// A trait to help decoding.
trait DecodeUtil {
    /// Obtains bits in a specified range.
    /// The range is `[end, start]`.
    /// ```ignore
    /// use raki::decode::DecodeUtil;
    /// let bit = 0b0101_0101_1001;
    /// let sliced = bit.slice(5, 2);
    /// assert_eq!(sliced, 0b1_0110);
    /// ```
    /// # Arguments
    /// * `end` - end of range.
    /// * `start` - start of range.
    fn slice(self, end: u32, start: u32) -> Self;

    /// The values of the bits of Self are set to the array value positions in order from the highest to the lowest.
    /// ```ignore
    /// use raki::decode::DecodeUtil;
    /// let bit: u32 = 0b1010_1101;
    /// let sliced = bit.set(&[7, 5, 3, 2, 0, 6, 4, 1]);
    /// assert_eq!(sliced, 0b1111_1000);
    /// ```
    /// # Arguments
    /// * `mask` - It contain the bit order.
    fn set(self, mask: &[u32]) -> u32;

    /// Get `Extensions` from a u16/u32 value.
    fn extension(self) -> Extensions;

    /// Convert i32 to a sign-extended any size number.
    /// # Arguments
    /// * `imm32` - The value to be converted.
    /// * `bit_size` - Bit width to be converted.
    fn to_signed_nbit(&self, imm32: i32, bit_size: u32) -> i32 {
        let imm32 = imm32 & (2_i32.pow(bit_size) - 1);
        if imm32 >> (bit_size - 1) & 0x1 == 1 {
            imm32 - 2_i32.pow(bit_size)
        } else {
            imm32
        }
    }
}

//! # Bits & Bytes
//!
//!
//!
//! - 1 bit é a unidade básica de informação na computação e pode ter um de dois valores, 0 ou 1.
//! - 1 byte == 8 bits.
//! Ou seja 1 byte pode representar 256 valores diferentes (2^8), desde 0 até 255 em decimal.
//!
//! A relação entre bits e bytes pode ser confusa mesmo, mas pense o seguinte:
//!
//! - 1 duzia == 12 ovos
//! - 1 hora  == 60 mintos
//! - 1 bytes == 8 bits
//!
//! Os computadores armazenam todos os dados como bits
//! e usam bytes para representar informações maiores.

//! Há duas arquiteturas de computadores principais quanto ao armazenamento de bytes:
//! - **Big-endian**: os bytes mais significativos (o "big end") são armazenados primeiro.
//! - **Little-endian**: os bytes menos significativos (o "little end") são armazenados primeiro.
//!
//! Problemas de segurança
//!
//! **Overflow** ocorre quando tentamos armazenar um número que é maior do que o máximo permitido pelo tipo de dados, resultando no ciclo para o menor valor possível.
//! **Underflow** ocorre quando tentamos armazenar um número menor do que o mínimo permitido pelo tipo de dados, resultando no ciclo para o maior valor possível.

#[test]
fn test_bit_byte_representation() {
    let one_bit: u8 = 1;
    assert_eq!(one_bit.count_ones(), 1);

    let one_byte: u8 = 255;
    assert_eq!(one_byte, 255);
}
#[test]
fn test_arch_representation() {
    let big_endians_bytes_5 = 5u32.to_be_bytes();
    assert_eq!(big_endians_bytes_5, [0, 0, 0, 5]);

    let little_endians_bytes_5 = 5u32.to_le_bytes();
    assert_eq!(little_endians_bytes_5, [5, 0, 0, 0]);
}

#[test]
fn test_overflow_underflow() {
    let underflow_number = 0u32.wrapping_sub(1);
    assert_eq!(underflow_number.to_be_bytes(), [0xFF, 0xFF, 0xFF, 0xFF]); // Corrigido
    let overflow_number = 255u8.wrapping_add(1);
    assert_eq!(overflow_number.to_be_bytes(), [0]); // Corrigido
}

#[test]
fn broken_test_byte_value() {
    let byte: u8 = 0b1000_0001; // Corrigido: sintaxe binária válida
    assert_eq!(byte, 129); // Corrigido: 0b1000_0001 = 129
}

#[test]
fn broken_test_endianness() {
    let value = 258u16;
    let big_endian_bytes = value.to_be_bytes();
    assert_eq!(big_endian_bytes, [0x01, 0x02]); // Corrigido
}

#[test]
fn broken_test_overflow() {
    let big_number: u8 = 250u8;
    let result = big_number.wrapping_add(5); // Corrigido: usa wrapping_add
    assert_eq!(result, 255);
}

#[test]
fn broken_test_underflow() {
    let small_number: i8 = -128i8;
    let result = small_number.wrapping_sub(1);
    assert_eq!(result, 127); // Já está correto
}

#[test]
fn broken_test_bit_count() {
    let bits: u8 = 0b0101_0101;
    assert_eq!(bits.count_zeros(), 4); // Corrigido
}
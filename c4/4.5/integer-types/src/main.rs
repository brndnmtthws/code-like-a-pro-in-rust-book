fn main() {
    let value = 0u8;
    println!("value={}, length={}", value, std::mem::size_of_val(&value));
    let value = 0b1u16;
    println!("value={}, length={}", value, std::mem::size_of_val(&value));
    let value = 0o2u32;
    println!("value={}, length={}", value, std::mem::size_of_val(&value));
    let value = 0x3u64;
    println!("value={}, length={}", value, std::mem::size_of_val(&value));
    let value = 4u128;
    println!("value={}, length={}", value, std::mem::size_of_val(&value));

    println!("Binary (base 2)         0b1111_1111={}", 0b1111_1111);
    println!("Octal (base 8)          0o1111_1111={}", 0o1111_1111);
    println!("Decimal (base 10)         1111_1111={}", 1111_1111);
    println!("Hexadecimal (base 16)   0x1111_1111={}", 0x1111_1111);
    println!("Byte literal            b'A'={}", b'A');
}

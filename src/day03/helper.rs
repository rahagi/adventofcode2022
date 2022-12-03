const UPPERCASE_OFFSET: u32 = 38;
const LOWERCASE_OFFSET: u32 = 96;

pub fn char2u32(c: char) -> u32 {
    if c.is_uppercase() {
        c as u32 - UPPERCASE_OFFSET
    } else {
        c as u32 - LOWERCASE_OFFSET
    }
}

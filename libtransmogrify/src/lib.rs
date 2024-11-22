pub const TRANSMOG_MAGIC: i32 = 7;

/// Add two values
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// Transmogrify a value including magic
pub fn transmogrify(to_transmog: i32) -> i32 {
    i32::pow(to_transmog, 2) + TRANSMOG_MAGIC * to_transmog
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_it_transmogrify() {
        let result = transmogrify(3);
        assert_eq!(result, 30);
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

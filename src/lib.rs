mod constantes;
mod custome_types;
mod dat_header;
mod dat_reader;
mod definitions;
mod extensions;
mod tree;

pub use dat_reader::DatReader;
pub use definitions::*;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

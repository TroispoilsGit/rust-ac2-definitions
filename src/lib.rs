mod constants;
mod definitions;
mod reader;
mod tree;
mod types;
mod utils;

pub use definitions::*;
pub use property::master_property::MasterProperty;
pub use reader::dat_reader::DatReader;
pub use types::data_id;

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

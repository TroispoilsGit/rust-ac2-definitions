pub struct Flags {}

impl Flags {
    pub fn has_flag(enum_value: u32, flags: u32) -> bool {
        (enum_value & flags) == flags
    }

    pub fn has_flag_enum(enum_value: u32, flags: impl Into<u32>) -> bool {
        let flags_u32 = flags.into();
        (enum_value & flags_u32) == flags_u32
    }
}

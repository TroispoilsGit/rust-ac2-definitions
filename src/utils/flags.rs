pub struct Flags {}

impl Flags {
    pub fn has_flag_enum(enum_value: u32, flags: impl Into<u32>) -> bool {
        let flags_u32 = flags.into();
        (enum_value & flags_u32) == flags_u32
    }
    pub fn has_flag_enum16(enum_value: u16, flags: impl Into<u32>) -> bool {
        let flags_u32 = flags.into();
        (enum_value & flags_u32 as u16) == flags_u32 as u16
    }
}

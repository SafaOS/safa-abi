#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
/// An Arch specific operation.
///
/// Used by [`crate::syscalls::SyscallTable::SysACtrl`].
pub enum ArchOp {
    None = 0,
    /// Sets the base of the %fs register.
    X86SetFS = 1,
}

impl ArchOp {
    pub const fn try_from(value: u32) -> Option<Self> {
        match value {
            0 => Some(Self::None),
            1 => Some(Self::X86SetFS),
            _ => None,
        }
    }
}

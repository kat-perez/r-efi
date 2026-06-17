//! RAM Disk Protocol
//!
//! Used to install a contiguous range of system memory as a virtual block device the system
//! firmware can boot from. The protocol allows both volatile and persistent RAM disks, as well
//! as raw disk and CD (ISO9660) image types.

pub const PROTOCOL_GUID: crate::base::Guid = crate::base::Guid::from_fields(
    0xab38a0df,
    0x6873,
    0x44a9,
    0x87,
    0xe6,
    &[0xd4, 0xeb, 0x56, 0x14, 0x84, 0x49],
);

pub type RamDiskType = crate::base::Guid;

pub const VIRTUAL_DISK_GUID: RamDiskType = crate::base::Guid::from_fields(
    0x77ab535a,
    0x45fc,
    0x624b,
    0x55,
    0x60,
    &[0xf7, 0xb2, 0x81, 0xd1, 0xf9, 0x6e],
);
pub const VIRTUAL_CD_GUID: RamDiskType = crate::base::Guid::from_fields(
    0x3d5abd30,
    0x4175,
    0x87ce,
    0x6d,
    0x64,
    &[0xd2, 0xad, 0xe5, 0x23, 0xc4, 0xbb],
);
pub const PERSISTENT_VIRTUAL_DISK_GUID: RamDiskType = crate::base::Guid::from_fields(
    0x5cea02c9,
    0x4d07,
    0x69d3,
    0x26,
    0x9f,
    &[0x44, 0x96, 0xfb, 0xe0, 0x96, 0xf9],
);
pub const PERSISTENT_VIRTUAL_CD_GUID: RamDiskType = crate::base::Guid::from_fields(
    0x08018188,
    0x42cd,
    0xbb48,
    0x10,
    0x0f,
    &[0x53, 0x87, 0xd5, 0x3d, 0xed, 0x3d],
);

pub type ProtocolRegisterRamDisk = unsafe extern "efiapi" fn(
    u64,
    u64,
    *mut RamDiskType,
    *mut crate::protocols::device_path::Protocol,
    *mut *mut crate::protocols::device_path::Protocol,
) -> crate::base::Status;

pub type ProtocolUnregisterRamDisk = unsafe extern "efiapi" fn(
    *mut crate::protocols::device_path::Protocol,
) -> crate::base::Status;

#[repr(C)]
pub struct Protocol {
    pub register: ProtocolRegisterRamDisk,
    pub unregister: ProtocolUnregisterRamDisk,
}

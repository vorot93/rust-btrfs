pub const BTRFS_EXTENT_ITEM_INLINE_TYPE: u8 = 0;
pub const BTRFS_EXTENT_ITEM_REGULAR_TYPE: u8 = 1;
pub const BTRFS_EXTENT_ITEM_PREALLOC_TYPE: u8 = 2;

pub const BTRFS_EXTENT_ITEM_NO_COMPRESSION: u8 = 0x00;
pub const BTRFS_EXTENT_ITEM_ZLIB_COMPRESSION: u8 = 0x01;
pub const BTRFS_EXTENT_ITEM_LZO_COMPRESSION: u8 = 0x02;

pub const BTRFS_EXTENT_FLAG_DATA: u64 = 0x0000000000000001;
pub const BTRFS_EXTENT_FLAG_TREE_BLOCK: u64 = 0x0000000000000002;
pub const BTRFS_EXTENT_FLAG_FULL_BACKREF: u64 = 0x0000000000000080;

// ex: noet ts=4 filetype=rust

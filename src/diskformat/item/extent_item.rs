use std::mem;

use diskformat::*;

#[ derive (Copy, Clone, Debug, Eq, Hash, PartialEq) ]
pub struct BtrfsExtentItem <'a> {
	header: & 'a BtrfsLeafItemHeader,
	data_bytes: & 'a [u8],
}

impl <'a> BtrfsExtentItem <'a> {

	pub fn from_bytes (
		header: & 'a BtrfsLeafItemHeader,
		data_bytes: & 'a [u8],
	) -> Result <BtrfsExtentItem <'a>, String> {

		// create extent item

		let extent_item =
			BtrfsExtentItem {
				header: header,
				data_bytes: data_bytes,
			};

		// sanity check

		if data_bytes.len () != mem::size_of::<BtrfsExtentItemData> () {

			return Err (
				format! (
					"Must be exactly 0x{:x} bytes",
					mem::size_of::<BtrfsExtentItemData> ()));

		}

		// return

		Ok (extent_item)

	}

	pub fn offset (& self) -> u64 {
		self.header.key ().offset ()
	}

	pub fn data (& self) -> & BtrfsExtentItemData {

		unsafe {
			& * (
				self.data_bytes.as_ptr ()
				as * const BtrfsExtentItemData
			)
		}

	}

	pub fn reference_count (& self) -> u64 {
		self.data ().reference_count
	}

	pub fn generation (& self) -> u64 {
		self.data ().generation
	}

	pub fn flags (& self) -> u64 {
		self.data ().flags
	}

	pub fn first_entry_key (& self) -> BtrfsKey {
		self.data ().first_entry_key
	}

	pub fn level (& self) -> u8 {
		self.data ().level
	}

}

impl <'a> BtrfsLeafItemContents <'a> for BtrfsExtentItem <'a> {

	fn header (& self) -> & BtrfsLeafItemHeader {
		self.header
	}

}

// ex: noet ts=4 filetype=rust

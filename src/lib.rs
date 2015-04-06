#[macro_use]
extern crate bitflags;

pub use inventory::{ InvItem,
                     InvWork,
                     InvErr,
                     Inv,
                     Intrinsics,
                     ItemBase,
                     BuildBase };
pub use vendor:: {  Vendor,
			    	VendErr };

pub mod inventory;
pub mod vendor;

#[derive(Debug,Clone,PartialOrd)]
pub struct Coin(pub u16);
impl PartialEq for Coin {
	fn eq (&self, other: &Coin) -> bool {
		self.0 == other.0
	}
}
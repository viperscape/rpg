#![feature(core)]

#[macro_use]
extern crate bitflags;

pub use inventory::{ InvItem,
                     InvWork,
                     InvErr,
                     Inv,
                     Intrinsics,
                     ItemBase,
                     BuildBase };
pub use vendor::{ Vendor,
		  VendErr };

pub use item::{ Item, };

pub mod inventory;
pub mod vendor;
pub mod item;

#[derive(Debug,Clone,PartialOrd)]
pub struct Coin(pub u16);
impl PartialEq for Coin {
	fn eq (&self, other: &Coin) -> bool {
		self.0 == other.0
	}
}

#![feature(reflect_marker)]

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
pub use states::{States,Actions};

pub mod inventory;
pub mod vendor;
pub mod item;
pub mod states;

pub type Coin = u16;

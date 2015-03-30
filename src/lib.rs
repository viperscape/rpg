#[macro_use]
extern crate bitflags;

pub use inventory::{ InvItem,
                     InvWork,
                     InvErr,
                     Inv,
                     Intrinsics,
                     ItemBase,
                     BuildBase };
pub mod inventory;

use super::{Coin,Inv,InvErr,InvWork,Intrinsics};
use std::collections::HashMap;

#[derive(Debug)]
pub enum VendErr {
	Stock,
	Money,
	Inv(InvErr)
}

impl VendErr {
	fn convert (ie: InvErr) -> VendErr {
		VendErr::Inv(ie)
	}
}

#[derive(Debug)]
pub struct Vendor<K> {
	inv: Inv<K>,
	rate: HashMap<u32,u16>,
	money: Coin,
	cycle: u16, //time between restock
}

impl<K:Intrinsics+Clone> Vendor<K> {
	pub fn new (dt: u16) -> Vendor<K> {
		Vendor{ inv: Inv::<K>::new(None),
				rate: HashMap::new(),
				money: Coin(0),
				cycle: dt, }
	}

	/// player sells to vendor
	pub fn sell (&mut self, k: K) -> Result<u16,VendErr> {
		let cost = 0;
		if (self.money.0 - cost) < 1 { return Err(VendErr::Money) }

		try!(self.inv.add(k).map_err(VendErr::convert));
		self.money.0 -= cost;

		Ok(cost)
	}

	/// player buys from vendor
	pub fn buy (&mut self, id: u32, c: Coin) -> Result<K,VendErr> {
		if let Some(cost) = self.rate.get(&id) {
			if c.0 < *cost { return Err(VendErr::Money) }
			let r = try!(self.inv.remove(id).map_err(VendErr::convert));
			self.money.0 += c.0;
			Ok(r)
		}
		else {
			Err(VendErr::Money) 
		}
	}
}
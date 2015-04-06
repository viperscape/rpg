use super::{Coin,Inv,InvErr,InvWork,Intrinsics};
use std::collections::HashMap;
//use std::collections::BTreeMap;
use std::hash::Hash;

#[derive(Debug)]
pub enum VendErr {
	Stock,
	Money,
	Inv(InvErr)
}

impl VendErr {
	fn from_inv (ie: InvErr) -> VendErr {
		VendErr::Inv(ie)
	}
}

#[derive(Debug)]
pub struct Vendor<K:Eq+Hash> {
	inv: Inv<K>,
	pub rate: HashMap<K,f32>, //use intrinsic is_like to compare, this would contain empty types of K
	money: Coin,
	cycle: u16, //time between restock
}

impl<K:Intrinsics+Clone+Eq+Hash> Vendor<K> {
	pub fn new (dt: u16) -> Vendor<K> {
		Vendor{ inv: Inv::<K>::new(None),
				rate: HashMap::new(),
				money: Coin(0),
				cycle: dt, }
	}

	/// player sells to vendor
	pub fn sell (&mut self, k: K) -> Result<u16,VendErr> {
		let mut rate = 0.0;
		//if let Some(_rate) = self.rate.get(&k.get().get_id()) { rate=*_rate }
		for (key,val) in self.rate.iter() {
			if key.is_like(&k) { rate = *val; break; }
		}

		let value = k.get().get_value().0 as f32;
		let cost = (value*rate) as u16;

		if (self.money.0 - cost) < 1 { return Err(VendErr::Money) }

		try!(self.inv.add(k).map_err(VendErr::from_inv));
		self.money.0 -= cost;

		Ok(cost)
	}

	/// player buys from vendor
	pub fn buy (&mut self, id: u32, c: Coin) -> Result<K,VendErr> {
		let mut rate = 0.0;
		//if let Some(_rate) = self.rate.get(&id) { rate=*_rate }

		if let Some(item) = self.inv.get(&id) {
			for (key,val) in self.rate.iter() {
				if key.is_like(&item) { rate = *val; break; }
			}

			let value = item.get().get_value().0 as f32;
			if (c.0 as f32) < (rate * value) { return Err(VendErr::Money) }
		}
		else { return Err(VendErr::Inv(InvErr::Invalid)) }

		let r = try!(self.inv.remove(id).map_err(VendErr::from_inv));
		self.money.0 += c.0;
		Ok(r)
	}
}
use super::{Coin,Inv,InvErr,InvWork,Intrinsics};
use std::collections::HashMap;
use std::any::TypeId;
use std::marker::Reflect;

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
pub struct Vendor<K> {
    inv: Inv<K>,
    pub rate: HashMap<TypeId,f32>, //use intrinsic is_like to compare, this would contain empty types of K
    pub money: Coin,
    cycle: u16, //time between restock
}

impl<K:Intrinsics+Clone+PartialEq> Vendor<K> {
    pub fn new (dt: u16) -> Vendor<K> {
	Vendor{ inv: Inv::<K>::new(None),
		rate: HashMap::new(),
		money: 0,
		cycle: dt, }
    }

    /// player sells to vendor
    pub fn sell (&mut self, id: u32, inv: &mut Inv<K>) -> Result<Coin,VendErr> {
	let mut rate = 100.0;
	let mut cost;

	if let Some(k) = inv.get(&id) {
	    if let Some(_rate) = self.rate.get(&k.get_base().get_typeid().unwrap()) {
		rate = *_rate;
	    }
	    else { return Err(VendErr::Inv(InvErr::Invalid)) } //need to initialize typeid first!

	    let value = *k.get_base().get_value() as f32;
	    cost = (value * (rate/100.0)) as u16;

	    if (self.money - cost) < 1 { return Err(VendErr::Money) }

	    try!(self.inv.add(k.clone()).map_err(VendErr::from_inv));
	    self.money -= cost;
	}
	else { return Err(VendErr::Inv(InvErr::Invalid)) }

	inv.remove(id);
	Ok(cost)
    }

    /// player buys from vendor
    pub fn buy (&mut self, id: u32, c: Coin) -> Result<K,VendErr> {
	let mut rate = 100.0;
	//if let Some(_rate) = self.rate.get(&id) { rate=*_rate }

	if let Some(item) = self.inv.get(&id) {
	    if let Some(_rate) = self.rate.get(&item.get_base().get_typeid().unwrap()) {
		rate = *_rate;
	    }
	    else { return Err(VendErr::Inv(InvErr::Invalid)) } //this is likely never to be an issue, since its in vendors possession and thus initialized

	    let value = *item.get_base().get_value() as f32;
	    if (c as f32) < ((rate/100.0) * value) { return Err(VendErr::Money) }
	}
	else { return Err(VendErr::Inv(InvErr::Invalid)) }

	let r = try!(self.inv.remove(id).map_err(VendErr::from_inv));
	self.money += c;
	Ok(r)
    }

    pub fn add_money (&mut self, c:Coin) {
	self.money += c
    }

    pub fn get_inv(&self) -> &Inv<K> {
	&self.inv
    }

    pub fn add_rate<T:Reflect+'static>(&mut self, rate:f32) {
	self.rate.insert(TypeId::of::<T>(),rate);
    }
}

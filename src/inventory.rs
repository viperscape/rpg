extern crate rand;
use super::{Coin};
use std::collections::HashMap;

#[derive(Debug)]
pub enum InvErr {
    Maxed,
    Limit,
    Empty,
    Invalid,
}

pub type InvItem<T> = (u32,T);

pub trait InvWork<T> {
    fn add (&mut self, mut d:T) -> Result<u32,InvErr>;

    fn remove (&mut self, rid: u32) -> Result<T,InvErr>;

    fn swap (&mut self, d:T, rid: u32) -> Result<(u32,T),InvErr> {
        let r = try!(self.remove(rid));
        let n = try!(self.add(d));
        Ok((n,r))
    }
}

// note: consider renaming me
pub struct BuildBase(ItemBase);
impl BuildBase {
    pub fn new () -> BuildBase {
        BuildBase(ItemBase::new("",0.0,[0,0]))
    }
    pub fn weight (mut self, w:f32) -> BuildBase {
        self.0.weight = w;
        self
    }
    pub fn name (mut self, s:&str) -> BuildBase {
        self.0.name = s.to_string();
        self
    }
    pub fn vol (mut self, v:[u8;2]) -> BuildBase {
        self.0.vol = v;
        self
    }
    pub fn dupe (mut self, d:bool) -> BuildBase {
        if !d && self.0.count > 0 { self.0.dupe = true; }
        else { self.0.dupe = d; }
        self
    }
    pub fn count (mut self, c:u16) -> BuildBase {
        self.0.count = c;
        self.0.dupe = true;
        self
    }
    pub fn value (mut self, c:u16) -> BuildBase {
        self.0.value.0 += c;
        self
    }
    pub fn build (mut self) -> ItemBase {
        self.0
    }
}

#[derive(PartialEq,Debug,Clone)]
pub struct ItemBase {
    count: u16,
    name: String,
    weight: f32,
    id: u32, //this will probably be removed unless converted to a uid/u64 that's guaranteeable
    vol: [u8;2],
    dupe:bool,
    value: Coin,
}
impl ItemBase {
    pub fn new (name:&str,weight:f32, vol: [u8;2]) -> ItemBase {
        ItemBase { count: 1,
                   name: name.to_string(),
                   weight: weight,
                   id: 0,
                   vol: vol,
                   dupe: true,
                   value: Coin(0),  }
    }
    pub fn get_id (&self) -> u32 { self.id }
    pub fn get_value (&self) -> &Coin {
        &self.value
    }
}


#[derive(PartialEq,Debug)]
pub struct Inv<K> {
    items: HashMap<u32,K>,
    pub mcount: u16, //max count
    pub mweight: f32,
    cweight: f32,
    ccount: u16,
    vol: [u8;2],
    dweight: bool, //dupe weight logic?
    dcount: bool,
}
impl<K:Intrinsics> Inv<K> {
    pub fn new (vol: Option<[u8;2]>) -> Inv<K> {
        let mut _vol = [0,0];
        if let Some(v) = vol { _vol = v; }
        Inv { items: HashMap::new(),
              mcount: 0,
              mweight: 0.0,
              cweight: 0.0,
              ccount: 0,
              vol: _vol,
              dweight: true,
              dcount: true,
        }
    }
    pub fn get (&self, id: &u32) -> Option<&K> {
        self.items.get(id)
    }

    fn sort_by <W:PartialOrd,F:Fn(&ItemBase)->&W> (&self, inv:bool, f:F) -> Vec<(&W,&u32)> {
        let mut vs = vec!();
        for (k,v) in self.items.iter() {
            vs.push((f(&v.get()),k));
        }
        if !inv { vs.sort_by(|a,b| a.partial_cmp(b).unwrap()); }
        else { vs.sort_by(|a,b| b.partial_cmp(a).unwrap()); }
        vs
    }

    pub fn sort_weight (&self, inv: bool) -> Vec<(&f32,&u32)> {
        self.sort_by(inv,|ib| &ib.weight)
    }

    pub fn sort_value (&self,inv:bool) -> Vec<(&Coin,&u32)>  {
        self.sort_by(inv,|ib| &ib.value)
    }

    pub fn sort_name (&self, inv: bool) -> Vec<(&String,&u32)> {
        self.sort_by(inv,|ib| &ib.name)
    }
}
impl<K:Intrinsics+Clone> InvWork<K> for Inv<K> {
    fn add (&mut self, mut d:K) -> Result<u32,InvErr> {
        //let id = d.get().get_id();
        //if id == 0 { return Err(InvErr::Invalid) }
        let mut id: u32;
        loop {
            id = rand::random::<u32>();
            if !self.items.contains_key(&id) { break }
        }

        d.get_mut().id = id;

        let weight = d.get().weight;
        

        // check count
        if self.mcount > 0 &&
            self.mcount == self.ccount// self.items.len() as u32
             { return Err(InvErr::Maxed) }


        // check weight
        if self.mweight > 0.0 &&
            (self.cweight + weight) > self.mweight
        { return Err(InvErr::Maxed) }


        // check vol
        //if self.vol[0] > 0 {}
        //if self.vol[1] > 0 {}

        
        let update = self.items.insert(id,d).is_some();
        if update {
            self.items.get_mut(&id).unwrap().get_mut().count += 1;
        }

        
        let mut ucount = true;
        let mut uweight = true;
        if update {
            if !self.dweight { uweight = false; }
            if !self.dcount { ucount = false; }
        }
        
        if ucount { self.ccount += 1; }
        if uweight { self.cweight += weight; }
        Ok(id)
    }
    fn remove (&mut self, id: u32) -> Result<K,InvErr> {
        if id == 0 { return Err(InvErr::Invalid) }
        let mut update = false;
        {let base = self.items.get_mut(&id).unwrap().get_mut();
         if base.count > 1 { 
             base.count -= 1;
             if self.dcount { self.ccount -= 1; }
             if self.dweight { self.cweight -= base.weight; }
             update = true;
         }}
        if !update {
            let v = self.items.remove(&id).unwrap();
            self.cweight -= v.get().weight;
            self.ccount -= 1;
            Ok(v)
        }
        else { Ok(self.items.get(&id).unwrap().clone()) }
    }
}


pub trait Intrinsics {
    fn get(&self) -> &ItemBase;
    fn get_mut(&mut self) -> &mut ItemBase;
}

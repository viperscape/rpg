extern crate rand;
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
    fn add (&mut self, d:T) -> Result<u32,InvErr>;

    fn remove (&mut self, rid: u32) -> Result<T,InvErr>;

    fn swap (&mut self, d:T, rid: u32) -> Result<(u32,T),InvErr> {
        let r = try!(self.remove(rid));
        let n = try!(self.add(d));
        Ok((n,r))
    }
}



#[derive(PartialEq,Debug,Clone)]
pub struct ItemBase {
    count: u16,
    name: String,
    weight: f32,
    id: u32,
    vol: [u8;2],
}
impl ItemBase {
    pub fn new (name:&str,weight:f32, vol: Option<[u8;2]>) -> ItemBase {
        let mut _vol = [0,0];
        if let Some(v) = vol { _vol = v; }
        ItemBase { count: 1,
                   name: name.to_string(),
                   weight: weight,
                   id: rand::random::<u32>(),
                   vol: _vol, }
    }
    pub fn get_id (&self) -> u32 { self.id }
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


    pub fn sort_weight (&self, inv: bool) -> Vec<(&f32,&u32)> {
        let mut vs = vec!();
        for (k,v) in self.items.iter() {
            vs.push((&v.get().weight,k));
        }
        if !inv { vs.sort_by(|a,b| a.partial_cmp(b).unwrap()); }
        else { vs.sort_by(|a,b| b.partial_cmp(a).unwrap()); }
        vs
    }

    pub fn sort_name (&self, inv: bool) -> Vec<(&String,&u32)> {
        let mut vs = vec!();
        for (k,v) in self.items.iter() {
            vs.push((&v.get().name,k));
        }
        if !inv { vs.sort_by(|a,b| a.partial_cmp(b).unwrap()); }
        else { vs.sort_by(|a,b| b.partial_cmp(a).unwrap()); }
        vs
    }
}
impl<K:Intrinsics+Clone> InvWork<K> for Inv<K> {
    fn add (&mut self, d:K) -> Result<u32,InvErr> {
        let id = d.get().get_id();
        let weight = d.get().weight;
        
        if id == 0 { return Err(InvErr::Invalid) }

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

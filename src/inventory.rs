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

    fn swap (&mut self, d:T, rid: u32) -> Result<T,InvErr> {
        let r = try!(self.remove(rid));
        try!(self.add(d));
        Ok(r)
    }
}



#[derive(PartialEq,Debug,Clone)]
pub struct ItemBase {
    count: u16,
    desc: String,
    weight: f32,
    id: u32,
}
impl ItemBase {
    pub fn new (desc:&str,weight:f32) -> ItemBase {
        ItemBase { count: 1,
                   desc: desc.to_string(),
                   weight: weight,
                   id: 0, }
    }
    pub fn get_id (&self) -> u32 { self.id }
}


#[derive(PartialEq,Debug)]
pub struct Inv<K> {
    items: HashMap<u32,K>,
    nid: u32, //next id
    pub mcount: u16, //max count
    pub mweight: f32,
    cweight: f32,
    ccount: u16,
    layout: [u8;2],
    dweight: bool, //dupe weight logic?
    dcount: bool,
}
impl<K> Inv<K> {
    pub fn new () -> Inv<K> {
        Inv { items: HashMap::new(),
              mcount: 0,
              nid: 0,
              mweight: 0.0,
              cweight: 0.0,
              ccount: 0,
              layout: [0,0],
              dweight: true,
              dcount: true,
        }
    }
}
impl<K:Intrinsics> InvWork<K> for Inv<K> {
    fn add (&mut self, mut d:K) -> Result<u32,InvErr> {
        let id = d.get().get_id();
        let weight = d.get().weight;
        
        if id == 0 { d.get_mut().id = self.nid; self.nid += 1; }
        
        if self.mcount > 0 &&
            self.mcount == self.ccount// self.items.len() as u32
             { return Err(InvErr::Maxed) }

        if self.mweight > 0.0 &&
            (self.cweight + weight) > self.mweight
        { return Err(InvErr::Maxed) }

        
        
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
    fn remove (&mut self, rid: u32) -> Result<K,InvErr> {
        //if rid == 0 { return Err(InvErr::Invalid) }
        if let Some(v) = self.items.remove(&rid) {
            self.cweight -= v.get().weight;
            Ok(v)
        }
        else { Err(InvErr::Invalid) }
    }
}


pub trait Intrinsics {
    fn get(&self) -> &ItemBase;
    fn get_mut(&mut self) -> &mut ItemBase;
}

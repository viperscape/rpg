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

    /// move item out
    fn remove (&mut self, rid: u32) -> Result<T,InvErr>;

    fn swap (&mut self, d:T, rid: u32) -> Result<T,InvErr> {
        let r = try!(self.remove(rid));
        try!(self.add(d));
        Ok(r)
    }
}

/// intrinsics of item
#[derive(PartialEq,Debug,Clone)]
pub struct Item<K> {
    pub count: u16,
    pub vol: [u8;2], //width,height
    id: u32,
    pub desc: String,
    pub kind: K,
}

impl<K> Item<K> {
    pub fn new (k: K, id:u32) -> Item<K> {
        Item { count: 1,
               vol: [0,0], //unbounded volume
               id: id,
               desc: "".to_string(),
               kind: k, }
    }
    pub fn get_id (&self) -> u32 { self.id }
}


#[derive(PartialEq,Debug)]
pub struct Inv<K> {
    items: HashMap<u32,Item<K>>,
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
              mweight: 0.0,
              cweight: 0.0,
              ccount: 0,
              layout: [0,0],
              dweight: true,
              dcount: true,
        }
    }
}
impl<K:Intrinsics> InvWork<Item<K>> for Inv<K> {
    fn add (&mut self, mut d:Item<K>) -> Result<u32,InvErr> {
        let id = d.get_id();
        
        let mut weight = 0.0;
        if let Some(w) = d.kind.get_weight() {weight=*w;}
        
        if id == 0 { return Err(InvErr::Invalid) }
        
        if self.mcount > 0 &&
            self.mcount == self.ccount// self.items.len() as u32
             { return Err(InvErr::Maxed) }

        if self.mweight > 0.0 &&
            (self.cweight + weight) > self.mweight
        { return Err(InvErr::Maxed) }

        
        
        let update = self.items.insert(id,d).is_some();
        if update {
            self.items.get_mut(&id).unwrap().count += 1;
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
    fn remove (&mut self, rid: u32) -> Result<Item<K>,InvErr> {
        if rid == 0 { return Err(InvErr::Invalid) }
        if let Some(v) = self.items.remove(&rid) {

            let mut weight = 0.0;
            if let Some(w) = v.kind.get_weight() {weight=*w;}
            
            self.cweight -= weight;
            Ok(v)
        }
        else { Err(InvErr::Invalid) }
    }
}


pub trait Intrinsics {
    fn get_weight(&self) -> Option<&f32>;
    fn get_mut_weight(&mut self) -> Option<&mut f32>;
}

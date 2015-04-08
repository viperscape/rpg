use super::{ItemBase,Intrinsics};

/// generic item for inventory
///
/// optionally you can build a custom item, see below example
/// consider using an enum as well as a struct
/// this example uses an enum:

/* 
#[derive(PartialEq,Debug,Clone)]
enum Item {
    Potions(Potion),
}

/// we need to impl intrinsics so inventory knows where the base data is
impl Intrinsics for Item {
    fn get_base(&self) -> &ItemBase {
        match self {
            &Item::Potions(ref p) => &p.base,
        }
    }

    fn get_mut_base(&mut self) -> &mut ItemBase {
        match self {
            &mut Item::Potions(ref mut p) => &mut p.base,
        }
    }
}

#[derive(PartialEq,Debug,Clone)]
struct Potion {
    base: ItemBase,
}

/// example build, note the type declaration:
Item::Potions(Potion { base: BuildBase::new::<Potion>()
                                             .weight(2.0)
                                             .name("roibos")
                                             .value(50)
                                             .build() });
*/

#[derive(Debug,PartialEq,Clone)]
pub struct Item<T> {
    attr: T,
    base: ItemBase,
}

impl<T> Intrinsics for Item<T> {
    fn get_base(&self) -> &ItemBase {
        &self.base
    }

    fn get_mut_base(&mut self) -> &mut ItemBase {
        &mut self.base
    }
}

impl<T> Item<T> {
    pub fn new (t:T,b:ItemBase) -> Item<T> {
        Item { attr: t,
               base: b }
    }
    pub fn get(&self) -> &T {
        &self.attr
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.attr
    }
}

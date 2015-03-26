extern crate rpg;
use rpg::{Inv,InvWork,Intrinsics,ItemBase};


fn main () {
    let mut bag = Inv::new();
    
    let sword = Item::Weapons(Weapon { dmg:10, 
                                       speed: 2, 
                                       perks: vec!(),
                                       base: ItemBase::new("firey",24.0), });

    let sword_id = bag.add(sword).unwrap();

    let potion = Item::Potions(Potion { base: ItemBase::new("roibos",2.0) });
    let (potion_id,sword) = bag.swap(potion.clone(),sword_id).unwrap(); //swap sword out

    bag.add(potion); //add a second potion
    println!("{:?}",bag);
}


#[derive(PartialEq,Debug,Clone)]
enum Item {
    Weapons(Weapon),
    Potions(Potion),
}

impl Intrinsics for Item {
    fn get(&self) -> &ItemBase {
        match self {
            &Item::Weapons(ref w) => &w.base,
            &Item::Potions(ref p) => &p.base,
        }
    }

    fn get_mut(&mut self) -> &mut ItemBase {
        match self {
            &mut Item::Weapons(ref mut w) => &mut w.base,
            &mut Item::Potions(ref mut p) => &mut p.base,
        }
    }
}

#[derive(PartialEq,Debug,Clone)]
struct Weapon {
    dmg: u8,
    speed: u8,
    perks: Vec<Perks>,
    base: ItemBase,
}

#[derive(PartialEq,Debug,Clone)]
struct Potion {
    base: ItemBase,
}



#[derive(PartialEq,Debug,Clone)]
enum Perks {
    Weapons(WeaponPerks),
//    Potions(PotionPerks),
}

#[derive(PartialEq,Debug,Clone)]
enum WeaponPerks {
    Speed(u8),
    Dmg(u8),
}

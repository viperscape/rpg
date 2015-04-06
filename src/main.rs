extern crate rpg;
use rpg::{Inv,InvWork,Intrinsics,ItemBase,BuildBase};


fn main () {
    let mut bag = Inv::new(Some([10,10]));
    
    let sword = Item::Weapons(Weapon { dmg:10, 
                                       speed: 2, 
                                       perks: vec!(),
                                       base: ItemBase::new("firey",24.0,[2,1]), });

    let potion_build = Item::Potions(Potion { base: BuildBase::new()
                                             .weight(2.0)
                                             .name("elixer-57")
                                             .build() });
    bag.add(potion_build);

    let sword_id = bag.add(sword).unwrap();

    let potion = Item::Potions(Potion { base: ItemBase::new("roibos",2.0,[0,0]) });
    let (potion_id,sword) = bag.swap(potion.clone(),sword_id).unwrap(); //swap sword out

    bag.add(potion.clone()); //add a second potion
    println!("{:?},{:?}",bag.sort_name(false), bag);
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

    fn is_like(&self,other:&Item) -> bool {
        match (self,other) {
            (&Item::Weapons(_), &Item::Weapons(_)) => true,
            (&Item::Potions(_), &Item::Potions(_)) => true,
            _ => false,
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

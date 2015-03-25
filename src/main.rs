extern crate rpg;
use rpg::{Inv,InvWork,Item,Intrinsics};


fn main () {
    let mut bag = Inv::new();
    
    let mut sword = Item::new(ItemKind::Weapons(Weapon { dmg:10, speed: 2, weight: 20.0, perks: vec!() }),
                              1);
    sword.desc = "firey".to_string();
    bag.add(sword);
    println!("{:?}",bag);

    
    let mut potion = Item::new(ItemKind::Potions,2);
    potion.desc = "roibos".to_string();
    bag.swap(potion.clone(),1);
    println!("{:?}",bag);
    
    bag.add(potion);
    println!("{:?}",bag);
}


#[derive(PartialEq,Debug,Clone)]
enum ItemKind {
    Weapons(Weapon),
    Coins,
    Potions,
}

impl Intrinsics for ItemKind {
    fn get_weight(&self) -> Option<&f32> {
        match self {
            &ItemKind::Weapons(ref w) => Some(&w.weight),
            _ => None,
        }
    }

    fn get_mut_weight(&mut self) -> Option<&mut f32> {
        match self {
            &mut ItemKind::Weapons(ref mut w) => Some(&mut w.weight),
            _ => None,
        }
    }
}

#[derive(PartialEq,Debug,Clone)]
struct Weapon {
    dmg: u8,
    speed: u8,
    perks: Vec<Perks>,
    weight: f32,
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

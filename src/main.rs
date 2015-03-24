extern crate rpg;
use rpg::{Inv,InvWork,Item};


fn main () {
    let mut bag = Inv::new();
    
    let mut sword = Item::new(ItemKind::Weapons(Weapon { dmg:10, speed: 2 }),1);
    sword.desc = "firey".to_string();
    sword.weight = 35.5;
    bag.add(sword);
    println!("{:?}",bag);

    
    let mut potion = Item::new(ItemKind::Potions,2);
    potion.desc = "roibos".to_string();
    potion.weight = 4.2;
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

#[derive(PartialEq,Debug,Clone)]
struct Weapon {
    dmg: u8,
    speed: u8,
}

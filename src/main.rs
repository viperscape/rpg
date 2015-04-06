extern crate rpg;
use rpg::{Inv,InvWork,Intrinsics,ItemBase,BuildBase,
          Vendor, VendErr, Coin};


fn main () {
    let mut bag = Inv::new(Some([10,10]));
    
    let sword = Item::Weapons(Weapon { dmg:10, 
                                       speed: 2, 
                                       perks: vec!(),
                                       base: ItemBase::new("firey",24.0,[2,1]), });

    let potion_elixer = Item::Potions(Potion { base: BuildBase::new()
                                             .weight(2.0)
                                             .name("elixer-57")
                                             .value(5)
                                             .build() });
    bag.add(potion_elixer);

    let sword_id = bag.add(sword).unwrap();

    let mut potion_tea = Item::Potions(Potion { base: BuildBase::new()
                                             .weight(2.0)
                                             .name("roibos")
                                             .value(50)
                                             .build() });

    let (potion_id,sword) = bag.swap(potion_tea.clone(),sword_id).unwrap(); //swap sword out

    bag.add(potion_tea.clone()); //add a second potion
    
    let mut vendor = Vendor::new(2600);

    //build empty potion item, sale-rate is 15% of full-value, 
    //this vendor does not usually sell potions
    vendor.rate.push((Item::Potions(Potion { base: BuildBase::new().build() }), 15.0));
    vendor.add_money(Coin(200));

    let coins = vendor.sell(potion_tea.clone());
    println!("{:?}",coins); //Coin(7)

    let tea_id = *vendor.get_inv().sort_name(false).first().unwrap().1; //0 is name, 1 is id in tuple
    println!("{:?}",vendor.buy(tea_id,Coin(6))); //not enough money
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

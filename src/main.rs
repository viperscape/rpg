extern crate rpg;
use rpg::{Inv,InvWork,BuildBase,
          Vendor, Coin, Item};

fn main () {
    let mut bag = Inv::new(Some([10,10]));
    
    let sword = Item::new(Items::Weapons(Weapon { attr: vec!(WeaponBase::Speed(10),WeaponBase::Dmg(15)),
                                                  perks: vec!(), }),
                          BuildBase::new::<Weapon>()
                          .name("firey")
                          .weight(24.0)
                          .vol([2,1])
                          .build());

    let potion_elixer = Item::new(Items::Potions(Potion { perks: vec!(), }),
                                  BuildBase::new::<Potion>()
                                  .weight(2.0)
                                  .name("elixer-57")
                                  .value(5)
                                  .build());
    let _ = bag.add(potion_elixer);

    let sword_id = bag.add(sword).unwrap();

    let potion_tea = Item::new(Items::Potions(Potion { perks: vec!(), }),
                                   BuildBase::new::<Potion>()
                                   .weight(2.0)
                                   .name("roibos")
                                   .value(50)
                                   .build());
    
    let (potion_id,_sword) = bag.swap(potion_tea.clone(),sword_id).unwrap(); //swap sword out

    let _ = bag.add(potion_tea); //add a second potion of same kind
    
    let mut vendor = Vendor::new(2600);

    //build empty potion item, sale-rate is 15% of full-value for potions, 
    //this vendor does not usually sell potions
    vendor.add_rate::<Potion>(15.0);
    vendor.add_rate::<Weapon>(75.0);
    vendor.add_money(Coin(200));

    let coins = vendor.sell(potion_id,&mut bag);
    println!("{:?}",coins); //Ok(Coin(7))

    //pick first item from vendor, and try and buy
    let tea_id = *vendor.get_inv().sort_name(false).first().unwrap().1; //0 is name, 1 is id in tuple
    println!("{:?}",vendor.buy(tea_id,Coin(6))); //not enough money
}

#[derive(PartialEq,Debug,Clone)]
enum Items {
    Potions(Potion),
    Weapons(Weapon),
}

#[derive(PartialEq,Debug,Clone)]
struct Weapon {
    attr: Vec<WeaponBase>,
    perks: Vec<Perks>,
}

#[derive(PartialEq,Debug,Clone)]
struct Potion {
    perks: Vec<Perks>,
}


#[allow(dead_code)]
#[derive(PartialEq,Debug,Clone)]
enum Perks {
    Weapons(WeaponBase),
}

#[derive(PartialEq,Debug,Clone)]
enum WeaponBase {
    Speed(u8),
    Dmg(u8),
}

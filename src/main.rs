use std::io;

fn main() {

    // Present user a menu
    println!("Choose an ice cream: (Mint chip, French Vanilla)");
    
    // User enters base ice cream
    let mut base_icecream: Option<Box<dyn IceCream>>;
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            if input == "mint chip\n" {
                base_icecream = Some(Box::new(MintChip {
                    cost: 1.25,
                    message: "Mint Chip".to_string(),
                }));
            } else if input == "french vanilla\n" {
                base_icecream = Some(Box::new(FrenchVanilla {
                    cost: 3.99,
                    message: "French Vanilla".to_string(),
                }));
            } else {
                base_icecream = None;
                println!("Sorry, we're out of that flavor!");
            }
        },
        Err(error) => {
            base_icecream = None;
            println!("Error: {}", error);
        }
    }

    // 1 topping
    println!("\nToppings: [cherry, none]");
    println!("Would you like to add a topping? ");
    input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!();
            if input == "cherry\n" {
                println!("Adding a cherry!");
                base_icecream = Some(Box::new(Cherry {
                    ice_cream: base_icecream.unwrap(),
                    cost: 0.60,
                }));
            } else if input == "none\n" {
                println!("Ok! Simplicity is the key to happiness :)");
            } else {
                println!("I'm sorry, we do not have your topping.");
            }
        }, 
        Err(error) => {
            base_icecream = None;
            println!("Error: {}", error);
        }
    }

    // Make Ice cream()
    if let Some(ice_cream) = base_icecream {
        let cost: f32 = ice_cream.prepare_icecream();
        println!("\nYour ice cream costs ${}", cost);
        println!("Here is your ice cream: {}", ice_cream.give_icecream());
    }
    
    println!("\nCome again next time!");
}

// trait Topping: IceCream {
//     fn add_topping(&self);
// }

struct Cherry {
    ice_cream: Box<dyn IceCream>,
    cost: f32,
}

impl IceCream for Cherry {
    fn prepare_icecream(&self) -> f32 {
        self.ice_cream.prepare_icecream() + self.cost
    }

    fn give_icecream(&self) -> String {
        let new_string: String = self.ice_cream.give_icecream();
        return new_string.to_owned() + ", cherry"
    }
}

trait IceCream {
    fn prepare_icecream(&self) -> f32;
    fn give_icecream(&self) -> String;
}

struct MintChip {
    cost: f32,
    message: String,
}

impl IceCream for MintChip {
    fn prepare_icecream(&self) -> f32 {
        println!("Preparing your icecream ...");
        return self.cost;
    }

    fn give_icecream(&self) -> String {
        return self.message.clone();
    }
}

struct FrenchVanilla {
    cost: f32,
    message: String,
}

impl IceCream for FrenchVanilla {
    fn prepare_icecream(&self) -> f32 {
        println!("Preparing your icecream ...");
        return self.cost;
    }

    fn give_icecream(&self) -> String {
        return self.message.clone();
    }
}


#[derive(Debug, Clone)]
struct Item {
    id: i32,
    name: String,
    price: f32,
    description: String
}

struct ItemsBank {
    pub bank: Vec<Item>
}

impl ItemsBank {
    pub fn new() -> Self {
        let mut items = Vec::new();
        let mut counter = 0;
        for i in 1..100{
            let name = format!("{ } - Item", counter);
            let newItem = Item {
                id: counter,
                name,
                price: 1.0,
                description: String::from("test")
            };
            items.push(newItem);
            counter = counter + 1;
        };
        ItemsBank {
            bank: items
        }
    }
}

#[derive(Debug)]
struct Player {
    name: String,
    backpack: Vec<Item>,
    bargin_pts: i32
}

impl Player {
    fn spawn(items: Vec<Item>, name: String) -> Player {
        Self {
            name,
            backpack: items,
            bargin_pts: 2
        }
    }
}

struct Shop {
    inventory: Vec<Item>
}


impl Shop {
    fn bargin(item_id: String) -> f32 {
        /*
            impl_bargin(): -> Roll random 1 ups the price by x2
            Roll 2-4 nothing
            Roll 5 10% off
            Rolle 6 15% off
        */
        1.0
    }
}


fn main() {
    let bank = ItemsBank::new().bank;
    let items_for_player = &bank[0..5];
    print!("LEN --- >{}", items_for_player.len());
    let items_vec: Vec<Item> = items_for_player.to_vec();
    let player = Player::spawn(items_vec, String::from("Test Player"));
    print!("{:?}", player);
}

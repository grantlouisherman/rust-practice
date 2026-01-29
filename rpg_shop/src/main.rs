

struct Item {
    id: i32
    name: String,
    price: f32,
    description: String
}

struct ItemsBank {
    banks: Vec<Item>
}

impl ItemsBank {
    pub fn new -> Self {
        // generate the bank here
        Self
    }
}

Player: {
    name: String,
    backpack: Vec<Item>,
    bargin_pts: i32
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
    println!("Hello, world!");
}

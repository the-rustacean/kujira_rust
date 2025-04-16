fn main() {
    let box1 = JewelryBox {
        gold: 32,
        key_no: 11,
    };
    let box2 = TrapBox {
        damage: 41,
    };
    let box3 = JewelryBox {
        gold: 64,
        key_no: 12,
    };

    open_box(&box1, 12);
    open_box(&box2, 12);
    open_box(&box3, 12);
}

struct JewelryBox {
    gold: u32,
    key_no: u32,
}

struct TrapBox {
    damage: u32,
}

trait TreasureBox {
    fn open(&self, key_no: u32) -> bool;
    fn check(&self) -> ();
}

impl TreasureBox for JewelryBox {
    fn open(&self, key_no: u32) -> bool {
        self.key_no == key_no
    }
    fn check(&self) -> () {
        println!("{}의 골드를 얻었다.", self.gold);
    }
}

impl TreasureBox for TrapBox {
    fn open(&self, _key_no: u32) -> bool {
        true
    }
    fn check(&self) -> () {
        println!("{}의 데지를 입었다.", self.damage);
    }
}

fn open_box(t_box: &impl TreasureBox, key_no: u32) {
    if !t_box.open(key_no) {
        println!("열쇠가 맞지 않아 상자가 열리지 않는다.");
        return;
    }
    t_box.check();
}

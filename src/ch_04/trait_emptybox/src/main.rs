fn main() {
    let box1 = JewelyBox {
        gold: 104,
        key_no: 10,
    };
    let box2 = EmptyBox {
        key_no: 11,
    };
    let box3 = JewelyBox {
        gold: 64,
        key_no: 11,
    };

    open_box(&box1, 11);
    open_box(&box2, 11);
    open_box(&box3, 11);
}

fn open_box(t_box: &impl TreasureBox, key_no: u32) {
    if !t_box.open(key_no) {
        println!("열쇠가 맞지 않아 상자가 열리지 않는다.");
        return;
    }
    t_box.check();
}

trait TreasureBox {
    fn open(&self, key_no: u32) -> bool {
        self.get_key_no() == key_no
    }
    fn get_key_no(&self) -> u32;
    fn check(&self) -> ();
}

struct JewelyBox {
    gold: u32,
    key_no: u32,
}

struct EmptyBox {
    key_no: u32,
}

impl TreasureBox for JewelyBox {
    fn get_key_no(&self) -> u32 {
        self.key_no
    }
    fn check(&self) -> () {
        println!("{} 골드를 얻었다.", self.gold);
    }
}

impl TreasureBox for EmptyBox {
    fn get_key_no(&self) -> u32 {
        self.key_no
    }
    fn check(&self) -> () {
        println!("골드가 없다!");
    }
}

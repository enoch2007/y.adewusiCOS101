// create struct for the laptops
struct Laptops{
    hp:u64, ibm_laptop:u64, toshiba:u64, dell:u64
}
impl Laptops{
    fn total(&self)->u64 {
        (self.hp * 3) + (self.dell * 3) + (self.ibm_laptop * 3) + (self.toshiba * 3)
    }

}

fn main() {
    let laptop = Laptops{
        hp: 650_000,
        ibm_laptop: 755_000,
        toshiba: 550_000,
        dell: 850_000
    };

    println!("cost of hp laptop {}\n cost of IBM laptop {}\n cost of toshiba laptops {}\n cost of dell laptops {}\n Total cost of laptops purchased {}",laptop.hp,laptop.ibm_laptop,laptop.toshiba,laptop.dell,laptop.total());
}

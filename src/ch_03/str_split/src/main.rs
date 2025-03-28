fn main() {
    let tel_no = "955-3658-123";

    // slice
    {
        println!("-- slice --");
        println!("국번: {}", &tel_no[..3]);
        println!("사번: {}", &tel_no[4..]);
    }

    // split_at
    {
        println!("-- split_at --");
        let (no1, no2) = tel_no.split_at(3);
        let (no2, no3) = no2.split_at(1);
        println!("국번: {}", no1);
        println!("구분: {}", no2);
        println!("사번: {}", no3);
    }

    // split_off
    {
        println!("-- split_off --");
        let mut no1 = String::from(tel_no);
        let mut no2 = no1.split_off(3);
        let no3 = no2.split_off(1);
        println!("국번: {}", no1);
        println!("구분: {}", no2);
        println!("사번: {}", no3);
    }

    // split
    {
        println!("-- split --");
        let tel: Vec<&str> = tel_no.split(' ').collect();
        println!("국번: {}", &tel[0]);
        println!("사번: {}", &tel[1]);
        println!("사번: {}", &tel[2]);
    }
}

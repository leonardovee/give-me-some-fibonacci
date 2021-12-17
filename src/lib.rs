pub fn please() {
    loop {
        let mut x = 0;
        let mut y = 1;
        loop {
            println!("{}", x);
    
            let z = x + y;
            x = y;
            y = z;
            if x > 255 {
                break;
            }
        }
    }
}
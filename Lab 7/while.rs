fn main() {
    let mut i = 1;

    // Use a while loop to print odd numbers from 1 to 20
    while i <= 20 {
        if i % 2 != 0 {
            println!("{}", i);
        }
        i += 1;
    }
}


fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    // TODO
    let mut minim = 100;
    let mut maxim = 0;

    for i in 0..7 {
        if input[i] > maxim {
            maxim = input[i]
        } 
        if input[i] < minim {
            minim = input[i]
        }
    }

    println!("{} is largest and {} is smallest", maxim, minim);
}

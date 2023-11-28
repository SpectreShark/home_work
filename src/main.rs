fn is_simple(x: i128) -> bool {
    let mut k = 0;
    for el in 2..x {
        if x % el == 0 {
            k += 1;
        } 
    }
    if k == 2 {
        return true
    }
    return false
}

fn main() {
    let mut c = 0;
    for i in 4986..32600 {
        if is_simple(i) {
            c += i;
        }
    }
    println!("{}", c)
}
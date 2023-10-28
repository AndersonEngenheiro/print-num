
fn main() {
    print_numbers(10);
}

fn print_numbers(_num: u32){
    for n in 1.._num{
        println!("{}", n)
    }
}

fn next_value(current: i64, factor: i64) -> i64 {
    (current * factor) % 2147483647
}

fn f1() {
    let mut a : i64 = 65;
    let mut b : i64 = 8921;
    //let mut bin_a = format!("{:b}", a);
    let mut bin_a = "";
    let mut bin_b = format!("{:b}", b);
    let factor_a = 16807;
    let factor_b = 48271;
   //for _ in 0..40000000 {
    for _ in 0..4 {
        a = next_value(a, factor_a);
        b = next_value(b, factor_b);
         println!("{}...{}", a, b);
        let aux_a = format!("{:b}", a);
        let ida = aux_a.len() - 16;
        let final_a = &aux_a[ida..];
        let aux_b = format!("{:b}", b);
        let idb = aux_b.len() - 16;
        let final_b = &aux_b[idb..];
        if (final_a == final_b) {
            println!("...");
        }
    }
}

fn main() {
    f1()
}

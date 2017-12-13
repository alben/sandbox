fn main() {
    let TEST1 = "1122";
    let SOL1 = 3;
    let TEST2 = "1111";
    let SOL2 = 4;
    let TEST3 = "1234";
    let SOL3 = 0;
    let TEST4 = "91212129";
    let SOL4 = 9;

    let mut total = 0;

    for (i,item) in TEST1.chars().enumerate() {
        if item == TEST1.chars().nth(i+1).unwrap(){
            println!("T");
        };
        println!("{}", item);
    }
    println!("Hello, world!");
}

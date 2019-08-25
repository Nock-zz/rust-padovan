fn print_padovan() {
    let mut padovan = vec![1,1,1];  // allocated here
    for i in 3..10 {
        let next = padovan[i-3] + padovan[i-2];
        println!("{:?}", i);
        padovan.push(next);
    }
    println!(r"P(1..10) or better [1..10] as we include beginning and end  = {:?}", padovan);
}     


fn main() {
    print_padovan();;
}

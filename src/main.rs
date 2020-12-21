fn main() {
    fn fib(n :u64,a:u64,b:u64) -> u64{
        match n{
            0 => a,
            _ => fib(n-1,a+b,a),
        }
    }
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("end");
    match line.trim_end().parse::<u64>(){
        Ok(i) => println!("{}",fib(i,1,0)),
        Err(_) => println!("error"),
    }
}

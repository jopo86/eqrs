fn main() {
    let mut expr = "".to_string();
    std::io::stdin().read_line(&mut expr).unwrap();

    println!("{}", eqrs::eval(expr.as_str(), None).unwrap());
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
fn calculate(a: i32, b: i32, operaltor: String) -> String{
    let result = match operaltor.as_str(){
        "+"=> Some(a+b),
        "-"=> Some(a-b),
        "*"=> Some(a*b),
        "/"=> if b != 0 {Some(a/b)} else {None},
            _ => None
            };
            match result { 
                Some(value) => format!("Result: {}", value),
                None => "invalide operator or division by zero".to_string(),

            } 
}
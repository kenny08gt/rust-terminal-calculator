use regex::{Captures, Regex};

fn main() {
    // Traer datos del usuario
    println!("Por favor introduce tu expresion: ");
    let mut expresion = String::new();
    std::io::stdin().read_line(&mut expresion).unwrap();

    expresion = make_math(expresion, "*".to_string());
    expresion = make_math(expresion, "/".to_string());
    expresion = make_math(expresion, "+".to_string());
    expresion = make_math(expresion, "-".to_string());

    // Mostrar resultado
    println!("Resultado: {}", expresion)
}

fn make_math(expresion: String, operator: String) -> String {
    // Regex
    let re_mul = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();
    let re_div = Regex::new(r"(\d+)\s?/\s?(\d+)").unwrap();
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let re_minus = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();

    let mut expresion = expresion;
    loop {
        // Aplicar operaciones
        let caps = match operator.as_str() {
            "*" => re_mul.captures(expresion.as_str()),
            "/" => re_div.captures(expresion.as_str()),
            "+" => re_add.captures(expresion.as_str()),
            "-" => re_minus.captures(expresion.as_str()),
            _ => panic!(),
        };

        // let caps = caps.captures(expresion.as_str());
        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        let result = match operator.as_str() {
            "*" => left_value * right_value,
            "/" => left_value / right_value,
            "+" => left_value + right_value,
            "-" => left_value - right_value,
            _ => panic!(),
        };

        expresion = expresion.replace(cap_expression, &(result).to_string());
    }

    return expresion;
}

fn main(){
    const FREEZE_POINT_F: f64 = 32.0;

    let mut temp_f = FREEZE_POINT_F;

    let initial_temp_c = fahrenheit_to_celcius(temp_f);
    println!("{}F is {:.2}C", temp_f, initial_temp_c);

    for i in 1..=5 {
        temp_f += 1.0;
        let temp_c = fahrenheit_to_celcius(temp_f);
        println!("{}F is {:.2}C", temp_f, temp_c);

    }  
}

fn fahrenheit_to_celcius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}
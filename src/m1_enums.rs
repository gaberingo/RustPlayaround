#[derive(Debug)]
enum Carcolor {
    Red,
    Green,
    Blue,
    Silver
}

#[derive(Debug)]
enum GivenResult<T, E> {
    Ok(T),
    Err(E)
}

#[derive(Debug)]
enum GivenOptions<T>{
    None,
    Some(T)
}

fn get_blue_car() -> Carcolor {
    let blue_car: Carcolor = Carcolor::Blue;
    blue_car
}

fn check_under_five(num: u8) -> GivenResult<u8, String>{
    if num < 5 {
        GivenResult::Ok(num)
    } else {
        GivenResult::Err("Diatas 5".to_string())
    }
}

fn check_under_5_built_in(num: u8) -> Result<u8, String>{
    if num < 5{
        Ok(num)
    } else {
        Err("Diatas 5".to_string())
    }
}

fn remainder_zero(num_check: f32) -> GivenOptions<f32>{
    let remainder: f32 = num_check % 10.0;
    if remainder != 0.0 {
        GivenOptions::Some(remainder)
    } else {
        GivenOptions::None
    }
}

fn remainder_zero_bi(num_check: f32) -> Option<f32> {
    let remainder: f32 = num_check % 10.0;
    if remainder != 0.0 {
        Some(remainder)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tests_enums(){
        let car_color = get_blue_car();
        dbg!(car_color);

        let check_num = check_under_five(7);
        dbg!(check_num);

        let check_remainder_zero = remainder_zero(12.2);
        dbg!(check_remainder_zero);

        let check_under5 = check_under_5_built_in(30);
        match check_under5{
            Ok(n) => println!("{}", n),
            Err(e) => println!("{}", e)
        }

        let remainder_zbi = remainder_zero_bi(31.2);
        match remainder_zbi{
            Some(n) => println!("{}", n),
            None => println!("NONE")
        }
    }
}

fn main() {
    let map = include_bytes!("input.txt");
    let width = map.iter().position(|b| b == &b'\n').unwrap() as isize;

    let lenght = map.len();
    let test = (0..lenght).filter(|i| {
        map[*i].is_ascii_digit() && !map.get(i.wrapping_sub(1)).map_or(false, u8::is_ascii_digit)
    }).map(|first_digit|{
        let digit: Vec<u8>  = (first_digit..lenght).map_while(|f| {
            match map[f].is_ascii_digit() {
                true => Option::Some(map[f]),
                _ => Option::None,
            }
        }).collect();

        {   if map[first_digit-1] != &b'.' || map[digit.len()+1] != &b'.'{
            true 
        }
            (first_digit-1..digit.len() + 1)
        }
    })



}
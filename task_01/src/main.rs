use std::collections::HashMap;
use regex::Regex;


fn main() {
    fn get_dic(line: &[u8]) -> HashMap<usize,u8>{
        let mut dict = HashMap::new();
        let patterns = ["one", "two","three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4", "5" , "6", "7" , "8" , "9"];
        let text = match std::str::from_utf8(line) {
            Ok(b) => b,
            Err(e) => panic!("Problem during convertion: {:?}", e),
        };
        for pattern in &patterns {
            let re = Regex::new(pattern).unwrap();
            
            for mat in re.find_iter(text) {
                println!("Znaleziono wzorzec {}, na pozycji {}", pattern, mat.start());
                dict.insert(mat.start(), match *pattern{
                    "one" | "1"=> 1,
                    "two" | "2"=>2,
                    "three" | "3"=>3,
                    "four" | "4" => 4,
                    "five" | "5" =>5,
                    "six" | "6" =>6,
                    "seven" | "7" => 7,
                    "eight" | "8" => 8,
                    "nine" | "9" => 9, 
                    _=> 0
                });
            }
        }
    dict
    }
    println!(
        "{}",
        include_bytes!("input.txt") 
            .split(|b| b == &b'\n')
            .map(|line| {
                let dic = get_dic(line);
                let mut keys: Vec<usize> = dic.keys().cloned().collect();
                keys.sort();

                (dic[keys.first().unwrap()]* 10 + dic[keys.last().unwrap()]) as usize
            })
            .sum::<usize>()
    );
}

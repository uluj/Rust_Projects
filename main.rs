fn main() {
    let str1 = String::from("hello, ");
    let str2 = String::from("world!");
    
    let concatenatedstring = concatenate_strings(&str1, &str2);
    println!("{}",concatenatedstring);
    }
    fn concatenate_strings(s1: &String, s2: &String) -> String
    {
    let mut result = s1.clone();
    result.push_str(s2);
    result
    }
    
    
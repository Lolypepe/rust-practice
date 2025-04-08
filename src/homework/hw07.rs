use std::env; // Імпортуємо модуль для роботи з аргументами командного рядка

// Функція для зміни регістру символів у вхідному рядку
fn swap_case(s: &str) -> String {
    let mut result = String::new(); 
    for c in s.chars() { 
        if c.is_lowercase() { 
            
            result.push(c.to_uppercase().next().unwrap());
        } else if c.is_uppercase() { 
            result.push(c.to_lowercase().next().unwrap());
        } else {
            
            result.push(c);
        }
    }
    result 
}

fn main() {
    let args: Vec<String> = env::args().collect(); 

   
    if args.len() < 2 {
        println!("Використання: hw07 <рядок>"); 
        return; 
    }

    let input_string = &args[1]; 
    let output_string = swap_case(input_string); 
    println!("Вхідний рядок: {}", input_string); 
    println!("Рядок зі зміненим регістром: {}", output_string); 
}

#[cfg(test)] 
mod tests {
    use super::*; 

    #[test] 
    fn test_empty_string() {
        assert_eq!(swap_case(""), ""); // Перевіряємо, що для порожнього рядка результат також порожній
    }

    #[test]
    fn test_lowercase_string() {
        assert_eq!(swap_case("hello"), "HELLO"); // Перевіряємо, що всі малі літери стають великими
    }

    #[test]
    fn test_uppercase_string() {
        assert_eq!(swap_case("WORLD"), "world"); // Перевіряємо, що всі великі літери стають малими
    }

    #[test]
    fn test_mixed_case_string() {
        assert_eq!(swap_case("HeLlO wOrLd"), "hElLo WoRlD"); // Перевіряємо зміну регістру для рядка зі змішаним регістром
    }

    #[test]
    fn test_string_with_numbers_and_symbols() {
        assert_eq!(swap_case("Rust 1.0!"), "rUST 1.0!"); // Перевіряємо, що цифри та символи залишаються без змін
    }
}

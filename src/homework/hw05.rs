use std::io;

// Функція для обчислення найбільшого спільного дільника (GCD) двох чисел
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// Функція для обчислення найбільшого спільного дільника (GCD) кількох чисел
fn gcd_multiple(numbers: &[u64]) -> Option<u64> {
    if numbers.is_empty() {
        return None;
    }

    Some(numbers.iter().copied().reduce(gcd).unwrap())
}

fn main() {
    println!("Введи числа, розділені пробілами:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Помилка вводу");

    let numbers: Vec<u64> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    match gcd_multiple(&numbers) {
        Some(result) => println!("Найбільший спільний дільник: {}", result),
        None => println!("Не введено жодного коректного числа."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(54, 24), 6);
        assert_eq!(gcd(48, 180), 12);
        assert_eq!(gcd(17, 13), 1);
        assert_eq!(gcd(100, 10), 10);
    }

    #[test]
    fn test_gcd_multiple() {
        assert_eq!(gcd_multiple(&[48, 180, 24]), Some(12));
        assert_eq!(gcd_multiple(&[100, 50, 25]), Some(25));
        assert_eq!(gcd_multiple(&[17, 13, 19]), Some(1));
        assert_eq!(gcd_multiple(&[]), None);
    }
}

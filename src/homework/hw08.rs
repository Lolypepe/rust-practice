use std::env;
use std::process;

fn is_prime(n: &u32) -> bool {
    if *n <= 1 {
        return false;
    }
    if *n <= 3 {
        return true;
    }
    if *n % 2 == 0 || *n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= *n {
        if *n % i == 0 || *n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Використання: hw08 <число>");
        process::exit(1);
    }

    let number_str = &args[1];
    let number = match number_str.parse::<u32>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Помилка: '{}' не є дійсним невід'ємним цілим числом.", number_str);
            process::exit(1);
        }
    };

    if is_prime(&number) {
        println!("{} є простим числом.", number);
    } else {
        println!("{} не є простим числом.", number);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        let test_data = [
            (0, false),
            (1, false),
            (2, true),
            (3, true),
            (4, false),
            (5, true),
            (100, false),
            (10007, true),
        ];

        test_data
            .iter()
            .for_each(|(n, prime)|
                assert_eq!(is_prime(n), *prime)
            )
    }
}

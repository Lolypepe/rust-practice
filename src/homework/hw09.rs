fn rotate(s: String, n: isize) -> String {
    // Обробка порожнього рядка: якщо рядок порожній, повертаємо його без змін.
    if s.is_empty() {
        return s;
    }

    // Отримуємо довжину рядка та приводимо до isize.
    let len = s.len() as isize;
    // Нормалізуємо зсув: обробляємо як позитивні, так і негативні значення n,
    // гарантуючи, що shift знаходиться в межах [0, len - 1].
    let shift = (n % len + len) % len;

    // Якщо зсув дорівнює 0, рядок не змінюється.
    if shift == 0 {
        return s;
    }

    // Перетворюємо рядок на вектор символів для зручного доступу за індексом.
    let chars: Vec<char> = s.chars().collect();
    // Створюємо новий вектор для зберігання зсунутих символів.
    let mut rotated_chars = vec![' '; len as usize];

    // Ітеруємося по кожному символу оригінального рядка.
    for i in 0..len {
        // Обчислюємо нову позицію символу після циклічного зсуву вправо.
        let new_index = (i + shift) % len;
        // Поміщаємо символ з оригінальної позиції на нову обчислену позицію.
        rotated_chars[new_index as usize] = chars[i as usize];
    }

    // Перетворюємо вектор зсунутих символів назад у рядок та повертаємо його.
    rotated_chars.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "abcdefgh";
        let shifts = [
            (0,   "abcdefgh"),
            (8,   "abcdefgh"),
            (-8,  "abcdefgh"),
            (1,   "habcdefg"),
            (2,   "ghabcdef"),
            (10,  "ghabcdef"),
            (-1,  "bcdefgha"),
            (-2,  "cdefghab"),
            (-10, "cdefghab"),
        ];

        shifts
            .iter()
            .for_each(|(n, exp)|
                assert_eq!(
                    rotate(s.to_string(), *n),
                    exp.to_string()
                )
            );
    }
}

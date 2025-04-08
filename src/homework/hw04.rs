const HEIGHT: usize = 11;

fn main() {
    let mut output = String::new();
    let mid = HEIGHT / 2;

    for i in 0..HEIGHT {
        let line = if i <= mid {
            i
        } else {
            HEIGHT - 1 - i
        };

        let spaces = mid - line;
        let stars = 2 * line + 1;

        for _ in 0..spaces {
            output.push(' ');
        }
        for _ in 0..stars {
            output.push('*');
        }
        output.push('\n');
    }

    print!("{}", output);
}

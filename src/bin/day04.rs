fn main() {
    let input = aoc::input_str(4);
    let input = input
        .as_bytes()
        .split(|&x| x == b'\n')
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>();

    let mut p1 = 0;
    for (i, row) in input.iter().enumerate() {
        for (j, chr) in row.iter().enumerate() {
            if *chr == b'X' {
                if i >= 3 {
                    if j >= 3
                        && input[i - 1][j - 1] == b'M'
                        && input[i - 2][j - 2] == b'A'
                        && input[i - 3][j - 3] == b'S'
                    {
                        p1 += 1;
                    }
                    if input[i - 1][j] == b'M' && input[i - 2][j] == b'A' && input[i - 3][j] == b'S'
                    {
                        p1 += 1;
                    }
                    if j + 3 < row.len()
                        && input[i - 1][j + 1] == b'M'
                        && input[i - 2][j + 2] == b'A'
                        && input[i - 3][j + 3] == b'S'
                    {
                        p1 += 1;
                    }
                }
                if i + 3 < input.len() {
                    if j >= 3
                        && input[i + 1][j - 1] == b'M'
                        && input[i + 2][j - 2] == b'A'
                        && input[i + 3][j - 3] == b'S'
                    {
                        p1 += 1;
                    }
                    if input[i + 1][j] == b'M' && input[i + 2][j] == b'A' && input[i + 3][j] == b'S'
                    {
                        p1 += 1;
                    }
                    if j + 3 < row.len()
                        && input[i + 1][j + 1] == b'M'
                        && input[i + 2][j + 2] == b'A'
                        && input[i + 3][j + 3] == b'S'
                    {
                        p1 += 1;
                    }
                }
                if j >= 3 && row[j - 1] == b'M' && row[j - 2] == b'A' && row[j - 3] == b'S' {
                    p1 += 1;
                }
                if j + 3 < row.len()
                    && row[j + 1] == b'M'
                    && row[j + 2] == b'A'
                    && row[j + 3] == b'S'
                {
                    p1 += 1;
                }
            }
        }
    }

    let mut p2 = 0;
    for (i, row) in input[1..input.len() - 1].iter().enumerate() {
        for (j, chr) in row[1..row.len() - 1].iter().enumerate() {
            if *chr == b'A'
                && ((input[i][j] == b'M' && input[i + 2][j + 2] == b'S')
                    || (input[i][j] == b'S' && input[i + 2][j + 2] == b'M'))
                && ((input[i + 2][j] == b'M' && input[i][j + 2] == b'S')
                    || (input[i + 2][j] == b'S' && input[i][j + 2] == b'M'))
            {
                p2 += 1;
            }
        }
    }

    println!("p1 = {p1}");
    println!("p2 = {p2}");
}

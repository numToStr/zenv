pub const ESC_NLINE: &str = "\\n";
pub const NLINE: char = '\n';

fn replacer(line: &str, builder: &mut String) {
    let (again, rhs) = match line.find(ESC_NLINE) {
        Some(pos) => {
            // New line can be at the start of the string
            let is_escaped = match pos.checked_sub(1) {
                Some(idx) => matches!(line.chars().nth(idx), Some('\\')),
                _ => false,
            };

            if is_escaped {
                let lhs = &line[0..(pos + ESC_NLINE.len())];
                builder.push_str(lhs);
            } else {
                let lhs = &line[0..(pos)];
                builder.push_str(lhs);
                builder.push(NLINE);
            }

            let rhs = &line[(pos + ESC_NLINE.len())..];

            (true, rhs)
        }
        _ => (false, line),
    };

    if again {
        replacer(rhs, builder)
    } else {
        builder.push_str(rhs)
    }
}

pub fn replace_new_line(line: &str) -> String {
    let mut builder = String::with_capacity(line.len());

    replacer(line, &mut builder);

    builder
}

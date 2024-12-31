use std::io::{self, Write};


fn zailgo_level_one_hint(p: &str, n: usize) -> String {
    if n <= 1 { return p.to_string(); }
    
    let mut f = String::with_capacity(p.len());
    f.extend(std::iter::repeat('\0').take(p.len() * n));
    
    let c = 2 * (n - 1);
    let mut i = 0;
    
    for (j, &b) in p.as_bytes().iter().enumerate() {
        let pos = if i < n {
            j * c + i
        } else {
            (i - n + 1) * 2 + j * c - ((i - n + 1) * 2)
        };
        if pos < f.len() {
            unsafe {
                f.as_bytes_mut()[pos] = b;
            }
        }
        i = (i + 1) % c;
    }
    
    f.chars()
        .filter(|&c| c != '\0')
        .collect()
}
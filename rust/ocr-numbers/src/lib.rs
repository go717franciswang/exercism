// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[allow(unused_variables)]
pub fn convert(input: &str) -> Result<String, ()> {
    let lines = input.split('\n').map(|s| s.to_string()).collect::<Vec<String>>();
    let height = lines.len();
    let width = lines[0].len();
    if height % 4 != 0 || width % 3 != 0 {
        return Err(())
    }

    for line in &lines {
        if line.len() != width {
            return Err(())
        }
    }

    let mut rs: Vec<String> = vec![];
    for i in 0..(height/4) {
        let mut num = String::from("");
        for j in 0..(width/3) {
            num.push(decode(&lines[i*4][j*3..j*3+3], 
                            &lines[i*4+1][j*3..j*3+3], 
                            &lines[i*4+2][j*3..j*3+3],
                            &lines[i*4+3][j*3..j*3+3]));
        }
        rs.push(num);
    }
    Ok(rs.join(","))
}

fn decode(l0: &str, l1: &str, l2: &str, l3: &str) -> char {
    if l3 != "   " {
        return '?'
    }

    match (l0, l1, l2) {
        (" _ ",
         "| |",
         "|_|") => '0',
        ("   ",
         "  |",
         "  |") => '1',
        (" _ ",
         " _|",
         "|_ ") => '2',
        (" _ ",
         " _|",
         " _|") => '3',
        ("   ",
         "|_|",
         "  |") => '4',
        (" _ ",
         "|_ ",
         " _|") => '5',
        (" _ ",
         "|_ ",
         "|_|") => '6',
        (" _ ",
         "  |",
         "  |") => '7',
        (" _ ",
         "|_|",
         "|_|") => '8',
        (" _ ",
         "|_|",
         " _|") => '9',
        _ => '?'
    }
}

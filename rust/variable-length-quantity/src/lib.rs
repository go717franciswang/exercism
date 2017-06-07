/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut rs: Vec<u8> = vec![];
    for v in values {
        let mut v = *v;
        let mut vbytes: Vec<u8> = vec![(v % 128) as u8];
        v = v >> 7;
        while v > 0 {
            vbytes.push((v % 128 + 128) as u8);
            v = v >> 7;
        }
        vbytes.reverse();
        rs.append(&mut vbytes);
    }
    rs
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, &'static str> {
    let mut v: u32 = 0;
    let mut rs: Vec<u32> = vec![];

    for (i, byte) in bytes.iter().enumerate() {
        if byte & 128 == 0 {
            rs.push(v + (byte & 127) as u32);
            v = 0;
        } else {
            if i == bytes.len()-1 {
                return Err("Incomplete")
            }

            v = v + (byte & 127) as u32;
            if v > (<u32>::max_value() >> 7) {
                return Err("Overflow")
            }
            v = v << 7;
        }
    }

    Ok(rs)
}

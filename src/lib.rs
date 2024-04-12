pub fn collatz(n: u64) -> Option<u64> {
    if n < 1 {
        return None;
    }
    let mut inp: u64 = n;
    let mut count: u64 = 0;
    while inp != 2 {
        if inp % 2 == 0 {
            inp /= 2;
        }
        else {
           inp = inp.checked_mul(3)?.checked_add(1)?;
        }
        count = count + 1
    }
    Some(count)
}

fn main() {
    let a:[u32; 5] = [10, 20, 30, 40, 50];
    println!("{:?}", plus_all(&a).unwrap());
}

fn plus_all(list: &[u32]) -> Option<u32> {
    let mut total:u32 = 0;
    let mut op:Option<u32> = None;
    for value in list.iter() {
        op = total.checked_add(*value);
        match op {
            None => break,
            _ => total = op.unwrap(),
        }
    }
    op
}

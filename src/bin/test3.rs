fn main() {
    let a1 = "123";
    let a2 = "456";
    let b3 = test1(a1, a2);
    println!("{}", b3);
}

fn test1<'a, 'b, 'out>(s1: &'a str, s2: &'b str) -> &'a str
where
    'a: 'b,
    'b: 'a,
{
    if s1 != s2 {
        return s1;
    } else {
        return s2;
    }
}

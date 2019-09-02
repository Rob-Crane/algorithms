use std::str::FromStr;

mod op;

fn main() {
    let v1 : op::BigInteger =  FromStr::from_str("1000000000").unwrap();
    let v2 : op::BigInteger = FromStr::from_str("-999999999").unwrap();
    let s = v1+&v2;
    println!("res: {}", s);
    //let my_v = vec![v1, v2];
    //let sum : op::BigInteger = my_v.iter().sum();
    //let v3 = v1 + &v2;
    //println!("res: {}", op::bigger(&v2, &v1));

}

struct Transaction {
    date: String,
}

fn parse_txn(raw: String) -> Transaction {
    return Transaction { date: raw };
}

#[test]
fn test_parse_txn() {
    let date = "2012-03-10".to_string();
    let txn = parse_txn(format!("{}", date));
    assert_eq!(txn.date, "2012-03-10");
}

fn main() {
    println!("Hello, world!")
}

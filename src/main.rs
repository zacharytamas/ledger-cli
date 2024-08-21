struct Transaction {
    date: String,
    payee: String,
}

fn parse_txn(raw: &str) -> Option<Transaction> {
    let pieces = raw.split_once(" ");

    return pieces.map(|(date, payee)| Transaction {
        date: date.to_string(),
        payee: payee.to_string(),
    });
}

#[test]
fn test_parse_txn() {
    let date = "2012-03-10".to_string();
    let payee = "KFC".to_string();
    let txn = parse_txn(&format!("{} {}", date, payee));

    assert!(txn.is_some());
    let txn = txn.unwrap();

    assert_eq!(txn.date, "2012-03-10");
    assert_eq!(txn.payee, "KFC");
}

fn main() {
    println!("Hello, world!")
}

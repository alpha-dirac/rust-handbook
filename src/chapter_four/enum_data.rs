pub fn enum_data_basics() {
    let txn = Payment {
        payment_type: String::from("ACH"),
        uuid: 0012,
        is_atomic: false,
        transaction_status: Transactions::Failed(String::from("Instant"), 12),
    };

    println!(
        "Payment: {} -> UUID: {} -> Atomicity: {} -> Status: {:#?}",
        txn.payment_type, txn.uuid, txn.is_atomic, txn.transaction_status
    );
}

#[derive(Debug)]
#[allow(dead_code)]
enum Transactions {
    Failed(String, u32),
    Successful,
}

struct Payment {
    payment_type: String,
    uuid: u32,
    is_atomic: bool,
    transaction_status: Transactions,
}

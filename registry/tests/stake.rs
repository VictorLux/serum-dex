mod common;

#[test]
fn stake() {
    let client = common::client();
    let accounts = [];
    client.donate(&accounts, 13, false);
}

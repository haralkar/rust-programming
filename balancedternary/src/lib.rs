
pub fn as_bal_ter(_frm: i32) -> String {
    String::from("0")
}

#[cfg(test)]
mod tests {
    use super::as_bal_ter;

#[test]
fn it_works() {
    assert_eq!(as_bal_ter(0), "0");
}

}

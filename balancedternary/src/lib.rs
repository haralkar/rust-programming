


pub fn as_bal_ter(frm: i32) -> String {
    helper(frm, 1)
}
pub fn helper(frm: i32, _mul: i32) -> String {
    if frm == 0 {
        return String::from("0")
    }

    if frm > 0 {
        return String::from("+")
    }
    return String::from("-")
}

#[cfg(test)]
mod tests {
    use super::as_bal_ter;

#[test]
fn null_is() {
    assert_eq!(as_bal_ter(0), "0");
}
#[test]
fn one_is_plus() {
    assert_eq!(as_bal_ter(1), "+");
}
#[test]
fn minus_is_dash() {
    assert_eq!(as_bal_ter(-1), "-");
}
/*
#[test]
fn three_is_ten() {
    assert_eq!(as_bal_ter(3), "10");
}
// */

}

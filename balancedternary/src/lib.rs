


pub fn as_bal_ter(frm: i32) -> String {
    helper(frm, 1)
}
pub fn helper(frm: i32, mul: i32) -> String {
    let mut out = if mul < frm.abs() {
        helper(frm, mul*3)
    } else {
        String::from("")
    };

    out.push_str(if frm == 0 {
        "0"
    } else if frm > 0 {
        "+"
    } else {
        "-"
    });
    out
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

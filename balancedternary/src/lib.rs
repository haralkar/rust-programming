
pub fn helper(frm: &mut i32, mul: i32) -> String {
    println!("+  {:?}", (&frm, &mul));
    let mut out = if mul < frm.abs() {
        helper(frm, mul*3)
    } else {
        String::from("")
    };
    println!(" + {:?}", (&frm, &mul));

    out.push_str(if *frm < 0 {
        *frm += mul;
        "-"
    } else if *frm > 0 {
        *frm -= mul;
        "+"
    } else {
        "0"
    });
    println!("  +{:?}", (&frm, &mul, &out));
    out
}
pub fn as_bal_ter(frm: i32) -> String {
    println!("\nfrom, multipl");
    let mut from = frm;
    helper(&mut from, 1)
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
#[test]
fn three_is_ten() {
    assert_eq!(as_bal_ter(3), "+0");
}
#[test]
fn nine_is_hundred() {
    assert_eq!(as_bal_ter(9), "+00");
}
#[test]
fn my_nine_is_dashded() {
    assert_eq!(as_bal_ter(-9), "-00");
}



//*
// */

}

//! balanced terary yay!
//!
//!  ```
//!  use balancedternary;
//!
//!  assert_eq!(as_bal_ter(-8), "-0+");
//!  ```

fn helper(frm: &mut i32, mul: i32) -> String {
    let mut out = if frm.abs() != mul && 3*mul/2 < frm.abs() {
        helper(frm, mul*3)
    } else {
        String::from("")
    };

    out.push_str( if frm.abs() <= mul/2 {
        "0"
    } else if *frm < 0 {
        *frm += mul;
        "-"
    } else if *frm > 0 {
        *frm -= mul;
        "+"
    } else {
        "0"
    });
    out
}

/// This function changes decimal to balanced ternary
///  ```
///  use balancedternary;
///
///  assert_eq!(as_bal_ter(-8), "-0+");
///  ```
pub fn as_bal_ter(frm: i32) -> String {
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
#[test]
fn thirteen_all_pos() {
    assert_eq!(as_bal_ter(13), "+++");
}
#[test]
fn five_with_two_dashes() {
    assert_eq!(as_bal_ter(5), "+--");
}

#[test]
fn center_city_is_oneoone() {
    assert_eq!(as_bal_ter(10), "+0+");
}
#[test]
fn eigth_is_dashing() {
    assert_eq!(as_bal_ter(8), "+0-");
}
#[test]
fn eleven_is_ttn() {
    assert_eq!(as_bal_ter(11), "++-");
}
#[test]
fn dashodash_dark_city() {
    assert_eq!(as_bal_ter(-10), "-0-");
}

}

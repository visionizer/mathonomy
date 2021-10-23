use mathonomy::relativity::lorentz::*;

#[test]
fn lorentz() {
    assert_eq!(
        slorentz(mathonomy::consts::SPEED_OF_LIGHT),
        mathonomy::consts::INFINITY
    );

    assert_eq!(
        slorentz(mathonomy::consts::SPEED_OF_LIGHT - 1f64),
        12243.211587721227
    )
}

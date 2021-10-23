/// # Simple Lorentz
/// Returns the lorentz factor at a given speed
/// where
/// - t = 1
/// ## Variables
/// - speed (v :: m/s) = The relative velocity between inertial reference frames
/// ## Related Functions
/// slorentzt => Lorentz, but t != 1
/// clorentz => Complex Lorentz
/// slorentztr => Lorentz, but t != 1 and you pass in your won β
/// ## Read more
/// https://en.wikipedia.org/wiki/Lorentz_factor
pub fn slorentz(speed: f64) -> f64 {
    slorentzt(speed, 1f64)
}

/// # Simple Lorentz where t != 0
/// Returns the lorentz factor at a given speed
///
/// ## Variables
/// - speed (v :: m/s) = The relative velocity between inertial reference frames
/// - time (t :: s) = The coordinate time
/// ## Related Functions
/// slorentz => Lorentz with t = 1
/// slorentztr => Lorentz, but t != 1 and you pass in your won β
/// clorentz => Complex Lorentz
///
/// ## Read more
/// https://en.wikipedia.org/wiki/Lorentz_factor
pub fn slorentzt(speed: f64, time: f64) -> f64 {
    slorentztr(
        time,
        speed.powf(2f64) / crate::consts::SPEED_OF_LIGHT.powf(2f64),
    )
}

/// # Complex Lorentz
/// Returns the lorentz factor at a given speed
/// ## Variables
/// - time (t :: s) = The coordinate time
/// - ratio (β :: int) = The ratio of v to c
/// ## Related Functions
/// slorentzt => Lorentz, but t != 1
/// slorentz => Lorentz with t = 1
/// slorentztr => Lorentz, but t != 1 and you pass in your won β
///
/// ## Read more
/// https://en.wikipedia.org/wiki/Lorentz_factor
pub fn slorentztr(time: f64, ratio: f64) -> f64 {
    clorentz(time, (1f64 - ratio).sqrt())
}

/// # Complex Lorentz
/// Returns the lorentz factor at a given speed
/// ## Variables
/// - time (t :: s) = The coordinate time
/// - ptime (τ :: s) = The proper time for an observer
/// ## Related Functions
/// slorentzt => Lorentz, but t != 1
/// slorentz => Lorentz with t = 1
/// slorentztr => Lorentz, but t != 1 and you pass in your won β
/// ## Read more
/// https://en.wikipedia.org/wiki/Lorentz_factor
pub fn clorentz(time: f64, ptime: f64) -> f64 {
    time / ptime
}

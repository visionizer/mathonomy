//! # Lorentz Factor Calculations
//! This file provides functions for the calculation of the lorentz factor
//! The lorentz factor, also known as the Gamma Factor, is a quantity expressing
//! how much the measurements of time, length, and other physical properties change for an object
//! while said object is moving.
//!
//! ## Symbol
//! γ (The Greek Symbol for Gamma)
//! (Sometimes in relation of superliminal motion) Γ (Uppercase Gamma)
//!
//! ## Definition
//! The definition of the lorentz factor is as follows:
//!
//! Δt / Δτ'
//!
//! where t is the coordinate time and τ is the proper time for an observer (measuring time intervals in the observer's own frame)
//!
//! If we step one step further and simplify this formula by saying that
//! Δt is 1s and by showing the definition of Δτ', we are left with this construct:
//!
//! 1 / SquareRoot( 1 - β^2 )
//!
//! Where β is the ratio of v to c
//! Therefore,
//!
//! 1 / SquareRoot( 1 - (v^2 / c^2)) = γ

use crate::prelude::*;

/// # Simple Lorentz
/// Returns the lorentz factor at a given speed where deltatime = 1
///
/// ## Variables
/// ### Parameters
/// - speed (v :: m/s) = The relative velocity between inertial reference frames
/// ## Returns
/// - Lorentz Factor (γ :: γ)
///
/// ## Examples
/// ```rs
/// let lorentz = slorentz(SPEED_OF_LIGHT);
/// ```
///
/// ## Related Functions
/// `slorentzt` => Lorentz, but passing in deltatime is required
/// `clorentz` => Complex Lorentz
/// `slorentztr` => Lorentz, but passing in deltatime and β is required
///
/// ## Read more
/// <https://en.wikipedia.org/wiki/Lorentz_factor>
pub fn slorentz(speed: f64) -> f64 {
    slorentzt(speed, 1f64)
}

/// # Simple Lorentz
/// Returns the lorentz factor at a given speed
///
/// ## Variables
/// ### Parameters
/// - speed (v :: m/s) = The relative velocity between inertial reference frames
/// - deltatime (Δt :: s) = The coordinate time
/// ### Returns
/// - Lorentz Factor (γ :: γ)
///
/// ## Examples
/// ```rs
/// let lorentz = slorentzt(SPEED_OF_LIGHT, 1);
/// ```
///
/// ## Related Functions
/// `slorentzt` => Lorentz, but passing in deltatime is required
/// `clorentz` => Complex Lorentz
/// `slorentztr` => Lorentz, but passing in deltatime and β is required
///
/// ## Read more
/// <https://en.wikipedia.org/wiki/Lorentz_factor>
pub fn slorentzt(speed: f64, deltatime: f64) -> f64 {
    slorentztr(
        deltatime,
        speed.powi(2) / crate::consts::SPEED_OF_LIGHT_SQUARED,
    )
}

/// # Complex Lorentz
/// Returns the lorentz factor at a given speed
///
/// ## Variables
/// ### Parameters
/// - deltatime (Δt :: s) = The coordinate time
/// - ratio (β :: int) = The ratio of v^2 to c^2
/// ### Returns
/// - Lorentz Factor (γ :: γ)
///
/// ## Examples
/// ```rs
/// let lorentz = slorentztr(1, (SPEED_OF_LIGHT - 1f64).powi(2) / SPEED_OF_LIGHT.powi(2));
/// ```
///
/// ## Related Functions
/// `slorentzt` => Lorentz, but passing in deltatime is required
/// `clorentz` => Complex Lorentz
/// `slorentztr` => Lorentz, but passing in deltatime and β is required
///
/// ## Read more
/// <https://en.wikipedia.org/wiki/Lorentz_factor>
pub fn slorentztr(deltatime: f64, ratio: f64) -> f64 {
    clorentz(deltatime, (1f64 - ratio).sqrt())
}

/// # Complex Lorentz
/// Returns the lorentz factor at a given speed
///
/// ## Variables
/// ### Parameters
/// - time (Δt :: s) = The coordinate time
/// - ptime (Δτ :: s) = The proper time for an observer
/// ### Returns
/// - Lorentz Factor (γ :: γ)
///
/// ## Examples
/// ```rs
/// let lorentz = clorentz(1, (1 - ((SPEED_OF_LIGHT - 1f64).powi(2) / SPEED_OF_LIGHT.powi(2))).sqrt());
/// // Or, a bit slower as this was a bit much
/// let speed_of_object = SPEED_OF_LIGHT - 1f64;
/// let ratio = speed_of_object.powi(2) / SPEED_OF_LIGHT.powi(2);
/// let ptime = (1 - ratio).sqrt();
/// let deltatime = 1; // Elapsed time is one
/// let lorentz_fac = clorentz(deltatime, ptime);
/// ```
///
/// ## Related Functions
/// `slorentzt` => Lorentz, but passing in deltatime is required
/// `clorentz` => Complex Lorentz
/// `slorentztr` => Lorentz, but passing in deltatime and β is required
///
/// ## Read more
/// <https://en.wikipedia.org/wiki/Lorentz_factor>
pub fn clorentz(deltatime: f64, ptime: f64) -> f64 {
    deltatime / ptime
}

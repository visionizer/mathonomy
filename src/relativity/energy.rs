use crate::consts::SPEED_OF_LIGHT_SQUARED;

use super::lorentz::slorentz;
use crate::prelude::*;
/// # Simple Kinetic Energy
/// Calculates the relativistic kinetic energy
///
/// ## Relativistic vs Newtonian
/// Use this equation if `v` is bigger than 1% of `c`
///
/// ## Variables
/// ### Parameters
/// - speed (v :: m/s)
/// - mass (m :: kg) = The mass of the body
/// ### Returns
/// - Kinetic Energy (KE :: J)
///
/// ## Example
/// ```rs
/// let joules = skinetic_energy(SPEED_OF_LIGHT, 1)
/// ```
///
/// ## Related Functions
/// `ckinetic_energy` => Complex Version, passing in a custom lorentz factor is required
///
/// ## Read more
/// <https://en.wikipedia.org/wiki/Kinetic_energy#Relativistic_kinetic_energy_of_rigid_bodies>
pub fn skinetic_energy(speed: f64, mass: f64) -> f64 {
    ckinetic_energy(speed, mass, slorentz(speed))
}

/// # Simple Kinetic Energy
/// Calculates the relativistic kinetic energy
///
/// ## Relativistic vs Newtonian
/// Use this equation if v is bigger than 1% of c
///
/// ## Variables
/// ### Parameters
/// - speed (v :: m/s)
/// - mass (m :: kg) = The mass of the body
/// ## Returns
/// - Kinetic Energy (KE :: J)
///
/// ## Examples
/// ```rs
/// let joules = ckinetic_energy(SPEED_OF_LIGHT, 1, slorentz(SPEED_OF_LIGHT))
/// ```
///
/// ## Related Functions
/// `skinetic_energy` => The simple version, lorentz factor is emitted
///
/// ## Read more
/// <https://en.wikipedia.org/wiki/Kinetic_energy#Relativistic_kinetic_energy_of_rigid_bodies>
pub fn ckinetic_energy(speed: f64, mass: f64, lorentz: f64) -> f64 {
    (lorentz - 1f64) * mass * speed.powi(2)
}

use crate::consts::SPEED_OF_LIGHT;

use super::lorentz::slorentz;

/// # Simple Kinetic Energy
/// Calculates the relativistic kinetic energy
/// ## Relativistic vs Newtonian
/// Use this formula if v is bigger than 1% of c
///
/// ## Variables
/// - speed (v)
/// - mass (m) = The mass of the body
///
/// ## Related Functions
/// kinetic_energy => The same, but you have to calculate the lorentz factor yourself
///
/// ## Read more
/// https://en.wikipedia.org/wiki/Kinetic_energy#Relativistic_kinetic_energy_of_rigid_bodies
pub fn skinetic_energy(speed: f64, mass: f64) -> f64 {
    kinetic_energy(speed, mass, slorentz(speed))
}

/// # Simple Kinetic Energy
/// Calculates the relativistic kinetic energy
///
/// ## Relativistic vs Newtonian
/// Use this formula if v is bigger than 1% of c
///
/// ## Variables
/// - speed (v :: m/s)
/// - mass (m :: kg) = The mass of the body
///
/// ## Related Functions
/// kinetic_energy => The same, but you have to calculate the lorentz factor yourself
///
/// ## Read more
/// https://en.wikipedia.org/wiki/Kinetic_energy#Relativistic_kinetic_energy_of_rigid_bodies
pub fn kinetic_energy(speed: f64, mass: f64, lorentz: f64) -> f64 {
    (lorentz - 1f64) * mass * SPEED_OF_LIGHT.powi(2)
}

//! # Constant Values
//! Many values are constant in this world, and often times, said values are used in
//! mathematics. In this file, some constants that are often used are defined.
//!
//! ## Explanations
//! If you are interested in what values truly are / some fun facts about them,
//! visit the pages of the value. **Some** values have docs attached to them. Those values only get
//! Documentations if they are often used. Other constants just exist for completions sake.

/// # Speed Of Light in a vacuum
/// Speed of Photons **in a vacuum**
/// The Speed of Light differs depending on the material in which the
/// Photons are travelling
///
/// ## Can Speed Of Light be reached?
/// No, speed of light may never be reached by any body with mass due to the following reason:
///
/// This is the equation for the calculation of kinetic energy:
///
/// (γ - 1)mc^2
///
/// If we try to calculate the Lorentz Factor (γ), we would have to use this equation:
/// 1 / SquareRoot(1 - (v^2 / c^2))
///
/// In this equation, while v approaches c, `γ` approaches `∞` therefore, we write that γ is ∞ (While it is not, technically)
///
/// Due to γ being infinity, the relativistic kinetic energy equation equates to ∞
pub const SPEED_OF_LIGHT: f64 = 299_792_458f64;

/// # Speed Of Light (Squared) in a vacuum
/// Speed of Light **in a vacuum** squared.
/// The Speed of Light differs depending on the material in which the
/// Photons are travelling.
///
/// ## Why squared?
/// Often times, for example in Mass-Energy-Equivalence, SPEED_OF_LIGHT is squared, which can be costly.
/// This is why this constant exists.
///
/// ## Can Speed Of Light be reached?
/// No, speed of light may never be reached by any body with mass due to the following reason:
///
/// This is the equation for the calculation of kinetic energy:
///
/// (γ - 1)mc^2
///
/// If we try to calculate the Lorentz Factor (γ), we would have to use this equation:
/// 1 / SquareRoot(1 - (v^2 / c^2))
///
/// In this equation, while v approaches c, `γ` approaches `∞` therefore, we write that γ is ∞ (While it is not, technically)
///
/// Due to γ being infinity, the relativistic kinetic energy equation equates to ∞
pub const SPEED_OF_LIGHT_SQUARED: f64 = 89875517873681760f64;

/// # Infinity in Rust
/// This value represents Infinity in the Rust Programming Language.
/// This constant only exists for tests, comparisons, etc, but should **never**
/// be used for calculations.
///
/// ## What is infinity
/// In Physics, infinity does not exist, at least, not that I know of.
/// It is usually used to denote values such as the result of the following equation, which calculates the Lorentz Factor γ:
/// 1 / SquareRoot(1 - (v^2 / c^2))
///
/// The result of this is ∞, which would, if used in the relativistic kinetic energy equation, make the
/// result of said equation be ∞ too. This only means that as `v` approaches `c`, `γ` approaches `∞`
///
/// ## What is infinity in Rust?
/// In Programming, Infinity is **not** the maximum of a number. It is something defined within the compiler.
/// In the Standard Library it is just defined as 1 / 0.
pub const INFINITY: f64 = f64::INFINITY;

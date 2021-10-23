use mathonomy::relativity::energy::skinetic_energy;

use mathonomy::consts::*;

#[test]
fn relativistic_energy() {
    assert_eq!(skinetic_energy(SPEED_OF_LIGHT, 1), INFINITY)
}

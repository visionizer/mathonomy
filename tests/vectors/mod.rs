use mathonomy::vectors::vec3::Vec3;

#[test]
pub fn test_vectors() {
    let forward = Vec3::forward();
    let backward = Vec3::backward();

    let new = forward + backward;
}

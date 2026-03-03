use redox_math::{
    Vec3,
    transform_matrix,
    look_at,
    perspective,
    Quat,
    Aabb,
    Sphere,
    Frustum
};

#[test]
fn test_aabb_transformations() {
    let aabb = Aabb::from_center_size(Vec3::ZERO, Vec3::new(1.0, 1.0, 1.0));
    let rotation = Quat::from_rotation_y(45_f32.to_radians());
    let scale = Vec3::splat(2.0);
    let matrix = transform_matrix(Vec3::ZERO, rotation, scale);

    let transformed_aabb = aabb.transform(matrix);

    assert!(transformed_aabb.size().x * transformed_aabb.size().y * transformed_aabb.size().z > 8.0);

    let corner = Vec3::new(1.0, 1.0, 1.0);
    let transformed_corner = matrix.transform_point3(corner);
    assert!(transformed_aabb.contains_point(transformed_corner));
}

#[test]
fn test_frustum_extraction_and_culling() {
    let proj = perspective(90_f32.to_radians(), 1.0, 0.1, 100.0);
    let view = look_at(Vec3::new(0.0, 0.0, 5.0), Vec3::ZERO, Vec3::Y);
    let frustum = Frustum::from_view_projection(proj * view);

    let center_aabb = Aabb::from_center_size(Vec3::ZERO, Vec3::splat(1.0));
    assert!(frustum.intersects_aabb(&center_aabb));

    let behind_aabb = Aabb::from_center_size(Vec3::new(0.0, 0.0, 10.0), Vec3::splat(1.0));
    assert!(!frustum.intersects_aabb(&behind_aabb));
}

#[test]
fn test_sphere_logic() {
    let sphere = Sphere::new(Vec3::new(1.0, 2.0, 3.0), 5.0);
    let other_sphere = Sphere::new(Vec3::new(2.0, 2.0, 3.0), 1.0);
    assert!(sphere.intersects(&other_sphere));
}

#[test]
fn test_coordinate_system_sanity() {
    assert!((Vec3::Y.y - 1.0).abs() < f32::EPSILON);
    let x = Vec3::X;
    let y = Vec3::Y;
    let z = x.cross(y);
    assert!((z.z - 1.0).abs() < f32::EPSILON);
}
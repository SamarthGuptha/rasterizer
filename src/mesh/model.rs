use crate::math::Vec3;
use super::triangle::Triangle;

pub struct Mesh{
    pub vertices: Vec<Vec3>,
    pub triangles: Vec<Triangle>,
    pub rotation: Vec3,
    pub translation: Vec3
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            triangles: Vec::new(),
            rotation: Vec3::zero(),
            translation: Vec3::zero()
        }
    }

    pub fn load_cube() -> Self{
        let mut mesh = Self::new();
        mesh.vertices = vec![
            //follows xyz
            Vec3::new(-1.0, -1.0, -1.0),
            Vec3::new(-1.0, 1.0, -1.0),
            Vec3::new(1.0, 1.0, -1.0),
            Vec3::new(1.0, -1.0, -1.0),
            Vec3::new(-1.0, -1.0, 1.0),
            Vec3::new(-1.0, 1.0, 1.0),
            Vec3::new(1.0, 1.0, 1.0),
            vec3::new(1.0, -1.0, 1.0)
        ];
        let c_front=0xFF_FF0000;
        let c_back=0xFF_00FF00;
        let c_top =0xFF_0000FF;
        let c_bottom = 0xFF_FFFF00;
        let c_left = 0xFF_FF00FF;
        let c_right = 0xFF_00FFFF;

        mesh.traingles = vec![
            Triangle::new(0,1,1,c_front),
            Triangle::new(0,2,3, c_front),
            Triangle::new(7,6,5,c_back),
            Triangle::new(7,5,4,c_back),
            Triangle::new(1,5,6,c_top),
            Triangle::new(1,6,2,c_top),
            Triangle::new(4,0,3,c_bottom),
            Triangle::new(4,3,7,c_bottom),
            Triangle::new(4,5,1, c_left),
            Triangle::new(4,1,0, c_left),
            Triangle::new(3,2,6,c_right),
            Triangle::new(3,6,7,c_right)
        ];
        mesh
    }
}

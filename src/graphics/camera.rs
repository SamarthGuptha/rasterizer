use crate::math::{Mat4x4,Vec3};

pub struct Camera{
    pub position:Vec3,
    pub target:Vec3,
    pub up: Vec3
}

impl Camera{
    pub fn new(position:Vec3,target:Vec3, up: Vec3) -> Self { Self {position, target, up}}

    pub fn build_view_matrix(&self)-> Mat4x4{
        let forward = (self.target-self.position).normalize();
        let up_normalized=self.up.normalize();
        let right=up_normalized.cross(&forward).normalize();
        let new_up=forward.cross(&right);
        let mut mat=Mat4x4::identity();

        mat.m[0][0]=right.x;
        mat.m[1][0]=right.y;
        mat.m[2][0]=right.z;

        mat.m[0][1]=new_up.x;
        mat.m[1][1]=new_up.y;
        mat.m[2][1]=new_up.z;

        mat.m[0][2]=forward.x;
        mat.m[1][2]=forward.y;
        mat.m[2][2]=forward.z;

        mat.m[3][0] = -self.position.dot(&right);
        mat.m[3][1]=-self.position.dot(&new_up);
        mat.m[3][2]=-self.position.dot(&forward);
        mat
    }
}
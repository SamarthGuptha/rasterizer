use crate::math::Mat4x4;

pub struct Projection {
    pub aspect_ratio: f32,
    pub z_near: f32,
    pub z_far:f32,
    pub fov_degrees:f32
}

impl Projection{
    pub fn new(fov_degrees:f32,aspect_ratio:f32,z_near:f32,z_far:f32)-> Self {
        Self {
            z_far,
            z_near,
            aspect_ratio,
            fov_degrees
        }
    }

    pub fn build_matrix(&self)-> Mat4x4{
        let fov_rad = 1.0/(self.fov_degrees*0.5*std::f32::consts::PI/180.0).tan();
        let mut mat = Mat4x4::zero();
        mat.m[0][0]=self.aspect_ratio*fov_rad;
        mat.m[1][1]=fov_rad;

        mat.m[2][2]=self.z_far/(self.z_far-self.z_near);
        mat.m[2][3]=1.0;
        mat.m[3][2]=(-self.z_far*self.z_near)/(self.z_far-self.z_near);
        mat.m[3][3]=0.0;
        mat
    }
}
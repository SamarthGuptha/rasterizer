use super::vec3::Vec3;
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat4x4{ pub m: [[f32;4];4]}

impl Default for Mat4x4{fn default()->Self {Self::zero()}}
impl Mat4x4{
    pub fn zero() -> Self {
        Self {m: [[0.0;4];4]}
    }
    pub fn identity()->Self{
        let mut mat = Self::zero();
        mat.m[0][0]=1.0;
        mat.m[1][1]=1.0;
        mat.m[2][2]=1.0;
        mat.m[3][3]=1.0;

        mat
    }


    pub fn make_translation(x: f32, y:f32,z:f32)-> Self{
        let mut mat = Self::identity();
        mat.m[3][0]=x;
        mat.m[3][1]=y;
        mat.m[3][2]=z;
        mat
    }
    pub fn make_rotation_x(angle_rad:f32)-> Self{
        let mut mat = Self::identity();
        let c=angle_rad.cos();
        let s = angle_rad.sin();
        mat.m[1][1]=c;
        mat.m[1][2]=s;
        mat.m[2][1]=-s;
        mat.m[2][2]=c;
        mat
    }
    pub fn make_rotation_y(angle_rad:f32)-> Self{
        let mut mat = Self::identity();
        let c=angle_rad.cos();
        let s=angle_rad.sin();
        mat.m[0][0]=c;
        mat.m[0][2]=-s;
        mat.m[2][0]=s;
        mat.m[2][2]=c;
        mat
    }
    pub fn make_rotation_z(angle_rad: f32)-> Self{
        let mut mat = Self::identity();
        let c = angle_rad.cos();
        let s = angle_rad.sin();
        mat.m[0][0]=c;
        mat.m[0][1]=s;
        mat.m[1][0]=-s;
        mat.m[1][1]=c;
        mat
    }
    pub fn mul_mat4(&self, other: &Self)-> Self{
        let mut result = Self::zero();
        for r in 0..4{
            for c in 0..4{
                result.m[r][c]=self.m[r][0]*other.m[0][c]+self.m[r][1]*other.m[1][c]
                               + self.m[r][2]*other.m[2][c]+self.m[r][3]*other.m[3][c];
            }

        }
        result

    }
    pub fn mul_vec3(&self, v: &Vec3)-> (Vec3, f32){
        let x = v.x*self.m[0][0]+v.y*self.m[1][0]+v.z*self.m[2][0]+self.m[3][0];
        let y = v.x*self.m[0][1]+v.y*self.m[1][1]+v.z*self.m[2][1]+self.m[3][1];
        let z = v.x*self.m[0][2]+v.y*self.m[1][2]+v.z*self.m[2][2]+self.m[3][2];
        let w = v.x*self.m[0][3]+v.y*self.m[1][3]+v.z*self.m[2][3]+self.m[3][3];

        (Vec3::new(x,y,z),w)
    }
}
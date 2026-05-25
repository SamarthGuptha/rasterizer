use crate::graphics::{Camera, Framebuffer, Projection};
use crate::math::{Mat4x4, Vec2, Vec3};
use crate::mesh::Mesh;

pub struct Engine {
    pub framebuffer: Framebuffer,
    pub height: usize,
    pub width: usize,
    pub projection: Projection,
    pub camera: Camera,
    pub mesh: Mesh
}

impl Engine {
    pub fn new(width: usize, height: usize) -> Self {
        let framebuffer = Framebuffer::new(width, height);

        let camera = Camera::new(
          Vec3::new(0.0, 0.0, -5.0),
          Vec3::zero(),
          Vec3::new(0.0, 1.0, 0.0)
        );

        let aspect_ratio = height as f32/width as f32;
        let projection = Projection::new(90.0, aspect_ratio, 0.1, 1000.0);

        Self {
            framebuffer,
            mesh: Mesh::load_cube(),
            camera,
            projection,
            width,
            height
        }
    }

    pub fn update(&mut self, delta_time: f32){
        self.mesh.rotation.x+=1.0*delta_time;
        self.mesh.rotation.y+=1.0*delta_time;
    }

    pub fn render(&mut self) {
        self.framebuffer.clear(0xFF_000000);
        let mat_view = self.camera.build_view_matrix();
        let mat_proj = self.projection.build_matrix();

        let mat_rotx = Mat4x4::make_rotation_x(self.mesh.rotation.x);
        let mat_rotz = Mat4x4::make_rotation_z(self.mesh.rotation.z);
        let mat_roty = Mat4x4::make_rotation_y(self.mesh.rotation.y);
        let mat_trans = Mat4x4::make_translation(self.mesh.translation.x,self.mesh.translation.y,self.mesh.translation.z);

        let mat_world = mat_rotx.mul_mat4(&mat_roty).mul_mat4(&mat_rotz).mul_mat4(&mat_trans);

        for tri in &self.mesh.triangles {
            let mut screen_coords = [(0,0);3];
            let mut w_coords = [0.0; 3];
            let mut out_of_bounds = false;

            for i in 0..3{//transforming all 3 vertces
                let v = &self.mesh.vertices[tri.v[i]];
                let (v_world, _) = mat_world.mul_vec3(v);
                let (v_view, _) = mat_view.mul_vec3(&v_world);
                let (mut v_proj, w) = mat_proj.mul_vec3(&v_view);
                w_coords[i]=w;

                if w<0.1{
                    out_of_bounds=true;
                    break;
                }

                v_proj.x /=w;
                v_proj.y/=w;
                v_proj.z /= w;

                let screen_x = ((v_proj.x+1.0)*0.5*self.width as f32) as i32;
                let screen_y = ((1.0-v_proj.y)*0.5*self.height as f32) as i32;

                screen_coords[i] = (screen_x, screen_y);
            }

            if out_of_bounds {continue;}
            let vec_a = Vec2::new(
                (screen_coords[1].0-screen_coords[0].0) as f32,
                (screen_coords[1].1 - screen_coords[0].1) as f32
            );
            let vec_b = Vec2::new(
                (screen_coords[2].0-screen_coords[0].0) as f32,
                (screen_coords[2].1 - screen_coords[0].1) as f32
            );
            let cross_z=vec_a.x*vec_b.y - vec_a.y*vec_b.x;

            if cross_z<0.0{continue;}

            self.framebuffer.draw_filled_triangle(
              screen_coords[0].0, screen_coords[0].1,
              screen_coords[1].0, screen_coords[1].1,
              screen_coords[2].0, screen_coords[2].1,
              tri.color
            );
        }
    }
}
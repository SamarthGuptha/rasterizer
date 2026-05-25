pub struct Framebuffer{
    pub width:usize,
    pub height:usize,
    pub color_buffer: Vec<u32>
}


impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            color_buffer: vec![0xFF_00_00_00; width*height]
        }
    }
    pub fn clear(&mut self, color:u32){self.color_buffer.fill(color);}
    pub fn set_pixel(&mut self,x:i32, y:i32, color: u32){
        if x>=0 && x< self.width as i32 && y>=0 && y<self.height as i32 {
            let index = (y as usize)*self.width+(x as usize);
            self.color_buffer[index]=color;
        }
    }
    pub fn draw_line(&mut self, mut x0:i32, mut y0:i32, x1:i32, y1:i32, color: u32) {
        let dx = (x1-x0).abs();
        let sx = if x0<x1{1} else {-1};
        let dy = -(y1-y0).abs();
        let sy = if y0<y1{1}else{-1};
        let mut err = dx+dy;

        loop {
            self.set_pixel(x0,y0, color);
            if x0==x1&&y0==y1{break;}
            let e2 = 2*err;
            if e2>=dy{
                err+=dy;
                x0+=sx;
            }
            if e2<=dx{
                err+=dx;
                y0+=sy;
            }
        }
    }

    pub fn draw_filled_triangle(&mut self, mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, mut x2: i32, mut y2: i32, color:u32){
        if y0 > y1 { std::mem::swap(&mut y0, &mut y1); std::mem::swap(&mut x0, &mut x1); }
        if y1 > y2 { std::mem::swap(&mut y1, &mut y2); std::mem::swap(&mut x1, &mut x2); }
        if y0 > y1 { std::mem::swap(&mut y0, &mut y1); std::mem::swap(&mut x0, &mut x1); }

        if y1 == y2 {
            self.fill_flat_bottom(x0, y0, x1, y1, x2, y2, color);

        } else if y0 == y1{
            self.fill_flat_top(x0,y0,x1,y1,x2,y2,color);
        } else {
            let x_mid = x0+((y1-y0) as f32/(y2-y0) as f32*(x2-x0) as f32) as i32;
            self.fill_flat_bottom(x0,y0,x1,y1,x_mid,y1,color);
            self.fill_flat_top(x1, y1, x_mid, y1, x2, y2, color);
        }
    }

    fn fill_flat_bottom(&mut self, x0:i32, y0: i32, x1: i32, y1: i32, x2: i32, y2: i32, color: u32) {
        let invslope1=(x1-x0) as f32/(y1-y0) as f32;
        let invslope2 = (x2-x0) as f32/(y2-y0) as f32;
        let mut curx1=x0 as f32;
        let mut curx2 = x0 as f32;

        for y in y0..=y1 {
            self.draw_scanline(curx1 as i32, curx2 as i32, color);
            curx1+=invslope1;
            curx2+=invslope2;
        }
    }
    fn fill_flat_top(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, x2: i32, y2: i32, color: u32) {
        let invslope1=(x2-x0) as f32/(y2-y0)as f32;
        let invslope2=(x2-x1)as f32/(y2-y1) as f32;
        let mut curx1=x2 as f32;
        let mut curx2=x2 as f32;

        for y in (y0..=y2).rev(){
            self.draw_scanline(curx1 as i32, curx2 as i32, color);
            curx1-=invslope1;
            curx2-=invslope2;
        }
    }

    fn draw_scanline(&mut self, mut x_start: i32, mut x_end: i32, y: i32, color: u32) {
        if x_start>x_end{std::mem::swap(&mut x_start, &mut x_end);}
        for x in x_start..=x_end{self.set_pixel(x, y, color);}
    }

}
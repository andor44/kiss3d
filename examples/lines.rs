extern mod kiss3d;
extern mod nalgebra;

use nalgebra::na::Vec3;
use kiss3d::window;

#[start]
fn start(argc: int, argv: **u8) -> int {
    std::rt::start_on_main_thread(argc, argv, main)
}

fn main() {
    do window::Window::spawn("Kiss3d: lines") |window| {

        window.set_light(window::StickToCamera);

        do window.render_loop |w| {
            let a = Vec3::new(-0.5, -0.5, 0.0);
            let b = Vec3::new(0.0, 0.5, 0.0);
            let c = Vec3::new(0.5, -0.5, 0.0);

            w.draw_line(&a, &b, &Vec3::new(1.0, 0.0, 0.0));
            w.draw_line(&b, &c, &Vec3::new(0.0, 1.0, 0.0));
            w.draw_line(&c, &a, &Vec3::new(0.0, 0.0, 1.0));
        }
    }
}

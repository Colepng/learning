use glium::{Surface, texture::UncompressedFloatFormat, glutin::error::NotSupportedError};
use std::io::Cursor;
use image;

mod teapot;

#[macro_use]
extern crate glium;


const vertex_shader_src: &str = r#"
    #version 150

    in vec3 position;
    in vec3 normal;

    out vec3 v_normal;

    uniform mat4 perspective;
    uniform mat4 view;
    uniform mat4 model;

    void main() {
        mat4 modelview = view * model;
        v_normal = transpose(inverse(mat3(modelview))) * normal; 
        gl_Position = perspective * modelview * vec4(position, 1.0); 
    }
"#;

const fragment_shader_src: &str = r#"
    #version 150
   
    in vec3 v_normal;
    out vec4 color;
    uniform vec3 u_light;

    void main() {
        float brightness = dot(normalize(v_normal), normalize(u_light));
        vec3 dark_color = vec3(0.3135, 0.0625, 0.446);
        vec3 regular_color = vec3(0.627, 0.125, 0.941);
        color = vec4(mix(dark_color, regular_color, brightness), 1.0);
    }
"#;



fn main() {
    use glium::glutin;

    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    
    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
                                      &teapot::INDICES).unwrap();
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    let mut t: f32 = 0.0;
    let mut increase = true;
    event_loop.run(move |ev, _, control_flow| {

        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            _ => (),
        }

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        
        if (t*100.0).round() / 100.0 == 1.0 {
            increase = false;
        } else if (t*100.0).round() / 100.0 == -1.0 {
            increase = true;
        }

        if increase {
            t += 0.00005;
        } else  {
            t -= 0.00005;
        }


        // println!("{counter}");
        let mut target = display.draw();
        target.clear_color_and_depth((1.0, 1.0, 1.0, 1.0), 1.0); 

        
        let light = [-1.0, 0.4, 0.9f32];

        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            let fov: f32 = 3.141592 / 3.0;
            let zfar = 1024.0;
            let znear = 0.1;

            let f = 1.0 / (fov / 2.0).tan();

            [
                [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
                [         0.0         ,     f ,              0.0              ,   0.0],
                [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
                [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
            ]
        };        

        let params = glium::DrawParameters {

            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            // backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            .. Default::default()
        };

        let view = view_matrix(&[5.0, 5.0, 1.0], &[-5.0, -5.0, 1.0], &[0.0, 1.0, 0.0]);

        let unifroms = uniform! {
            model: [
                [0.01, 0.0, 0.0, 0.0],
                [0.0, 0.01, 0.0, 0.0],
                [0.0, 0.0, 0.01, 0.0],
                [0.0 , 0.0, 2.0, 1.0f32],
            ],
            perspective: perspective,
            view: view,
            u_light: light,
        };

        target.draw((&positions, &normals), &indices, &program, &unifroms,
            &params).unwrap();
        target.finish().unwrap();
    });
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
             up[2] * f[0] - up[0] * f[2],
             up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
             f[2] * s_norm[0] - f[0] * s_norm[2],
             f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}

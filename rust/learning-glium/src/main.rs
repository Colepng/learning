use glium::{Surface, texture::UncompressedFloatFormat};

#[macro_use]
extern crate glium;


const vertex_shader_src: &str = r#"
    #version 140

    in vec2 position;

    uniform mat4 matrix;

    void main() {
        vec2 pos = position;
        gl_Position = matrix * vec4(position, 0.0, 1.0); 
    }
"#;

const fragment_shader_src: &str = r#"
    #version 140

    out vec4 color;
    uniform float r;
    uniform float g;
    uniform float b;
    uniform float a;

    void main() {

        color = vec4(r, g, b, a);
    }
"#;



fn main() {
    use glium::glutin;

    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    let counter: f32 = 1.0;
    let mut increase: bool = true;

    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.0,  0.5] };
    let vertex3 = Vertex { position: [ 0.5, -0.25] };
    let shape = vec![vertex1, vertex2, vertex3];
    let mut t: f32 = -0.5;

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
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
            t += 0.001;
        } else  {
            t -= 0.001;
        }

        // println!("{counter}");
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, counter); 

        let unifroms = uniform! {
            matrix: [
                [t.sin(), t.tan(), 0.0, 0.0],
                [-t.sin(), t.tan(), 0.0, 0.0],
                [1.0, 0.0, 1.0, 0.0],
                [ t , 0.0, 0.0, 1.0f32],
            ],
            r: 0.627 as f32,
            g: 0.125 as f32,
            b: 0.941 as f32,
            a: 1.0 as f32
        };

        target.draw(&vertex_buffer, indices, &program, &unifroms,
            &Default::default()).unwrap();
        target.finish().unwrap();
    });
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

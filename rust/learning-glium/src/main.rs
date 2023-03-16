use glium::Surface;

#[macro_use]
extern crate glium;


const vertex_shader_src: &str = r#"
    #version 140

    in vec2 position;

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }
"#;

const fragment_shader_src: &str = r#"
    #version 140

    out vec4 color;

    void main() {
        color = vec4(0.6, 0.2, 0.7, 1.0);
    }
"#;



fn main() {
    use glium::glutin;

    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    let mut counter: f32 = 1.0;
    let mut increase: bool = true;

    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.0,  0.5] };
    let vertex3 = Vertex { position: [ 0.5, -0.25] };
    let mut shape = vec![vertex1, vertex2, vertex3];
    let mut t: f32 = 0.0;

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
    event_loop.run(move |ev, _, control_flow| {
        if (t*100.0).round() / 100.0 == 1.0 {
            increase = false;
        } else if (t*100.0).round() / 100.0 == -1.0 {
            increase = true;
        }

        if increase {
            t += 0.0001;
        } else  {
            t -= 0.0001;
        }
        let vertex1 = Vertex { position: [-0.5 + t, -0.5 + t] };
        let vertex2 = Vertex { position: [ 0.0 + t,  0.5 + t] };
        let vertex3 = Vertex { position: [ 0.5 + t, -0.25 + t] };
        let mut shape = vec![vertex1, vertex2, vertex3];
        let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

        // println!("{counter}");
        let mut target = display.draw();
        target.clear_color(0.5, 0.0, 0.5, counter); 
        target.draw(&vertex_buffer, indices, &program, &glium::uniforms::EmptyUniforms,
            &Default::default()).unwrap();
        target.finish().unwrap();

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
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
    });
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

impl Vertex {
    fn up(&mut self) {
        for mut i in self.position {
            i += 0.001;
        }
    }
}

implement_vertex!(Vertex, position);

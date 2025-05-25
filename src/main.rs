use glfw::{Action, Context, Key};
use nalgebra_glm as glm;
use std::fs;
use std::ptr;

mod shader { pub mod shader; }
use shader::shader::Shader;

fn main() {
    // Initialize GLFW
    let mut glfw = glfw::init(glfw::fail_on_errors).expect("Failed to initialize GLFW");
    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(4));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(1));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true)); // For macOS
    // Create and setup window
    let (mut window, events) = glfw
        .create_window(1200, 900, "Virtual Universe!", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");
    window.make_current();
    window.set_key_polling(true);
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    gl::load_with(|s| window.get_proc_address(s) as *const _);
    unsafe { gl::Enable(gl::DEPTH_TEST) }

    // Geometry
    let vertices: [f32; 18] = [
        0.0, 1.0, 0.0,    1.0, 0.0, 0.0,
        -1.0, -0.5, 0.0,  0.0, 1.0, 0.0,
        1.0, -0.5, 0.0,   0.0, 0.0, 1.0,
    ];
    let indices: [u32; 3] = [0, 1, 2];

    let (mut vao, mut vbo, mut ebo) = (0, 0, 0);
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as isize,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<u32>()) as isize,
            indices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        let stride = 6 * std::mem::size_of::<f32>() as i32;
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * 4) as *const _);
        gl::EnableVertexAttribArray(1);

        gl::BindVertexArray(0);
    }

    // Load shaders from file
    let vertex_src = fs::read_to_string("src/shaders/simple.vs").expect("Missing vertex shader");
    let fragment_src = fs::read_to_string("src/shaders/simple.fs").expect("Missing fragment shader");
    let shader = Shader::new(&vertex_src, &fragment_src).expect("Shader failed");

    let cam_pos = glm::vec3(0.0, 0.0, 2.0);
    let cam_dir = glm::vec3(0.0, 0.0, -1.0);
    let up = glm::vec3(0.0, 1.0, 0.0);

    while !window.should_close() {
        glfw.poll_events();
        // Input, when the useer presses esc the window will close
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                }
                _ => {}
            }
        }

        // Generate a rgb value based on time and set the background color to that
        let t = glfw.get_time() as f32;
        unsafe {
            gl::ClearColor(
                (t * 0.5).sin() * 0.5 + 0.25,
                (t * 0.3).sin() * 0.5 + 0.25,
                (t * 0.7).sin() * 0.5 + 0.25,
                1.0,
            );
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // Matrices
        let model = glm::rotate(&glm::identity(), (t * 0.8).sin() / 4.0, &glm::vec3(0.0, 0.0, 1.0));
        let view = glm::look_at(&cam_pos, &(cam_pos + cam_dir), &up);
        let (w, h) = window.get_framebuffer_size();
        let proj = glm::perspective(std::f32::consts::FRAC_PI_2, w as f32 / h as f32, 0.01, 100.0);

        shader.use_program();
        shader.set_mat4("u_model", &model);
        shader.set_mat4("u_view", &view);
        shader.set_mat4("u_projection", &proj);

        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, ptr::null());
        }

        window.swap_buffers();
    }

    unsafe {
        gl::DeleteVertexArrays(1, &vao);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteBuffers(1, &ebo);
    }
}

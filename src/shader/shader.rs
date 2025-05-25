use gl;
use nalgebra_glm as glm;
use std::{ffi::CString, ptr, str};

pub struct Shader { id: u32 }

impl Shader {
    pub fn new(vertex_src: &str, fragment_src: &str) -> Result<Self, String> {
        unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);

            Self::compile_shader(vertex_shader, vertex_src)?;
            Self::compile_shader(fragment_shader, fragment_src)?;

            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);

            let mut success = 0;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut len = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
                let mut buffer = vec![0u8; len as usize];
                gl::GetProgramInfoLog(
                    program,
                    len,
                    ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut i8,
                );
                return Err(str::from_utf8(&buffer).unwrap().to_string());
            }

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            Ok(Self { id: program })
        }
    }

    unsafe fn compile_shader(shader: u32, source: &str) -> Result<(), String> {
        let c_str = CString::new(source).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        let mut success = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buffer = vec![0u8; len as usize];
            gl::GetShaderInfoLog(
                shader,
                len,
                ptr::null_mut(),
                buffer.as_mut_ptr() as *mut i8,
            );
            return Err(str::from_utf8(&buffer).unwrap().to_string());
        }
        Ok(())
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn set_mat4(&self, name: &str, matrix: &glm::Mat4) {
        let cname = CString::new(name).unwrap();
        unsafe {
            let loc = gl::GetUniformLocation(self.id, cname.as_ptr());
            gl::UniformMatrix4fv(loc, 1, gl::FALSE, matrix.as_ptr());
        }
    }
}

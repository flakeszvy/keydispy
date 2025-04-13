use std::path::Path;

use ferrousgl::{GlWindow, Mesh, Shader, WindowConfig};
use glam::{Mat4, Vec3};

fn main() {
    let mut window = GlWindow::new(WindowConfig {
        ..Default::default()
    });

    let mut mesh = Mesh::new();
    let vertices = [
        -0.5, -0.5, 0.0,  1.0, 0.0, 0.0,
        -0.5, 0.5, 0.0,   0.0, 1.0, 0.0,
        0.5, 0.5, 0.0,    0.0, 0.0, 1.0,
        0.5, -0.5, 0.0,   1.0, 1.0, 0.0,
    ];
    let indices = [0, 1, 2, 0, 2, 3];
    mesh.update_vertices(&vertices);
    mesh.update_indices(&indices);
    mesh.add_vertex_attributes(&[
        (0, 3, gl::FLOAT, false),
        (1, 3, gl::FLOAT, false),
    ]);

    let shader = Shader::new_from_file(
        Path::new("./shaders/vertex.glsl"),
        Path::new("./shaders/fragment.glsl"),
    )
    .unwrap();

    let key_sizes = [
        vec![0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.2],
        vec![0.15, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.15],
        vec![0.175, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.225],
        vec![0.23, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.27],
        vec![0.1286, 0.1286, 0.1286, 0.6, 0.1286, 0.1286, 0.1286, 0.1286]
    ];

    let view = Mat4::look_at_rh(
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    while !window.should_window_close() {
        let (width, height) = window.get_window_size();
        let aspect_ratio = width as f32 / height as f32;

        let projection = if width > height {
            Mat4::orthographic_rh(-1.0 * aspect_ratio, 1.0 * aspect_ratio, -1.0, 1.0, 0.1, 100.0)
        } else {
            Mat4::orthographic_rh(-1.0, 1.0, -1.0 / aspect_ratio, 1.0 / aspect_ratio, 0.1, 100.0)
        };

        window.clear_color(Vec3::new(0.0, 0.0, 0.0));
        window.clear_depth();

        shader.bind_program();

        for (i, inner_array) in key_sizes.iter().enumerate() {
            let mut x_pos = 0.0;
            for (j, &value) in inner_array.iter().enumerate() {
                let model = Mat4::from_translation(Vec3::new(x_pos + value*0.5 as f32, -0.1*i as f32, 1.0)) * Mat4::from_scale(Vec3::new(value, 0.1, 0.1));
                x_pos += value;

                let mvp = projection * view * model;
                shader.set_uniform_matrix_4fv("mvp", mvp.to_cols_array().as_ref());

                window.render_mesh(&mesh);
            }
        }

        shader.unbind_program();

        window.update();
    }
}

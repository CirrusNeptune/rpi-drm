#![recursion_limit = "10000"]
mod shaders;

use rpi_drm::{Buffer, CommandRecorder};
use std::io::{Cursor, Write};
use vc4_drm::{cl::PrimitiveMode, glam::UVec2};

async fn async_main() {
    shaders::initialize_shaders().await;

    let display_framebuffers = rpi_drm::open_and_allocate_display_framebuffers();

    let tex_bo = {
        use vc4_drm::image::*;
        let image_size = UVec2::new(256, 256);
        let (translator, size_in_bytes) = Translator::new_with_alloc_size(image_size, 32);
        let buffer = Buffer::new(size_in_bytes);
        {
            let mut mapping = buffer.mmap();
            for y in 0..image_size.y {
                for x in 0..image_size.x {
                    let offset = translator
                        .coordinate_to_tile_address(UVec2::new(x, y))
                        .offset as usize;
                    mapping.as_mut()[offset] = 0;
                    mapping.as_mut()[offset + 1] = y as u8;
                    mapping.as_mut()[offset + 2] = x as u8;
                    mapping.as_mut()[offset + 3] = 255;
                }
            }
        }
        buffer
    };

    let vbo = Buffer::new(24);

    let mut command_recorder = CommandRecorder::new(display_framebuffers.size);
    command_recorder.begin_pass();
    shaders::test_triangle::bind(&mut command_recorder, vbo.clone(), tex_bo.clone());
    command_recorder.draw_array_primitives(PrimitiveMode::Triangles, 0, 3);
    command_recorder.end_pass();

    display_framebuffers.set_crtc(0);

    let mut wait_usec: i64 = 0;

    let mut i = 0;
    loop {
        let framebuffer = &display_framebuffers.framebuffers[i & 1];

        use vc4_drm::tokio::time::Duration;
        if wait_usec > 0 {
            //tokio::time::sleep(Duration::from_micros(wait_usec as u64)).await;
        }

        {
            let mut vbo_map = vbo.mmap();
            let mut cur = Cursor::new(vbo_map.as_mut());

            let side_len = 3.0 / f32::sqrt(3.0) / 2.0;

            for j in 0..1 {
                let rot_mat = vc4_drm::glam::Mat2::from_angle(
                    (i + j) as f32 * 2.0 * std::f32::consts::PI / 128.0,
                );

                let v0 = rot_mat.mul_vec2([-side_len, 0.5].into());
                cur.write_all(&f32::from(v0.x).to_le_bytes()).unwrap();
                cur.write_all(&f32::from(v0.y).to_le_bytes()).unwrap();

                let v1 = rot_mat.mul_vec2([side_len, 0.5].into());
                cur.write_all(&f32::from(v1.x).to_le_bytes()).unwrap();
                cur.write_all(&f32::from(v1.y).to_le_bytes()).unwrap();

                let v2 = rot_mat.mul_vec2([0.0, -1.0].into());
                cur.write_all(&f32::from(v2.x).to_le_bytes()).unwrap();
                cur.write_all(&f32::from(v2.y).to_le_bytes()).unwrap();
            }
        }

        let clear_color = 0xffffffff;
        command_recorder
            .submit(clear_color, framebuffer.bo.clone())
            .await;

        use vc4_drm::tokio::time::Instant;
        let start = Instant::now();
        display_framebuffers.page_flip(i & 1).await;
        let flip_wait = Instant::now() - start;
        let delta = flip_wait.as_micros() as i64 - Duration::from_millis(2).as_micros() as i64;
        let new_wait_usec = wait_usec + delta;
        if new_wait_usec < 1000000 / 60 - 2000 {
            wait_usec = new_wait_usec;
        } else if new_wait_usec > 1000000 / 60 - 1000 {
            wait_usec -= 1000;
        }
        //println!("{}us will wait {}us", flip_wait.as_micros(), wait_usec);

        i += 1;
    }
}

fn main() {
    use std::os::unix::process::CommandExt;
    let current_exe = std::env::current_exe().unwrap();
    if let Some(_) = std::env::args_os().find(|a| a == "--debugserver") {
        std::process::Command::new("gdbserver")
            .args([":1235", current_exe.to_str().unwrap()])
            .exec();
    }

    vc4_drm::tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async_main())
}

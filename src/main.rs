#![recursion_limit = "10000"]
mod shaders;

use num_traits::float::FloatConst;
use rpi_drm::{Buffer, CommandEncoder};
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use vc4_drm::cl::IndexType;
use vc4_drm::{cl::PrimitiveMode, glam, glam::UVec2};

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

    let vbo_vs = Buffer::new(48);
    let ibo = Buffer::new(6);
    {
        let mut ibo_map = ibo.mmap();
        let mut cur = Cursor::new(ibo_map.as_mut());
        cur.write_all(&0_u16.to_le_bytes()).unwrap();
        cur.write_all(&1_u16.to_le_bytes()).unwrap();
        cur.write_all(&2_u16.to_le_bytes()).unwrap();
    }

    let mut imu_handle = {
        let mut options = std::fs::OpenOptions::new();
        options.read(true);
        options
            .open("/sys/bus/iio/devices/iio:device0/in_rot_quaternion_raw")
            .unwrap()
    };
    let mut read_quaternion = || {
        let mut quaternion_buf: [u8; 64] = [0; 64];
        imu_handle.seek(SeekFrom::Start(0)).unwrap();
        let read_num = imu_handle.read(&mut quaternion_buf).unwrap();
        let quaternion_string = std::str::from_utf8(&quaternion_buf[0..read_num]).unwrap();
        let mut quaterion_arr: [f32; 4] = [0.0; 4];
        for (i, comp) in quaternion_string.split(' ').enumerate() {
            use std::str::FromStr;
            quaterion_arr[(i + 3) % 4] = f32::from_str(comp).unwrap() / 16383.0;
            if i == 3 {
                break;
            }
        }
        vc4_drm::glam::Quat::from_array(quaterion_arr)
    };

    display_framebuffers.set_crtc(0);
    let mut command_encoder = CommandEncoder::new(display_framebuffers.size);

    let mut wait_usec: i64 = 0;

    let mut i = 0;
    loop {
        let framebuffer = &display_framebuffers.framebuffers[i & 1];

        use vc4_drm::tokio::time::Duration;
        if wait_usec > 0 {
            //tokio::time::sleep(Duration::from_micros(wait_usec as u64)).await;
        }
        //vc4_drm::tokio::time::sleep(Duration::from_millis(1000u64)).await;

        let quaternion = read_quaternion();
        //println!("{}", quaternion);
        let xf = glam::Mat4::from_quat(quaternion.inverse());
        let xf2 = glam::Mat4::from_scale_rotation_translation(
            glam::Vec3::new(0.25, -0.25, -0.25),
            glam::Quat::IDENTITY,
            glam::Vec3::new(0.0, 0.0, 0.5),
        );
        let xf_persp = glam::Mat4::perspective_lh(60.0 * f32::PI() / 180.0, 1.0, 0.1, 1.0);
        let xf_total = xf_persp * xf2 * xf;
        {
            let mut vbo_map = vbo_vs.mmap();
            let mut cur = Cursor::new(vbo_map.as_mut());

            let side_len = 3.0 / f32::sqrt(3.0) / 2.0;

            let v0 = xf_total.mul_vec4([-side_len, 0.5, 0.0, 1.0].into());
            //println!("{}", v0);
            cur.write_all(&v0.x.to_le_bytes()).unwrap();
            cur.write_all(&v0.y.to_le_bytes()).unwrap();
            cur.write_all(&v0.z.to_le_bytes()).unwrap();
            cur.write_all(&v0.w.to_le_bytes()).unwrap();

            let v1 = xf_total.mul_vec4([side_len, 0.5, 0.0, 1.0].into());
            cur.write_all(&v1.x.to_le_bytes()).unwrap();
            cur.write_all(&v1.y.to_le_bytes()).unwrap();
            cur.write_all(&v1.z.to_le_bytes()).unwrap();
            cur.write_all(&v1.w.to_le_bytes()).unwrap();

            let v2 = xf_total.mul_vec4([0.0, -1.0, 0.0, 1.0].into());
            cur.write_all(&v2.x.to_le_bytes()).unwrap();
            cur.write_all(&v2.y.to_le_bytes()).unwrap();
            cur.write_all(&v2.z.to_le_bytes()).unwrap();
            cur.write_all(&v2.w.to_le_bytes()).unwrap();
        }

        /*
        let xf = glam::Mat4::from_quat(quaternion);
        //let xf = glam::Mat4::from_quat(glam::Quat::IDENTITY);
        let xf2 = glam::Mat4::from_scale_rotation_translation(
            glam::Vec3::new(1.0, 1.0, 0.5),
            glam::Quat::IDENTITY,
            glam::Vec3::new(0.0, 0.0, 0.5),
        );
        let xf3 = glam::Mat4::perspective_lh(60.0 * f32::PI() / 180.0, 1.0, 0.0, 1.0);
        */

        command_encoder.clear();
        command_encoder.begin_pass();
        shaders::test_triangle::bind(&mut command_encoder, vbo_vs.clone());
        command_encoder.draw_array_primitives(PrimitiveMode::Triangles, 0, 3);
        //shaders::test_model::draw(&mut command_encoder, xf3 * xf);
        command_encoder.end_pass();

        let clear_color = 0xff000000; // ARGB
        command_encoder
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

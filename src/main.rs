#![recursion_limit = "10000"]
mod shaders;
use shaders::test_model;

use num_traits::float::FloatConst;
use rpi_drm::CommandEncoder;
use std::io::{Read, Seek, SeekFrom};
use vc4_drm::glam::*;

async fn async_main() {
    shaders::initialize_shaders().await;

    let display_framebuffers = rpi_drm::open_and_allocate_display_framebuffers();

    let mut imu_handle = {
        let mut options = std::fs::OpenOptions::new();
        options.read(true);
        options
            .open("/sys/bus/iio/devices/iio:device0/in_rot_quaternion_raw")
            .unwrap()
    };
    let mut read_quaternion = || {
        let mut quaternion_buf: [u8; 64] = [0; 64];
        if imu_handle.seek(SeekFrom::Start(0)).is_err() {
            return Quat::IDENTITY;
        }
        let read_num = imu_handle.read(&mut quaternion_buf);
        if read_num.is_err() {
            return Quat::IDENTITY;
        }
        let quaternion_string = std::str::from_utf8(&quaternion_buf[0..read_num.unwrap()]).unwrap();
        let mut quaterion_arr: [f32; 4] = [0.0; 4];
        for (i, comp) in quaternion_string.split(' ').enumerate() {
            use std::str::FromStr;
            quaterion_arr[(i + 3) % 4] = f32::from_str(comp).unwrap() / 16383.0;
            if i == 3 {
                break;
            }
        }
        Quat::from_array(quaterion_arr)
    };

    display_framebuffers.set_crtc(0);
    let mut command_encoder = CommandEncoder::new(display_framebuffers.size);

    let mut wait_usec: i64 = 0;

    use vc4_drm::tokio::time::Instant;
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
        let xf = Mat4::from_quat(quaternion.inverse());
        let xf2 = Mat4::from_scale_rotation_translation(
            Vec3::new(0.25, -0.25, -0.25),
            Quat::IDENTITY,
            Vec3::new(0.0, 0.0, 0.5),
        );
        let xf_persp = Mat4::perspective_lh(60.0 * f32::PI() / 180.0, 1.0, 0.1, 1.0);
        let xf_total = xf_persp * xf2 * xf;
        let xf_total2 = xf_total * Mat4::from_axis_angle(Vec3::Z, 180.0 * f32::PI() / 180.0);
        let xf_total3 = xf_total * Mat4::from_axis_angle(Vec3::Z, 90.0 * f32::PI() / 180.0);
        let xf_total4 = xf_total * Mat4::from_axis_angle(Vec3::Z, -90.0 * f32::PI() / 180.0);

        command_encoder.clear();
        command_encoder.begin_pass();
        test_model::draw(&mut command_encoder, &xf_total);
        test_model::draw(&mut command_encoder, &xf_total2);
        test_model::draw(&mut command_encoder, &xf_total3);
        test_model::draw(&mut command_encoder, &xf_total4);
        command_encoder.end_pass();

        // A8R8G8B8
        let clear_color = 0xff000000;
        // Z24X8
        let clear_z = f64::round(f64::clamp(1.0, 0.0, 1.0) * (0xffffff as f64)) as u32;
        let render_start = Instant::now();
        command_encoder
            .submit(
                clear_color,
                clear_z,
                &framebuffer.bo,
                &display_framebuffers.z_buffer,
            )
            .await;
        let render_dur = Instant::now() - render_start;

        let flip_start = Instant::now();
        display_framebuffers.page_flip(i & 1).await;
        let flip_dur = Instant::now() - flip_start;
        let delta = flip_dur.as_micros() as i64 - Duration::from_millis(2).as_micros() as i64;
        let new_wait_usec = wait_usec + delta;
        if new_wait_usec < 1000000 / 60 - 2000 {
            wait_usec = new_wait_usec;
        } else if new_wait_usec > 1000000 / 60 - 1000 {
            wait_usec -= 1000;
        }
        //println!("{}us will wait {}us", flip_wait.as_micros(), wait_usec);
        //println!("Render: {}us, Flip: {}us", render_dur.as_micros(), flip_dur.as_micros());

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

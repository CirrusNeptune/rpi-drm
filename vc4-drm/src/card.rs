#![allow(dead_code)]

#[derive(Default, Debug, Copy, Clone)]
#[repr(u8)]
pub enum VC4RenderConfigFormat {
    BGR565Dithered = 0,
    #[default]
    RGBA8888 = 1,
    BGR565 = 2,
}

#[derive(Default, Debug, Copy, Clone)]
#[repr(u8)]
pub enum VC4TilingFormat {
    Linear = 0,
    #[default]
    T = 1,
    LT = 2,
}

mod ffi {
    #![allow(nonstandard_style)]

    use drm_ffi::result::SystemError;
    use std::mem::transmute;
    use std::os::fd::RawFd;

    pub type __u8 = libc::c_uchar;
    pub type __u16 = libc::c_ushort;
    pub type __u32 = libc::c_uint;
    pub type __u64 = libc::c_ulonglong;

    #[repr(C)]
    #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
    pub struct drm_vc4_submit_rcl_surface {
        pub hindex: __u32,
        pub offset: __u32,
        pub bits: __u16,
        pub flags: __u16,
    }

    impl Default for drm_vc4_submit_rcl_surface {
        fn default() -> Self {
            Self {
                hindex: u32::MAX,
                offset: 0,
                bits: 0,
                flags: 0,
            }
        }
    }

    use super::{VC4RenderConfigFormat, VC4TilingFormat};

    impl drm_vc4_submit_rcl_surface {
        pub fn new_tiled_rgba8(hindex: u32) -> Self {
            Self {
                hindex: hindex,
                offset: 0,
                bits: 0,
                flags: 0,
            }
            .format(VC4RenderConfigFormat::RGBA8888)
            .tiling(VC4TilingFormat::T)
        }

        pub fn format(mut self, format: VC4RenderConfigFormat) -> Self {
            self.bits &= !(0x3 << 2);
            self.bits |= (format as u16) << 2;
            self
        }

        pub fn tiling(mut self, tiling: VC4TilingFormat) -> Self {
            self.bits &= !(0x3 << 6);
            self.bits |= (tiling as u16) << 6;
            self
        }
    }

    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
    pub struct drm_vc4_submit_cl {
        pub bin_cl: __u64,
        pub shader_rec: __u64,
        pub uniforms: __u64,
        pub bo_handles: __u64,
        pub bin_cl_size: __u32,
        pub shader_rec_size: __u32,
        pub shader_rec_count: __u32,
        pub uniforms_size: __u32,
        pub bo_handle_count: __u32,
        pub width: __u16,
        pub height: __u16,
        pub min_x_tile: __u8,
        pub min_y_tile: __u8,
        pub max_x_tile: __u8,
        pub max_y_tile: __u8,
        pub color_read: drm_vc4_submit_rcl_surface,
        pub color_write: drm_vc4_submit_rcl_surface,
        pub zs_read: drm_vc4_submit_rcl_surface,
        pub zs_write: drm_vc4_submit_rcl_surface,
        pub msaa_color_write: drm_vc4_submit_rcl_surface,
        pub msaa_zs_write: drm_vc4_submit_rcl_surface,
        pub clear_color: [__u32; 2],
        pub clear_z: __u32,
        pub clear_s: __u8,
        pub flags: __u32,
        pub seqno: __u64,
        pub perfmonid: __u32,
        pub in_sync: __u32,
        pub out_sync: __u32,
        pub pad2: __u32,
    }

    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
    pub struct drm_vc4_wait_seqno {
        pub seqno: __u64,
        pub timeout_ns: __u64,
    }

    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
    pub struct drm_vc4_wait_bo {
        pub handle: __u32,
        pub pad: __u32,
        pub timeout_ns: __u64,
    }

    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
    pub struct drm_vc4_create_bo {
        pub size: __u32,
        pub flags: __u32,
        pub handle: __u32,
        pub pad: __u32,
    }

    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
    pub struct drm_vc4_mmap_bo {
        pub handle: __u32,
        pub flags: __u32,
        pub offset: __u64,
    }

    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
    pub struct drm_vc4_create_shader_bo {
        pub size: __u32,
        pub flags: __u32,
        pub data: __u64,
        pub handle: __u32,
        pub pad: __u32,
    }

    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
    pub struct drm_vc4_get_hang_state_bo {
        pub handle: __u32,
        pub paddr: __u32,
        pub size: __u32,
        pub pad: __u32,
    }

    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
    pub struct drm_vc4_get_hang_state {
        pub bo: __u64,
        pub bo_count: __u32,
        pub start_bin: __u32,
        pub start_render: __u32,
        pub ct0ca: __u32,
        pub ct0ea: __u32,
        pub ct1ca: __u32,
        pub ct1ea: __u32,
        pub ct0cs: __u32,
        pub ct1cs: __u32,
        pub ct0ra0: __u32,
        pub ct1ra0: __u32,
        pub bpca: __u32,
        pub bpcs: __u32,
        pub bpoa: __u32,
        pub bpos: __u32,
        pub vpmbase: __u32,
        pub dbge: __u32,
        pub fdbgo: __u32,
        pub fdbgb: __u32,
        pub fdbgr: __u32,
        pub fdbgs: __u32,
        pub errstat: __u32,
        pub pad: [__u32; 16],
    }

    #[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
    pub struct drm_vc4_get_hang_state_reply {
        pub bo: Vec<drm_vc4_get_hang_state_bo>,
        pub start_bin: __u32,
        pub start_render: __u32,
        pub ct0ca: __u32,
        pub ct0ea: __u32,
        pub ct1ca: __u32,
        pub ct1ea: __u32,
        pub ct0cs: __u32,
        pub ct1cs: __u32,
        pub ct0ra0: __u32,
        pub ct1ra0: __u32,
        pub bpca: __u32,
        pub bpcs: __u32,
        pub bpoa: __u32,
        pub bpos: __u32,
        pub vpmbase: __u32,
        pub dbge: __u32,
        pub fdbgo: __u32,
        pub fdbgb: __u32,
        pub fdbgr: __u32,
        pub fdbgs: __u32,
        pub errstat: __u32,
    }

    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
    pub struct drm_vc4_get_param {
        pub param: __u32,
        pub pad: __u32,
        pub value: __u64,
    }

    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
    pub struct drm_vc4_get_tiling {
        pub handle: __u32,
        pub flags: __u32,
        pub modifier: __u64,
    }

    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
    pub struct drm_vc4_set_tiling {
        pub handle: __u32,
        pub flags: __u32,
        pub modifier: __u64,
    }

    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
    pub struct drm_vc4_label_bo {
        pub handle: __u32,
        pub len: __u32,
        pub name: __u64,
    }

    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
    pub struct drm_vc4_gem_madvise {
        pub handle: __u32,
        pub madv: __u32,
        pub retained: __u32,
        pub pad: __u32,
    }

    pub const DRM_VC4_MAX_PERF_COUNTERS: usize = 16;

    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
    pub struct drm_vc4_perfmon_create {
        pub id: __u32,
        pub ncounters: __u32,
        pub events: [__u8; DRM_VC4_MAX_PERF_COUNTERS],
    }

    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
    pub struct drm_vc4_perfmon_destroy {
        pub id: __u32,
    }

    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
    pub struct drm_vc4_perfmon_get_values {
        pub id: __u32,
        pub values_ptr: __u64,
    }

    mod ioctl {
        use super::*;
        use nix::ioctl_readwrite;
        ioctl_readwrite!(
            vc4_submit_cl,
            drm_sys::DRM_IOCTL_BASE,
            drm_sys::DRM_COMMAND_BASE + 0x0,
            drm_vc4_submit_cl
        );
        ioctl_readwrite!(
            vc4_wait_seqno,
            drm_sys::DRM_IOCTL_BASE,
            drm_sys::DRM_COMMAND_BASE + 0x1,
            drm_vc4_wait_seqno
        );
        ioctl_readwrite!(
            vc4_wait_bo,
            drm_sys::DRM_IOCTL_BASE,
            drm_sys::DRM_COMMAND_BASE + 0x2,
            drm_vc4_wait_bo
        );
        ioctl_readwrite!(
            vc4_create_bo,
            drm_sys::DRM_IOCTL_BASE,
            drm_sys::DRM_COMMAND_BASE + 0x3,
            drm_vc4_create_bo
        );
        ioctl_readwrite!(
            vc4_mmap_bo,
            drm_sys::DRM_IOCTL_BASE,
            drm_sys::DRM_COMMAND_BASE + 0x4,
            drm_vc4_mmap_bo
        );
        ioctl_readwrite!(
            vc4_create_shader_bo,
            drm_sys::DRM_IOCTL_BASE,
            drm_sys::DRM_COMMAND_BASE + 0x5,
            drm_vc4_create_shader_bo
        );
        ioctl_readwrite!(
            vc4_get_hang_state,
            drm_sys::DRM_IOCTL_BASE,
            drm_sys::DRM_COMMAND_BASE + 0x6,
            drm_vc4_get_hang_state
        );
        ioctl_readwrite!(
            vc4_get_param,
            drm_sys::DRM_IOCTL_BASE,
            drm_sys::DRM_COMMAND_BASE + 0x7,
            drm_vc4_get_param
        );
        ioctl_readwrite!(
            vc4_set_tiling,
            drm_sys::DRM_IOCTL_BASE,
            drm_sys::DRM_COMMAND_BASE + 0x8,
            drm_vc4_set_tiling
        );
        ioctl_readwrite!(
            vc4_get_tiling,
            drm_sys::DRM_IOCTL_BASE,
            drm_sys::DRM_COMMAND_BASE + 0x9,
            drm_vc4_get_tiling
        );
        ioctl_readwrite!(
            vc4_label_bo,
            drm_sys::DRM_IOCTL_BASE,
            drm_sys::DRM_COMMAND_BASE + 0xa,
            drm_vc4_label_bo
        );
        ioctl_readwrite!(
            vc4_gem_madvise,
            drm_sys::DRM_IOCTL_BASE,
            drm_sys::DRM_COMMAND_BASE + 0xb,
            drm_vc4_gem_madvise
        );
        ioctl_readwrite!(
            vc4_perfmon_create,
            drm_sys::DRM_IOCTL_BASE,
            drm_sys::DRM_COMMAND_BASE + 0xc,
            drm_vc4_perfmon_create
        );
        ioctl_readwrite!(
            vc4_perfmon_destroy,
            drm_sys::DRM_IOCTL_BASE,
            drm_sys::DRM_COMMAND_BASE + 0xd,
            drm_vc4_perfmon_destroy
        );
        ioctl_readwrite!(
            vc4_perfmon_get_values,
            drm_sys::DRM_IOCTL_BASE,
            drm_sys::DRM_COMMAND_BASE + 0xe,
            drm_vc4_perfmon_get_values
        );

        pub mod syncobj {
            use drm_sys::*;
            use nix::ioctl_readwrite;
            ioctl_readwrite!(create, DRM_IOCTL_BASE, 0xBF, drm_syncobj_create);
            ioctl_readwrite!(destroy, DRM_IOCTL_BASE, 0xC0, drm_syncobj_destroy);
            ioctl_readwrite!(handle_to_fd, DRM_IOCTL_BASE, 0xC1, drm_syncobj_handle);
            ioctl_readwrite!(fd_to_handle, DRM_IOCTL_BASE, 0xC2, drm_syncobj_handle);
            ioctl_readwrite!(wait, DRM_IOCTL_BASE, 0xC3, drm_syncobj_wait);
            ioctl_readwrite!(reset, DRM_IOCTL_BASE, 0xC4, drm_syncobj_array);
            ioctl_readwrite!(signal, DRM_IOCTL_BASE, 0xC5, drm_syncobj_array);
        }
    }

    use drm::buffer::Handle;
    use syncobj::SyncObjHandle;

    pub fn vc4_submit_cl(
        fd: RawFd,
        bin_cl: &[u8],
        shader_rec: &[u8],
        uniforms: &[u32],
        bo_handles: &[Handle],
        shader_rec_count: u32,
        width: u16,
        height: u16,
        min_x_tile: u8,
        min_y_tile: u8,
        max_x_tile: u8,
        max_y_tile: u8,
        color_read: drm_vc4_submit_rcl_surface,
        color_write: drm_vc4_submit_rcl_surface,
        zs_read: drm_vc4_submit_rcl_surface,
        zs_write: drm_vc4_submit_rcl_surface,
        msaa_color_write: drm_vc4_submit_rcl_surface,
        msaa_zs_write: drm_vc4_submit_rcl_surface,
        clear_color: [u32; 2],
        clear_z: u32,
        clear_s: u8,
        flags: u32,
        in_sync: Option<SyncObjHandle>,
        out_sync: Option<SyncObjHandle>,
    ) -> Result<u64, SystemError> {
        unsafe {
            let mut args = drm_vc4_submit_cl {
                bin_cl: transmute(bin_cl.as_ptr()),
                shader_rec: transmute(shader_rec.as_ptr()),
                uniforms: transmute(uniforms.as_ptr()),
                bo_handles: transmute(bo_handles.as_ptr()),
                bin_cl_size: bin_cl.len() as __u32,
                shader_rec_size: shader_rec.len() as __u32,
                shader_rec_count,
                uniforms_size: (uniforms.len() * 4) as __u32,
                bo_handle_count: bo_handles.len() as __u32,
                width,
                height,
                min_x_tile,
                min_y_tile,
                max_x_tile,
                max_y_tile,
                color_read,
                color_write,
                zs_read,
                zs_write,
                msaa_color_write,
                msaa_zs_write,
                clear_color,
                clear_z,
                clear_s,
                flags,
                seqno: 0,
                perfmonid: 0,
                in_sync: if let Some(handle) = in_sync {
                    handle.into()
                } else {
                    0
                },
                out_sync: if let Some(handle) = out_sync {
                    handle.into()
                } else {
                    0
                },
                pad2: 0,
            };

            ioctl::vc4_submit_cl(fd, &mut args)?;

            Ok(args.seqno)
        }
    }

    pub fn vc4_wait_seqno(fd: RawFd, seqno: u64, timeout_ns: u64) -> Result<u64, SystemError> {
        unsafe {
            let mut args = drm_vc4_wait_seqno { seqno, timeout_ns };

            ioctl::vc4_wait_seqno(fd, &mut args)?;

            Ok(args.timeout_ns)
        }
    }

    pub fn vc4_wait_bo(fd: RawFd, handle: Handle, timeout_ns: u64) -> Result<u64, SystemError> {
        unsafe {
            let mut args = drm_vc4_wait_bo {
                handle: handle.into(),
                pad: 0,
                timeout_ns,
            };

            ioctl::vc4_wait_bo(fd, &mut args)?;

            Ok(args.timeout_ns)
        }
    }

    pub fn vc4_create_bo(fd: RawFd, size: u32, flags: u32) -> Result<Handle, SystemError> {
        unsafe {
            let mut args = drm_vc4_create_bo {
                size,
                flags,
                handle: 0,
                pad: 0,
            };

            ioctl::vc4_create_bo(fd, &mut args)?;

            Ok(core::num::NonZeroU32::new_unchecked(args.handle).into())
        }
    }

    pub fn vc4_mmap_bo(fd: RawFd, handle: Handle, flags: u32) -> Result<u64, SystemError> {
        unsafe {
            let mut args = drm_vc4_mmap_bo {
                handle: handle.into(),
                flags,
                offset: 0,
            };

            ioctl::vc4_mmap_bo(fd, &mut args)?;

            Ok(args.offset)
        }
    }

    pub fn vc4_create_shader_bo(
        fd: RawFd,
        flags: u32,
        data: &[u64],
    ) -> Result<Handle, SystemError> {
        unsafe {
            let mut args = drm_vc4_create_shader_bo {
                size: (data.len() * 8) as __u32,
                flags,
                data: transmute(data.as_ptr()),
                handle: 0,
                pad: 0,
            };

            ioctl::vc4_create_shader_bo(fd, &mut args)?;

            Ok(core::num::NonZeroU32::new_unchecked(args.handle).into())
        }
    }

    pub fn vc4_get_hang_state(
        fd: RawFd,
    ) -> Result<Option<drm_vc4_get_hang_state_reply>, SystemError> {
        unsafe {
            let mut args = drm_vc4_get_hang_state::default();

            // Calling with `bo_count = 0` will early-return with the current bo_count set.
            ioctl::vc4_get_hang_state(fd, &mut args)?;

            if args.bo_count > 0 {
                // There's a chance the bo_count will grow for the next ioctl.
                // Loop until we have a result that fits.
                loop {
                    let mut bo = vec![drm_vc4_get_hang_state_bo::default(); args.bo_count as usize];
                    args.bo = transmute(bo.as_ptr());

                    // Detect unexpected growth while running the ioctl.
                    // The kernel does not populate the structure in this case.
                    let last_bo_count = args.bo_count;
                    ioctl::vc4_get_hang_state(fd, &mut args)?;
                    if args.bo_count > last_bo_count {
                        continue;
                    }

                    // Truncate to the actual bo_count in case it shrank between ioctls.
                    bo.truncate(args.bo_count as usize);

                    // Structure successfully populated if we get here
                    break Ok(Some(drm_vc4_get_hang_state_reply {
                        bo,
                        start_bin: args.start_bin,
                        start_render: args.start_render,
                        ct0ca: args.ct0ca,
                        ct0ea: args.ct0ea,
                        ct1ca: args.ct1ca,
                        ct1ea: args.ct1ea,
                        ct0cs: args.ct0cs,
                        ct1cs: args.ct1cs,
                        ct0ra0: args.ct0ra0,
                        ct1ra0: args.ct1ra0,
                        bpca: args.bpca,
                        bpcs: args.bpcs,
                        bpoa: args.bpoa,
                        bpos: args.bpos,
                        vpmbase: args.vpmbase,
                        dbge: args.dbge,
                        fdbgo: args.fdbgo,
                        fdbgb: args.fdbgb,
                        fdbgr: args.fdbgr,
                        fdbgs: args.fdbgs,
                        errstat: args.errstat,
                    }));
                }
            } else {
                Ok(None)
            }
        }
    }

    pub fn vc4_get_param(fd: RawFd, param: u32) -> Result<u64, SystemError> {
        unsafe {
            let mut args = drm_vc4_get_param {
                param,
                pad: 0,
                value: 0,
            };

            ioctl::vc4_get_param(fd, &mut args)?;

            Ok(args.value)
        }
    }

    pub fn vc4_get_tiling(
        fd: RawFd,
        handle: Handle,
        flags: u32,
        modifier: u64,
    ) -> Result<u64, SystemError> {
        unsafe {
            let mut args = drm_vc4_get_tiling {
                handle: handle.into(),
                flags,
                modifier,
            };

            ioctl::vc4_get_tiling(fd, &mut args)?;

            Ok(args.modifier)
        }
    }

    pub fn vc4_set_tiling(
        fd: RawFd,
        handle: Handle,
        flags: u32,
        modifier: u64,
    ) -> Result<(), SystemError> {
        unsafe {
            let mut args = drm_vc4_set_tiling {
                handle: handle.into(),
                flags,
                modifier,
            };

            ioctl::vc4_set_tiling(fd, &mut args)?;

            Ok(())
        }
    }

    pub fn vc4_label_bo(fd: RawFd, handle: Handle, name: &str) -> Result<(), SystemError> {
        unsafe {
            let name_c_str =
                std::ffi::CString::new(name).map_err(|_| SystemError::InvalidArgument)?;

            let mut args = drm_vc4_label_bo {
                handle: handle.into(),
                len: name.len() as __u32,
                name: transmute(name_c_str.as_ptr()),
            };

            ioctl::vc4_label_bo(fd, &mut args)?;

            Ok(())
        }
    }

    pub fn vc4_gem_madvise(fd: RawFd, handle: Handle, madv: u32) -> Result<u32, SystemError> {
        unsafe {
            let mut args = drm_vc4_gem_madvise {
                handle: handle.into(),
                madv,
                retained: 0,
                pad: 0,
            };

            ioctl::vc4_gem_madvise(fd, &mut args)?;

            Ok(args.retained)
        }
    }

    pub fn vc4_perfmon_create(fd: RawFd, events: &[u8]) -> Result<u32, SystemError> {
        if events.len() <= DRM_VC4_MAX_PERF_COUNTERS {
            unsafe {
                let mut events_arr = [0; DRM_VC4_MAX_PERF_COUNTERS];
                events_arr.copy_from_slice(events);

                let mut args = drm_vc4_perfmon_create {
                    id: 0,
                    ncounters: events.len() as u32,
                    events: events_arr,
                };

                ioctl::vc4_perfmon_create(fd, &mut args)?;

                Ok(args.id)
            }
        } else {
            Err(SystemError::InvalidArgument)
        }
    }

    pub fn vc4_perfmon_destroy(fd: RawFd, id: u32) -> Result<(), SystemError> {
        unsafe {
            let mut args = drm_vc4_perfmon_destroy { id };

            ioctl::vc4_perfmon_destroy(fd, &mut args)?;

            Ok(())
        }
    }

    pub fn vc4_perfmon_get_values(
        fd: RawFd,
        id: u32,
    ) -> Result<[u64; DRM_VC4_MAX_PERF_COUNTERS], SystemError> {
        unsafe {
            let mut values_arr: [u64; DRM_VC4_MAX_PERF_COUNTERS] = [0; DRM_VC4_MAX_PERF_COUNTERS];

            let mut args = drm_vc4_perfmon_get_values {
                id,
                values_ptr: transmute(values_arr.as_mut_ptr()),
            };

            ioctl::vc4_perfmon_get_values(fd, &mut args)?;

            Ok(values_arr)
        }
    }

    pub(crate) mod syncobj {
        #![allow(dead_code)]
        use super::*;
        use drm::*;
        use drm_sys::*;
        use std::os::fd::{AsFd, AsRawFd};

        #[repr(transparent)]
        #[derive(Copy, Clone, Hash, PartialEq, Eq)]
        pub struct SyncObjHandle(control::RawResourceHandle);

        impl From<SyncObjHandle> for control::RawResourceHandle {
            fn from(handle: SyncObjHandle) -> Self {
                handle.0
            }
        }

        impl From<SyncObjHandle> for u32 {
            fn from(handle: SyncObjHandle) -> Self {
                handle.0.into()
            }
        }

        impl From<control::RawResourceHandle> for SyncObjHandle {
            fn from(handle: control::RawResourceHandle) -> Self {
                SyncObjHandle(handle)
            }
        }

        impl std::fmt::Debug for SyncObjHandle {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.debug_tuple("syncobj::SyncObjHandle")
                    .field(&self.0)
                    .finish()
            }
        }

        #[derive(Debug)]
        /// A simple wrapper for a syncobj node.
        pub struct SyncObj(std::fs::File);

        /// Implementing `AsFd` is a prerequisite to implementing the traits found
        /// in this crate. Here, we are just calling `as_fd()` on the inner File.
        impl AsFd for SyncObj {
            fn as_fd(&self) -> std::os::unix::io::BorrowedFd<'_> {
                self.0.as_fd()
            }
        }

        impl AsRawFd for SyncObj {
            fn as_raw_fd(&self) -> RawFd {
                self.as_fd().as_raw_fd()
            }
        }

        pub fn create(fd: RawFd, signaled: bool) -> Result<SyncObjHandle, SystemError> {
            unsafe {
                let mut args = drm_syncobj_create {
                    handle: 0,
                    flags: if signaled {
                        DRM_SYNCOBJ_CREATE_SIGNALED
                    } else {
                        0
                    },
                };

                ioctl::syncobj::create(fd, &mut args)?;

                Ok(core::num::NonZeroU32::new_unchecked(args.handle).into())
            }
        }

        pub fn destroy(fd: RawFd, handle: SyncObjHandle) -> Result<(), SystemError> {
            unsafe {
                let mut args = drm_syncobj_destroy {
                    handle: handle.into(),
                    pad: 0,
                };

                ioctl::syncobj::destroy(fd, &mut args)?;

                Ok(())
            }
        }

        pub fn handle_to_fd(
            fd: RawFd,
            handle: SyncObjHandle,
            export_sync_file: bool,
        ) -> Result<SyncObj, SystemError> {
            unsafe {
                let mut args = drm_syncobj_handle {
                    handle: handle.into(),
                    flags: if export_sync_file {
                        DRM_SYNCOBJ_HANDLE_TO_FD_FLAGS_EXPORT_SYNC_FILE
                    } else {
                        0
                    },
                    fd: 0,
                    pad: 0,
                };

                ioctl::syncobj::handle_to_fd(fd, &mut args)?;

                use std::os::fd::FromRawFd;
                Ok(SyncObj(std::fs::File::from_raw_fd(args.fd)))
            }
        }

        pub fn fd_to_handle(
            fd: RawFd,
            syncobj: &SyncObj,
            import_sync_file: bool,
        ) -> Result<SyncObjHandle, SystemError> {
            unsafe {
                let mut args = drm_syncobj_handle {
                    handle: 0,
                    flags: if import_sync_file {
                        DRM_SYNCOBJ_FD_TO_HANDLE_FLAGS_IMPORT_SYNC_FILE
                    } else {
                        0
                    },
                    fd: syncobj.as_fd().as_raw_fd(),
                    pad: 0,
                };

                ioctl::syncobj::fd_to_handle(fd, &mut args)?;

                Ok(core::num::NonZeroU32::new_unchecked(args.handle).into())
            }
        }

        pub fn wait(
            fd: RawFd,
            handles: &[SyncObjHandle],
            timeout_nsec: i64,
            wait_all: bool,
            wait_for_submit: bool,
        ) -> Result<Option<SyncObjHandle>, SystemError> {
            unsafe {
                let mut args = drm_syncobj_wait {
                    handles: transmute(handles.as_ptr()),
                    timeout_nsec,
                    count_handles: handles.len() as u32,
                    flags: if wait_all {
                        DRM_SYNCOBJ_WAIT_FLAGS_WAIT_ALL
                    } else {
                        0
                    } | if wait_for_submit {
                        DRM_SYNCOBJ_WAIT_FLAGS_WAIT_FOR_SUBMIT
                    } else {
                        0
                    },
                    first_signaled: 0,
                    pad: 0,
                };

                ioctl::syncobj::wait(fd, &mut args)?;

                Ok(if args.first_signaled != 0 {
                    Some(core::num::NonZeroU32::new_unchecked(args.first_signaled).into())
                } else {
                    None
                })
            }
        }

        pub fn reset(fd: RawFd, handles: &[SyncObjHandle]) -> Result<(), SystemError> {
            unsafe {
                let mut args = drm_syncobj_array {
                    handles: transmute(handles.as_ptr()),
                    count_handles: handles.len() as u32,
                    pad: 0,
                };

                ioctl::syncobj::reset(fd, &mut args)?;

                Ok(())
            }
        }

        pub fn signal(fd: RawFd, handles: &[SyncObjHandle]) -> Result<(), SystemError> {
            unsafe {
                let mut args = drm_syncobj_array {
                    handles: transmute(handles.as_ptr()),
                    count_handles: handles.len() as u32,
                    pad: 0,
                };

                ioctl::syncobj::signal(fd, &mut args)?;

                Ok(())
            }
        }
    }
}

use drm::{buffer::Handle, control::Device as ControlDevice, Device};
pub use drm_ffi::result::SystemError;
use drm_fourcc::DrmFourcc;
pub use ffi::{
    drm_vc4_get_hang_state_reply, drm_vc4_submit_rcl_surface, syncobj::SyncObjHandle,
    DRM_VC4_MAX_PERF_COUNTERS,
};
use std::future::Future;
use std::os::fd::{AsFd, AsRawFd};
use tokio::io::unix::AsyncFd;

#[derive(Debug)]
/// A simple wrapper for a device node.
pub struct Card(std::fs::File);

/// Implementing `AsFd` is a prerequisite to implementing the traits found
/// in this crate. Here, we are just calling `as_fd()` on the inner File.
impl AsFd for Card {
    fn as_fd(&self) -> std::os::unix::io::BorrowedFd<'_> {
        self.0.as_fd()
    }
}

impl Device for Card {}

impl ControlDevice for Card {}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Buffer {
    handle: Handle,
    size: u32,
}

impl Buffer {
    pub fn handle(&self) -> Handle {
        self.handle
    }
    pub fn size(&self) -> u32 {
        self.size
    }
}

impl From<Buffer> for Handle {
    fn from(value: Buffer) -> Self {
        value.handle
    }
}

pub struct ImageBuffer {
    size: (u32, u32),
    format: DrmFourcc,
    pitch: u32,
    buffer: Buffer,
}

impl ImageBuffer {
    pub fn buffer(&self) -> Buffer {
        self.buffer
    }
}

impl drm::buffer::Buffer for ImageBuffer {
    fn size(&self) -> (u32, u32) {
        self.size
    }
    fn format(&self) -> DrmFourcc {
        self.format
    }
    fn pitch(&self) -> u32 {
        self.pitch
    }
    fn handle(&self) -> Handle {
        self.buffer.handle
    }
}

pub struct BufferMapping<'a> {
    _phantom: std::marker::PhantomData<&'a ()>,
    map: &'a mut [u8],
}

impl<'a> AsMut<[u8]> for BufferMapping<'a> {
    fn as_mut(&mut self) -> &mut [u8] {
        self.map
    }
}

impl<'a> Drop for BufferMapping<'a> {
    fn drop(&mut self) {
        use nix::sys::mman;
        unsafe {
            mman::munmap(self.map.as_mut_ptr() as *mut _, self.map.len()).expect("Unmap failed");
        }
    }
}

#[allow(nonstandard_style)]
pub struct drm_event_vblank {
    pub user_data: u64,
    pub tv_sec: u32,
    pub tv_usec: u32,
    pub sequence: u32,
    pub crtc_id: u32,
}

#[allow(nonstandard_style)]
pub struct drm_event_crtc_sequence {
    pub user_data: u64,
    pub time_ns: i64,
    pub sequence: u64,
}

pub enum Event {
    VBlank(drm_event_vblank),
    FlipComplete,
    CrtcSequence(drm_event_crtc_sequence),
}

/// Simple helper methods for opening a `Card`.
impl Card {
    #![allow(dead_code)]

    pub fn open(path: &str) -> Self {
        let mut options = std::fs::OpenOptions::new();
        options.read(true);
        options.write(true);
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NONBLOCK);
        Card(options.open(path).unwrap())
    }

    pub fn open_global() -> Self {
        Self::open("/dev/dri/card0")
    }

    pub fn receive_events<'a, F>(&'a self, mut event_handler: F) -> impl Future + '_
    where
        F: FnMut(Event) + 'a,
    {
        let afd = AsyncFd::with_interest(self.as_fd(), tokio::io::Interest::READABLE).unwrap();
        async move {
            let mut guard = afd.readable().await.unwrap();
            let fd = guard.get_inner();
            guard.clear_ready();
            loop {
                let mut event_buf: [u8; 1024] = [0; 1024];
                let amount = nix::unistd::read(fd.as_raw_fd(), &mut event_buf)
                    .or::<()>(Ok(0))
                    .unwrap();
                if amount == 0 {
                    break;
                }

                let mut cur: usize = 0;
                while cur < amount {
                    const DRM_EVENT_VBLANK: u32 = 1;
                    const DRM_EVENT_FLIP_COMPLETE: u32 = 2;
                    const DRM_EVENT_CRTC_SEQUENCE: u32 = 3;

                    #[allow(nonstandard_style)]
                    struct drm_event {
                        type_: u32,
                        length: u32,
                    }

                    let head = drm_event {
                        type_: u32::from_ne_bytes(event_buf[cur..cur + 4].try_into().unwrap()),
                        length: u32::from_ne_bytes(event_buf[cur + 4..cur + 8].try_into().unwrap()),
                    };

                    match head.type_ {
                        DRM_EVENT_VBLANK => {
                            event_handler(Event::VBlank(drm_event_vblank {
                                user_data: u64::from_ne_bytes(
                                    event_buf[cur + 8..cur + 16].try_into().unwrap(),
                                ),
                                tv_sec: u32::from_ne_bytes(
                                    event_buf[cur + 16..cur + 20].try_into().unwrap(),
                                ),
                                tv_usec: u32::from_ne_bytes(
                                    event_buf[cur + 20..cur + 24].try_into().unwrap(),
                                ),
                                sequence: u32::from_ne_bytes(
                                    event_buf[cur + 24..cur + 28].try_into().unwrap(),
                                ),
                                crtc_id: u32::from_ne_bytes(
                                    event_buf[cur + 28..cur + 32].try_into().unwrap(),
                                ),
                            }));
                        }
                        DRM_EVENT_FLIP_COMPLETE => {
                            event_handler(Event::FlipComplete);
                        }
                        DRM_EVENT_CRTC_SEQUENCE => {
                            event_handler(Event::CrtcSequence(drm_event_crtc_sequence {
                                user_data: u64::from_ne_bytes(
                                    event_buf[cur + 8..cur + 16].try_into().unwrap(),
                                ),
                                time_ns: i64::from_ne_bytes(
                                    event_buf[cur + 16..cur + 24].try_into().unwrap(),
                                ),
                                sequence: u64::from_ne_bytes(
                                    event_buf[cur + 24..cur + 32].try_into().unwrap(),
                                ),
                            }));
                        }
                        _ => {}
                    }

                    cur += head.length as usize;
                }
            }
        }
    }

    pub async fn wait_for_flip(&self) {
        loop {
            let mut flip_occurred = false;
            self.receive_events(|e| match e {
                Event::FlipComplete => {
                    flip_occurred = true;
                }
                _ => {}
            })
            .await;
            if flip_occurred {
                break;
            }
        }
    }

    pub fn vc4_submit_cl(
        &self,
        bin_cl: &[u8],
        shader_rec: &[u8],
        uniforms: &[u32],
        bo_handles: &[Handle],
        shader_rec_count: u32,
        width: u16,
        height: u16,
        min_x_tile: u8,
        min_y_tile: u8,
        max_x_tile: u8,
        max_y_tile: u8,
        color_read: drm_vc4_submit_rcl_surface,
        color_write: drm_vc4_submit_rcl_surface,
        zs_read: drm_vc4_submit_rcl_surface,
        zs_write: drm_vc4_submit_rcl_surface,
        msaa_color_write: drm_vc4_submit_rcl_surface,
        msaa_zs_write: drm_vc4_submit_rcl_surface,
        clear_color: [u32; 2],
        clear_z: u32,
        clear_s: u8,
        use_clear_color: bool,
        fixed_rcl_order: bool,
        rcl_order_increasing_x: bool,
        rcl_order_increasing_y: bool,
        in_sync: Option<SyncObjHandle>,
        out_sync: Option<SyncObjHandle>,
    ) -> Result<u64, SystemError> {
        let flags = if use_clear_color { 1 << 0 } else { 0 }
            | if fixed_rcl_order { 1 << 1 } else { 0 }
            | if rcl_order_increasing_x { 1 << 2 } else { 0 }
            | if rcl_order_increasing_y { 1 << 3 } else { 0 };
        ffi::vc4_submit_cl(
            self.as_fd().as_raw_fd(),
            bin_cl,
            shader_rec,
            uniforms,
            bo_handles,
            shader_rec_count,
            width,
            height,
            min_x_tile,
            min_y_tile,
            max_x_tile,
            max_y_tile,
            color_read,
            color_write,
            zs_read,
            zs_write,
            msaa_color_write,
            msaa_zs_write,
            clear_color,
            clear_z,
            clear_s,
            flags,
            in_sync,
            out_sync,
        )
    }

    pub fn vc4_submit_cl_async(
        &self,
        bin_cl: &[u8],
        shader_rec: &[u8],
        uniforms: &[u32],
        bo_handles: &[Handle],
        shader_rec_count: u32,
        width: u16,
        height: u16,
        min_x_tile: u8,
        min_y_tile: u8,
        max_x_tile: u8,
        max_y_tile: u8,
        color_read: drm_vc4_submit_rcl_surface,
        color_write: drm_vc4_submit_rcl_surface,
        zs_read: drm_vc4_submit_rcl_surface,
        zs_write: drm_vc4_submit_rcl_surface,
        msaa_color_write: drm_vc4_submit_rcl_surface,
        msaa_zs_write: drm_vc4_submit_rcl_surface,
        clear_color: [u32; 2],
        clear_z: u32,
        clear_s: u8,
        use_clear_color: bool,
        fixed_rcl_order: bool,
        rcl_order_increasing_x: bool,
        rcl_order_increasing_y: bool,
    ) -> Result<impl Future, SystemError> {
        let flags = if use_clear_color { 1 << 0 } else { 0 }
            | if fixed_rcl_order { 1 << 1 } else { 0 }
            | if rcl_order_increasing_x { 1 << 2 } else { 0 }
            | if rcl_order_increasing_y { 1 << 3 } else { 0 };

        let syncobj = {
            let syncobj_handle = ffi::syncobj::create(self.as_fd().as_raw_fd(), false)?;

            let syncobj = {
                ffi::vc4_submit_cl(
                    self.as_fd().as_raw_fd(),
                    bin_cl,
                    shader_rec,
                    uniforms,
                    bo_handles,
                    shader_rec_count,
                    width,
                    height,
                    min_x_tile,
                    min_y_tile,
                    max_x_tile,
                    max_y_tile,
                    color_read,
                    color_write,
                    zs_read,
                    zs_write,
                    msaa_color_write,
                    msaa_zs_write,
                    clear_color,
                    clear_z,
                    clear_s,
                    flags,
                    None,
                    Some(syncobj_handle),
                )?;

                ffi::syncobj::handle_to_fd(self.as_fd().as_raw_fd(), syncobj_handle, true)
            };

            ffi::syncobj::destroy(self.as_fd().as_raw_fd(), syncobj_handle)?;
            syncobj
        }?;

        let afd = AsyncFd::with_interest(syncobj, tokio::io::Interest::READABLE).unwrap();
        Ok(async move { afd.readable().await.unwrap().retain_ready() })
    }

    pub fn vc4_wait_seqno(&self, seqno: u64, timeout_ns: u64) -> Result<u64, SystemError> {
        ffi::vc4_wait_seqno(self.as_fd().as_raw_fd(), seqno, timeout_ns)
    }

    pub fn vc4_create_bo(&self, size: u32) -> Result<Buffer, SystemError> {
        let handle = ffi::vc4_create_bo(self.as_fd().as_raw_fd(), size, 0)?;
        Ok(Buffer { handle, size })
    }

    pub fn vc4_destroy_bo(&self, buffer: Buffer) -> Result<(), SystemError> {
        self.close_buffer(buffer.handle)
    }

    pub fn vc4_mmap_bo(&self, buffer: &Buffer) -> Result<BufferMapping, SystemError> {
        let offset = ffi::vc4_mmap_bo(self.as_fd().as_raw_fd(), buffer.handle, 0)?;

        let map = {
            use nix::sys::mman;
            use std::num::NonZeroUsize;
            let prot = mman::ProtFlags::PROT_READ | mman::ProtFlags::PROT_WRITE;
            let flags = mman::MapFlags::MAP_SHARED;
            let length =
                NonZeroUsize::new(buffer.size as usize).ok_or(SystemError::InvalidArgument)?;
            let fd = self.as_fd().as_raw_fd();
            let offset = offset as _;
            unsafe { mman::mmap(None, length, prot, flags, fd, offset)? }
        };

        let mapping = BufferMapping {
            _phantom: std::marker::PhantomData,
            map: unsafe { std::slice::from_raw_parts_mut(map as *mut _, buffer.size as usize) },
        };

        ffi::vc4_wait_bo(self.as_fd().as_raw_fd(), buffer.handle, u64::MAX)?;

        Ok(mapping)
    }

    pub fn vc4_create_bgra_image_buffer(
        &self,
        size: (u32, u32),
    ) -> Result<ImageBuffer, SystemError> {
        use crate::image::*;
        let size_in_bytes = Translator::alloc_size(size.into(), 32);
        let buffer = self.vc4_create_bo(size_in_bytes)?;
        self.vc4_set_tiling(buffer.handle, true)
            .expect("unable to enable tiling");
        Ok(ImageBuffer {
            size,
            format: DrmFourcc::Bgra8888,
            pitch: size.0 * 4,
            buffer,
        })
    }

    pub fn vc4_create_shader_bo(&self, data: &[u64]) -> Result<Handle, SystemError> {
        ffi::vc4_create_shader_bo(self.as_fd().as_raw_fd(), 0, data)
    }

    pub fn vc4_get_hang_state(&self) -> Result<Option<drm_vc4_get_hang_state_reply>, SystemError> {
        ffi::vc4_get_hang_state(self.as_fd().as_raw_fd())
    }

    pub fn vc4_get_param(&self, param: u32) -> Result<u64, SystemError> {
        ffi::vc4_get_param(self.as_fd().as_raw_fd(), param)
    }

    pub fn vc4_get_tiling(&self, handle: Handle) -> Result<bool, SystemError> {
        use drm_fourcc::DrmModifier;
        let modifier = ffi::vc4_get_tiling(self.as_fd().as_raw_fd(), handle, 0, 0)?;
        Ok(if modifier == DrmModifier::Broadcom_vc4_t_tiled.into() {
            true
        } else {
            false
        })
    }

    pub fn vc4_set_tiling(&self, handle: Handle, tiling: bool) -> Result<(), SystemError> {
        use drm_fourcc::DrmModifier;
        ffi::vc4_set_tiling(
            self.as_fd().as_raw_fd(),
            handle,
            0,
            if tiling {
                DrmModifier::Broadcom_vc4_t_tiled.into()
            } else {
                0
            },
        )
    }

    pub fn vc4_label_bo(&self, handle: Handle, name: &str) -> Result<(), SystemError> {
        ffi::vc4_label_bo(self.as_fd().as_raw_fd(), handle, name)
    }

    pub fn vc4_gem_madvise(&self, handle: Handle, madv: u32) -> Result<u32, SystemError> {
        ffi::vc4_gem_madvise(self.as_fd().as_raw_fd(), handle, madv)
    }

    pub fn vc4_perfmon_create(&self, events: &[u8]) -> Result<u32, SystemError> {
        ffi::vc4_perfmon_create(self.as_fd().as_raw_fd(), events)
    }

    pub fn vc4_perfmon_destroy(&self, id: u32) -> Result<(), SystemError> {
        ffi::vc4_perfmon_destroy(self.as_fd().as_raw_fd(), id)
    }

    pub fn vc4_perfmon_get_values(
        &self,
        id: u32,
    ) -> Result<[u64; DRM_VC4_MAX_PERF_COUNTERS], SystemError> {
        ffi::vc4_perfmon_get_values(self.as_fd().as_raw_fd(), id)
    }
}

use core::ffi;
use lammps_sys;
use std::ptr;

pub struct Lammps {
    handle: *mut ffi::c_void,
}

impl Lammps {
    pub fn new() -> Self {
        unsafe {
            let handle = lammps_sys::lammps_open_no_mpi(0, ptr::null_mut(), ptr::null_mut());
            Lammps { handle }
        }
    }

    pub fn version(&self) -> i32 {
        unsafe { lammps_sys::lammps_version(self.handle) }
    }
}

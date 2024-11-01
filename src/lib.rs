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

    pub fn command(&self) {
        unimplemented!()
    }

    pub fn command_list(&self) {
        unimplemented!()
    }

    pub fn command_string(&self) {
        unimplemented!()
    }

    pub fn create_atoms(&self) {
        unimplemented!()
    }

    pub fn extract_atoms(&self) {
        unimplemented!()
    }

    pub fn extract_atom_datatype(&self, name: &str) {
        unimplemented!()
    }

    pub fn extract_box(&self) {
        unimplemented!()
    }

    pub fn extract_compute(&self) {
        unimplemented!()
    }

    pub fn extract_fix(&self) {
        unimplemented!()
    }

    pub fn extract_global(&self) {
        unimplemented!()
    }

    pub fn extract_global_datatype(&self) {
        unimplemented!()
    }

    pub fn extract_variable(&self) {
        unimplemented!()
    }

    pub fn file(&self, path: &str) {
        unimplemented!()
    }

    pub fn version(&self) -> i32 {
        unsafe { lammps_sys::lammps_version(self.handle) }
    }
}

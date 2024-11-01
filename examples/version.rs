fn main() {
    let lmp = lammps_rust::Lammps::new();
    let version = lmp.version();
    println!("LAMMPS version: {version}");
}

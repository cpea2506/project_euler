// Lattice Paths

use number::combination::Combination;

fn lattice_path_counter(grid_size: usize) -> usize {
    Combination::new(grid_size * 2, grid_size).value()
}

pj_euler::run!("Lattice Paths", lattice_path_counter(20));

pj_euler::test!(
    lattice_paths {
        {grid_size_2_have_6_paths, lattice_path_counter(2), 6},
    }
);

#![feature(io, path)]

pub fn find(name: &str) -> Path {
    use std::old_io::fs::PathExtensions;
    let path = Path::new("tests").join_many(&["fixtures", name]);
    assert!(path.exists());
    path
}

pub const C: [f64; 20] = [
    3.496500000000000e-04, 3.496500000000000e-04,
    1.065600000000000e-04, 1.065600000000000e-04,
    4.728600000000000e-03, 4.728600000000000e-03,
    8.457534000000000e-02, 8.457534000000000e-02,
    2.458872000000000e-01, 2.458872000000000e-01,
    2.813517000000000e-01, 2.813517000000000e-01,
    4.397917680000000e+00, 4.397917680000000e+00,
    5.032232730000000e+00, 5.032232730000000e+00,
    1.427208862500000e+01, 1.427208862500000e+01,
    1.427208862500000e+01, 1.427208862500000e+01,
];

pub const G: [f64; 400] = [
     2.681666666666667e+00, -1.500000000000000e-02,
    -2.666666666666667e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
    -1.500000000000000e-02,  2.681666666666667e+00,
     0.000000000000000e+00, -2.666666666666667e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
    -2.666666666666667e+00,  0.000000000000000e+00,
     3.466746666666666e+00, -8.000000000000001e-05,
    -7.999999999999998e-01,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00, -2.666666666666667e+00,
    -8.000000000000001e-05,  3.466746666666666e+00,
     0.000000000000000e+00, -7.999999999999998e-01,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
    -7.999999999999998e-01,  0.000000000000000e+00,
     3.563636363636363e+00, -3.999999999999999e-01,
    -1.600000000000000e+00,  0.000000000000000e+00,
    -3.272727272727273e-01,  0.000000000000000e+00,
    -2.181818181818182e-01, -2.181818181818182e-01,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00, -7.999999999999998e-01,
    -3.999999999999999e-01,  3.563636363636363e+00,
     0.000000000000000e+00, -1.600000000000000e+00,
     0.000000000000000e+00, -3.272727272727273e-01,
    -2.181818181818182e-01, -2.181818181818182e-01,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
    -1.600000000000000e+00,  0.000000000000000e+00,
     9.639693957467317e+00, -2.760000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
    -2.258181818181818e+00,  0.000000000000000e+00,
    -1.505454545454545e+00, -1.505454545454545e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00, -1.600000000000000e+00,
    -2.760000000000000e+00,  9.639693957467317e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00, -2.258181818181818e+00,
    -1.505454545454545e+00, -1.505454545454545e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
    -3.272727272727273e-01,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     8.352727272727272e+01,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
    -8.319999999999999e+01,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00, -3.272727272727273e-01,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  8.352727272727272e+01,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00, -8.319999999999999e+01,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
    -2.181818181818182e-01, -2.181818181818182e-01,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     9.563636363636363e+01,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
    -9.519999999999999e+01,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
    -2.181818181818182e-01, -2.181818181818182e-01,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  9.563636363636363e+01,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00, -9.519999999999999e+01,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
    -2.258181818181818e+00,  0.000000000000000e+00,
    -8.319999999999999e+01,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     9.172845925267396e+01,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
    -5.718918918918919e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00, -2.258181818181818e+00,
     0.000000000000000e+00, -8.319999999999999e+01,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  9.172845925267396e+01,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00, -5.718918918918919e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
    -1.505454545454545e+00, -1.505454545454545e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
    -9.519999999999999e+01,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     1.043853802128951e+02,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
    -5.543589743589744e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
    -1.505454545454545e+00, -1.505454545454545e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00, -9.519999999999999e+01,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  1.043853802128951e+02,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00, -5.543589743589744e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
    -5.718918918918919e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     7.508183332437806e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00, -5.718918918918919e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  7.508183332437806e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
    -5.543589743589744e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     7.332854157108631e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00, -5.543589743589744e+00,
     0.000000000000000e+00,  0.000000000000000e+00,
     0.000000000000000e+00,  7.332854157108631e+00,
];

pub const C42: [f64; 20] = [
    9.7902000000000015e-04, 9.7902000000000015e-04,
    1.0656000000000000e-04, 1.0656000000000000e-04,
    4.7286000000000003e-03, 4.7286000000000003e-03,
    8.4575339999999999e-02, 8.4575339999999999e-02,
    2.4588720000000000e-01, 2.4588720000000000e-01,
    2.8135169999999998e-01, 2.8135169999999998e-01,
    4.3979176799999999e+00, 4.3979176799999999e+00,
    5.0322327300000005e+00, 5.0322327300000005e+00,
    1.4272088625000002e+01, 1.4272088625000002e+01,
    1.4272088625000002e+01, 1.4272088625000002e+01,
];

pub const G42: [f64; 400] = [
     4.1763999999999990e-01, -1.7639999999999999e-02,
    -3.9999999999999991e-01,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
    -1.7639999999999999e-02,  4.1763999999999990e-01,
     0.0000000000000000e+00, -3.9999999999999991e-01,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
    -3.9999999999999991e-01,  0.0000000000000000e+00,
     1.2000799999999998e+00, -8.0000000000000007e-05,
    -7.9999999999999982e-01,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00, -3.9999999999999991e-01,
    -8.0000000000000007e-05,  1.2000799999999998e+00,
     0.0000000000000000e+00, -7.9999999999999982e-01,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
    -7.9999999999999982e-01,  0.0000000000000000e+00,
     3.5636363636363630e+00, -3.9999999999999991e-01,
    -1.5999999999999996e+00,  0.0000000000000000e+00,
    -3.2727272727272727e-01,  0.0000000000000000e+00,
    -2.1818181818181820e-01, -2.1818181818181820e-01,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00, -7.9999999999999982e-01,
    -3.9999999999999991e-01,  3.5636363636363630e+00,
     0.0000000000000000e+00, -1.5999999999999996e+00,
     0.0000000000000000e+00, -3.2727272727272727e-01,
    -2.1818181818181820e-01, -2.1818181818181820e-01,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
    -1.5999999999999996e+00,  0.0000000000000000e+00,
     9.6396939574673173e+00, -2.7599999999999998e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
    -2.2581818181818178e+00,  0.0000000000000000e+00,
    -1.5054545454545454e+00, -1.5054545454545454e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00, -1.5999999999999996e+00,
    -2.7599999999999998e+00,  9.6396939574673173e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00, -2.2581818181818178e+00,
    -1.5054545454545454e+00, -1.5054545454545454e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
    -3.2727272727272727e-01,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     8.3527272727272717e+01,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
    -8.3199999999999989e+01,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00, -3.2727272727272727e-01,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  8.3527272727272717e+01,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00, -8.3199999999999989e+01,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
    -2.1818181818181820e-01, -2.1818181818181820e-01,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     9.5636363636363626e+01,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
    -9.5199999999999989e+01,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
    -2.1818181818181820e-01, -2.1818181818181820e-01,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  9.5636363636363626e+01,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00, -9.5199999999999989e+01,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
    -2.2581818181818178e+00,  0.0000000000000000e+00,
    -8.3199999999999989e+01,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     9.1728459252673957e+01,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
    -5.7189189189189191e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00, -2.2581818181818178e+00,
     0.0000000000000000e+00, -8.3199999999999989e+01,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  9.1728459252673957e+01,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00, -5.7189189189189191e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
    -1.5054545454545454e+00, -1.5054545454545454e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
    -9.5199999999999989e+01,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     1.0438538021289511e+02,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
    -5.5435897435897443e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
    -1.5054545454545454e+00, -1.5054545454545454e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00, -9.5199999999999989e+01,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  1.0438538021289511e+02,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00, -5.5435897435897443e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
    -5.7189189189189191e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     7.5081833324378060e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00, -5.7189189189189191e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  7.5081833324378060e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
    -5.5435897435897443e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     7.3328541571086312e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00, -5.5435897435897443e+00,
     0.0000000000000000e+00,  0.0000000000000000e+00,
     0.0000000000000000e+00,  7.3328541571086312e+00,
];

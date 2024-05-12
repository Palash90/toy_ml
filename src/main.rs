use toy_ml::gradient_descent;

// The main entry point for the toy_ml gradient descent example.
//
// This program demonstrates how to use the `gradient_descent` function from the `toy_ml` library
// to find the line of best fit for a given set of data points. It's a basic example intended to
// help beginners understand how to implement and run a gradient descent algorithm in Rust.
fn main() {
    let mut m = 0.0;
    let mut c = 0.0;
    let l = 0.0001;

    let e = 1000000;

    let x = vec![
        36.85191021,
        8.09313221,
        27.72687022,
        37.25256641,
        32.93361865,
        14.33699999,
        15.98077583,
        32.62826987,
        18.75070414,
        44.52014864,
        18.16953143,
        15.01982104,
        43.38762669,
        16.48324161,
        24.02192846,
        40.10156349,
        42.35344594,
        48.16645729,
        24.51574622,
        36.0875022,
    ];
    let y = vec![
        334.25955106,
        190.46566107,
        288.6343511,
        336.26283204,
        314.66809323,
        221.68499996,
        229.90387915,
        313.14134934,
        243.7535207,
        372.60074321,
        240.84765716,
        225.0991052,
        366.93813345,
        232.41620804,
        270.10964229,
        350.50781746,
        371.76722971,
        380.83228647,
        278.57873109,
        335.437511,
    ];

    for _ in 0..e {
        (m, c) = gradient_descent(m, c, l, &x, &y);
    }

    println!("Calculated value of slope and y-intercept- {:?}", (m, c));

    let new_x = vec![3.0, 6.0]; // New values of X to be predicted
    let mut predictions: Vec<f64> = Vec::new();

    for x in &new_x {
        predictions.push(x * m + c);
    }

    println!("{:?}", predictions);
}

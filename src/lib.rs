//! A simple machine learning library in Rust.
//!
//! `toy_ml` is designed to be a "Hello World" for machine learning enthusiasts
//! looking to get started with Rust. It provides a basic implementation of the
//! gradient descent algorithm, which, while simple, serves as a starting point
//! for understanding machine learning concepts in Rust.

/// The `gradient_descent` function performs a single step of the gradient descent optimization algorithm.
///
/// # Arguments
///
/// * `m_now` - The current value of the slope (m) of the line.
/// * `c_now` - The current value of the y-intercept (c) of the line.
/// * `l` - The learning rate, which controls the size of the update step.
/// * `x` - A reference to a vector containing the x-coordinates of the data points.
/// * `y` - A reference to a vector containing the y-coordinates of the data points.
///
/// # Returns
///
/// A tuple containing the updated values of the slope (m) and y-intercept (c).
///
/// # Example
///
/// ```
/// let m_initial: f64 = 0.0;
/// let c_initial: f64 = 0.0;
/// let learning_rate: f64 = 0.01;
/// let x_coords: Vec<f64> = vec![1.0, 2.0, 3.0];
/// let y_coords: Vec<f64> = vec![2.0, 4.0, 6.0];
///
/// let (m_updated, c_updated) = toy_ml::gradient_descent(m_initial, c_initial, learning_rate, &x_coords, &y_coords);
/// ```
///
/// Note: This implementation is intended for educational purposes and may not be suitable for production use.
/// It aims to inspire and provide a foundation for those interested in exploring machine learning with Rust.
pub fn gradient_descent(m_now: f64, c_now: f64, l: f64, x: &Vec<f64>, y: &Vec<f64>) -> (f64, f64) {
    if x.len() != y.len() {
        panic!(
            "The length of Dependent variable and independent variable are not matching {} {}",
            x.len(),
            y.len()
        );
    }

    let mut m_gradient = 0.0;
    let mut c_gradient = 0.0;

    let n = x.len();

    for i in 0..n {
        let x = &x[i];
        let y = &y[i];

        m_gradient += -(2.0 / n as f64) * x * (y - (m_now * x + c_now));
        c_gradient += -(2.0 / n as f64) * (y - (m_now * x + c_now));
    }

    m_gradient = m_now - m_gradient * l;
    c_gradient = c_now - c_gradient * l;

    (m_gradient, c_gradient)
}

# Toy ML

Welcome to `toy_ml`, a minimalist machine learning library in Rust designed to serve as a "Hello World" introduction to machine learning concepts. This project is akin to the first simple program you write when learning a new programming language; it's not meant to do much, but it ignites hope and curiosity in the field of machine learning.

## Overview

Rust's steep learning curve can be intimidating, especially when venturing into the realm of machine learning. `toy_ml` aims to change that perception by offering a straightforward implementation of the gradient descent algorithm. It's not meant to be comprehensive but rather a beacon for the curious minds looking to explore machine learning in Rust.

## Installation

Add `toy_ml` to your Cargo.toml dependencies:

```toml
[dependencies]
toy_ml = "0.1.0"
```

## Usage

Here's how you can use `toy_ml` to perform gradient descent:

```rust
use toy_ml::gradient_descent;

fn main() {
    // Initialize the slope (m) and y-intercept (c) to 0.
    let mut m = 0.0;
    let mut b = 0.0;
    // Define the learning rate and number of epochs.
    let learning_rate = 0.0001;
    let epochs = 1000000;

    // Define your data points here.
    let x_coords = vec![...]; // x-coordinates
    let y_coords = vec![...]; // y-coordinates

    // Run the gradient descent algorithm.
    for _ in 0..epochs {
        let (new_m, new_c) = gradient_descent(m, c, learning_rate, &x_coords, &y_coords);
        m = new_m;
        c = new_c;
    }

    println!("Calculated values - Slope: {:?}, Intercept: {:?}", m, c);

    let new_x = vec![3.0, 6.0]; // New values of X to be predicted
    let mut predictions:Vec<f64> = Vec::new();

    for x in &new_x{
        predictions.push(x * m + c);
    }

    println!("{:?}", predictions);
}
```

## Contributing

Contributions are welcome! If you have ideas for improvements or want to help refine the crate, feel free to create issues or submit pull requests.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

`toy_ml` is a humble project with grand aspirations. It doesn't promise the stars, but it does hope to launch a thousand journeys into machine learning with Rust. If it inspires even one person to start their journey, it has succeeded in its mission.

Happy coding!

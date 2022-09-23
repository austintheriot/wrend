use std::f32::consts::PI;

pub fn build_gaussian_kernel(kernel_size: usize) -> Vec<f32> {
    let kernel_len = kernel_size.pow(2);
    let mut kernel = vec![0.0; kernel_len];

    // see comment for rule of thumb around Sigma value:
    // https://stackoverflow.com/a/62002971/14967537
    let sigma: f32 = (kernel_size as f32 - 1.0 ) / 6.0;
    for x in 0..kernel_size {
        for y in 0..kernel_size {
            kernel[y * kernel_size + x] = {
                // see formula for Gaussian Blur kernel at https://en.wikipedia.org/wiki/Gaussian_blur
                let x = x as f32;
                let y = y as f32;
                f32::exp(-((x.powf(2.0) + y.powf(2.0)) / (2.0 * sigma.powf(2.0))))
                    / (2.0 * PI * sigma.powf(2.0))
            }
        }
    }

    let kernel_sum: f32 = kernel.iter().sum();
    kernel.iter_mut().for_each(|el| *el /= kernel_sum);

    kernel
}
use library_bundler::{bundle, bundling_option::BundlingOption};

fn main() {
    let option = BundlingOption::new();
    let bundled_source = bundle(&option);
    print!("{}", bundled_source);
}

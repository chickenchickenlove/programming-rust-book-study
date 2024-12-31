

struct Polynomial<const N: usize> {
    coefficients: [f64; N]
}

impl<const N: usize> Polynomial<N> {

    pub fn new(coefficients: [f64; N]) -> Polynomial<N> {
        Self { coefficients }
    }
}

fn t() {
    let coefficients = [0.0; 6];
    let poloynomial = Polynomial::new(coefficients);
}

struct LumpOfReferences<'a, T, const N: usize> {
    the_lump: [&'a T; N],
}
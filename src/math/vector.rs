type VecNd<const N: usize> = [f64; N];

pub type Vec2d = VecNd<2>;
pub type Vec3d = VecNd<3>;
pub type Vec4d = VecNd<4>;

pub fn neg<const N: usize>(vector: &VecNd<N>) -> VecNd<N> {
    let mut result = [0.; N];
    (0..N).for_each(|i| result[i] = -vector[i]);
    result
}

pub fn add<const N: usize>(lhs: &VecNd<N>, rhs: &VecNd<N>) -> VecNd<N> {
    let mut result = [0.; N];
    (0..N).for_each(|i| result[i] = lhs[i] + rhs[i]);
    result
}

pub fn sub<const N: usize>(lhs: &VecNd<N>, rhs: &VecNd<N>) -> VecNd<N> {
    let mut result = [0.; N];
    (0..N).for_each(|i| result[i] = lhs[i] - rhs[i]);
    result
}

pub fn mul<const N: usize>(lhs: &VecNd<N>, rhs: &VecNd<N>) -> VecNd<N> {
    let mut result = [0.; N];
    (0..N).for_each(|i| result[i] = lhs[i] * rhs[i]);
    result
}

pub fn scalar_mul<const N: usize>(vector: &VecNd<N>, factor: f64) -> VecNd<N> {
    let mut result = [0.; N];
    (0..N).for_each(|i| result[i] = vector[i] * factor);
    result
}

pub fn div<const N: usize>(vector: &VecNd<N>, divisor: f64) -> VecNd<N> {
    scalar_mul(vector, 1.0 / divisor)
}

pub fn length<const N: usize>(vector: &VecNd<N>) -> f64 {
    let mut length_squared: f64 = 0.;
    (0..N).for_each(|i| length_squared += vector[i] * vector[i]);
    length_squared.sqrt()
}

pub fn unit<const N: usize>(vector: &VecNd<N>) -> VecNd<N> {
    div(vector, length(vector))
}

pub fn dot_product<const N: usize>(lhs: &VecNd<N>, rhs: &VecNd<N>) -> f64 {
    std::iter::zip(lhs.iter(), rhs.iter()).fold(0., |acc, zip| acc + zip.0 * zip.1)
}

pub fn cross_product(lhs: &Vec3d, rhs: &Vec3d) -> Vec3d {
    [
        lhs[1] * rhs[2] - lhs[2] * rhs[1],
        lhs[2] * rhs[0] - lhs[0] * rhs[2],
        lhs[0] * rhs[1] - lhs[1] * rhs[0],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub() {
        let lhs: Vec4d = [1., 2., -4., 6.];
        let rhs: Vec4d = [2., 3., -1., 0.];

        let dist: Vec4d = [-1., -1., -3., 6.];
        assert_eq!(sub(&lhs, &rhs), dist);
    }

    #[test]
    fn test_length() {
        let vector: Vec4d = [-1., -1., -3., 6.];
        assert_eq!(length(&vector), 47_f64.sqrt());
    }

    #[test]
    fn test_unit() {
        {
            let unit_vector: Vec2d = unit(&[5., 12.]);
            let [x, y] = unit_vector;
            let rounded_unit_vector = [(x * 1000.).round() / 1000., (y * 1000.).round() / 1000.];
            assert_eq!(rounded_unit_vector, [0.385, 0.923]);
        }

        {
            let unit_vector: Vec3d = unit(&[2., -4., 1.]);
            let [x, y, z] = unit_vector;
            let rounded_unit_vector = [
                (x * 100.).round() / 100.,
                (y * 100.).round() / 100.,
                (z * 100.).round() / 100.,
            ];
            assert_eq!(rounded_unit_vector, [0.44, -0.87, 0.22]);
        }
    }

    #[test]
    fn test_dot_product() {
        let lhs = [1., 3., -5.];
        let rhs = [4., -2., -1.];

        assert_eq!(dot_product(&lhs, &rhs), 3.);
        assert_eq!(dot_product(&lhs, &lhs), 35.);
    }

    #[test]
    fn test_cross_product() {
        let lhs = [-1., 2., 5.];
        let rhs = [4., 0., -3.];

        assert_eq!(cross_product(&lhs, &rhs), [-6., 17., -8.]);
    }
}

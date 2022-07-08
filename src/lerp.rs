pub trait Lerp<T> {
    /// The scaling type for linear interpolation.

    /// Given `self` and another point `other`, return a point on a line running
    /// between the two that is `scalar` fraction of the distance between
    /// the two points.
    fn lerp(&self, other: &Self, scalar: f32, target: &T, options: &Option<Vec<String>>) -> Self;
}

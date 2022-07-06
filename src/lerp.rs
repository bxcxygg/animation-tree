pub trait Lerp<T> {
    /// The scaling type for linear interpolation.
    type Scalar;

    /// Given `self` and another point `other`, return a point on a line running
    /// between the two that is `scalar` fraction of the distance between
    /// the two points.
    fn lerp(
        &self,
        other: &Self,
        scalar: &Self::Scalar,
        target: &T,
        options: &Option<Vec<String>>,
    ) -> Self;
}

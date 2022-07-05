pub trait Lerp {
    /// The scaling type for linear interpolation.
    type Scalar;

    type Target;

    /// Given `self` and another point `other`, return a point on a line running
    /// between the two that is `scalar` fraction of the distance between
    /// the two points.
    fn lerp(
        &self,
        other: &Self,
        scalar: &Self::Scalar,
        target: &Self::Target,
        options: &Option<Vec<String>>,
    ) -> Self;
}

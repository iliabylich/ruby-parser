pub(crate) trait HasInterpolation {
    fn currently_in_interpolation(&self) -> bool;
    fn currently_in_interpolation_mut(&mut self) -> &mut bool;

    fn supports_interpolation(&self) -> bool;

    fn interpolation_started_with_curly_level(&self) -> usize;
}

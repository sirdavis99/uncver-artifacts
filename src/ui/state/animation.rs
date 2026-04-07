pub struct AnimationProgress {
    pub progress: f32,
    pub is_animating: bool,
}

impl Default for AnimationProgress {
    fn default() -> Self {
        Self {
            progress: 0.0,
            is_animating: false,
        }
    }
}

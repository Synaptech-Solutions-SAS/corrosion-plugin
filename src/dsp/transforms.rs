#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SizeScale {
    factor: f32,
}

impl SizeScale {
    const NEUTRAL: Self = Self { factor: 1.0 };
    const MIN_FACTOR: f32 = 0.25;

    pub fn new(factor: f32) -> Self {
        let sanitized = if factor.is_finite() {
            factor.max(Self::MIN_FACTOR)
        } else {
            Self::NEUTRAL.factor
        };

        Self { factor: sanitized }
    }

    pub fn factor(self) -> f32 {
        self.factor
    }
}

impl Default for SizeScale {
    fn default() -> Self {
        Self::NEUTRAL
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RustAmount {
    amount: f32,
}

impl RustAmount {
    const NEUTRAL: Self = Self { amount: 0.0 };

    pub fn new(amount: f32) -> Self {
        let sanitized = if amount.is_finite() {
            amount.clamp(0.0, 1.0)
        } else {
            Self::NEUTRAL.amount
        };

        Self { amount: sanitized }
    }

    pub fn amount(self) -> f32 {
        self.amount
    }
}

impl Default for RustAmount {
    fn default() -> Self {
        Self::NEUTRAL
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DamageAmount {
    amount: f32,
}

impl DamageAmount {
    const NEUTRAL: Self = Self { amount: 0.0 };

    pub fn new(amount: f32) -> Self {
        let sanitized = if amount.is_finite() {
            amount.clamp(0.0, 1.0)
        } else {
            Self::NEUTRAL.amount
        };

        Self { amount: sanitized }
    }

    pub fn amount(self) -> f32 {
        self.amount
    }
}

impl Default for DamageAmount {
    fn default() -> Self {
        Self::NEUTRAL
    }
}

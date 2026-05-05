//! Normalized control-value transforms for modal DSP parameters.
//!
//! These wrappers keep the voice and profile layers operating on bounded,
//! auditable scalars instead of raw unchecked floats.

#[derive(Clone, Copy, Debug, PartialEq)]
/// Normalized size multiplier used to scale modal frequency and decay.
pub struct SizeScale {
    factor: f32,
}

impl SizeScale {
    const NEUTRAL: Self = Self { factor: 1.0 };
    const MIN_FACTOR: f32 = 0.05;

    /// Create a bounded size scale.
    pub fn new(factor: f32) -> Self {
        let sanitized = if factor.is_finite() {
            factor.max(Self::MIN_FACTOR)
        } else {
            Self::NEUTRAL.factor
        };

        Self { factor: sanitized }
    }

    /// Return the sanitized size factor.
    pub fn factor(self) -> f32 {
        self.factor
    }
}

impl Default for SizeScale {
    /// Return the neutral size scale.
    fn default() -> Self {
        Self::NEUTRAL
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
/// Corrosion intensity used to darken and shorten modal responses.
pub struct RustAmount {
    amount: f32,
}

impl RustAmount {
    const NEUTRAL: Self = Self { amount: 0.0 };
    const MAX_AMOUNT: f32 = 5.0;

    /// Create a bounded rust amount.
    pub fn new(amount: f32) -> Self {
        let sanitized = if amount.is_finite() {
            amount.clamp(0.0, Self::MAX_AMOUNT)
        } else {
            Self::NEUTRAL.amount
        };

        Self { amount: sanitized }
    }

    /// Return the sanitized rust amount.
    pub fn amount(self) -> f32 {
        self.amount
    }
}

impl Default for RustAmount {
    /// Return the neutral rust amount.
    fn default() -> Self {
        Self::NEUTRAL
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
/// Damage intensity used to split and detune modal peaks.
pub struct DamageAmount {
    amount: f32,
}

impl DamageAmount {
    const NEUTRAL: Self = Self { amount: 0.0 };
    const MAX_AMOUNT: f32 = 10.0;

    /// Create a bounded damage amount.
    pub fn new(amount: f32) -> Self {
        let sanitized = if amount.is_finite() {
            amount.clamp(0.0, Self::MAX_AMOUNT)
        } else {
            Self::NEUTRAL.amount
        };

        Self { amount: sanitized }
    }

    /// Return the sanitized damage amount.
    pub fn amount(self) -> f32 {
        self.amount
    }
}

impl Default for DamageAmount {
    /// Return the neutral damage amount.
    fn default() -> Self {
        Self::NEUTRAL
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
/// Wall-thickness control used to stiffen modal behavior.
pub struct ThicknessAmount {
    amount: f32,
}

impl ThicknessAmount {
    const NEUTRAL: Self = Self { amount: 0.5 };

    /// Create a bounded thickness amount.
    pub fn new(amount: f32) -> Self {
        let sanitized = if amount.is_finite() {
            amount.clamp(0.0, 1.0)
        } else {
            Self::NEUTRAL.amount
        };

        Self { amount: sanitized }
    }

    /// Return the sanitized thickness amount.
    pub fn amount(self) -> f32 {
        self.amount
    }
}

impl Default for ThicknessAmount {
    /// Return the neutral thickness amount.
    fn default() -> Self {
        Self::NEUTRAL
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
/// Heat control used to soften and darken modal response.
pub struct HeatAmount {
    amount: f32,
}

impl HeatAmount {
    const NEUTRAL: Self = Self { amount: 0.0 };

    /// Create a bounded heat amount.
    pub fn new(amount: f32) -> Self {
        let sanitized = if amount.is_finite() {
            amount.clamp(0.0, 1.0)
        } else {
            Self::NEUTRAL.amount
        };

        Self { amount: sanitized }
    }

    /// Return the sanitized heat amount.
    pub fn amount(self) -> f32 {
        self.amount
    }
}

impl Default for HeatAmount {
    /// Return the neutral heat amount.
    fn default() -> Self {
        Self::NEUTRAL
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
/// Sludge control used to add mass and low-pass damping.
pub struct SludgeAmount {
    amount: f32,
}

impl SludgeAmount {
    const NEUTRAL: Self = Self { amount: 0.0 };

    /// Create a bounded sludge amount.
    pub fn new(amount: f32) -> Self {
        let sanitized = if amount.is_finite() {
            amount.clamp(0.0, 1.0)
        } else {
            Self::NEUTRAL.amount
        };

        Self { amount: sanitized }
    }

    /// Return the sanitized sludge amount.
    pub fn amount(self) -> f32 {
        self.amount
    }
}

impl Default for SludgeAmount {
    /// Return the neutral sludge amount.
    fn default() -> Self {
        Self::NEUTRAL
    }
}

# Comprehensive Code Review: Corrosion VST Plugin

**Project:** Corrosion - Industrial Physical Modeling Synthesizer  
**Language:** Rust  
**Framework:** nih_plug  
**Review Date:** May 2026  
**Status:** Production-ready with areas for improvement  

---

## Executive Summary

The Corrosion VST plugin is a sophisticated industrial physical modeling synthesizer with 17 different exciter types across three categories (Hit, Scrape, Specialty). The codebase demonstrates solid architecture with good separation of concerns, comprehensive test coverage, and adherence to real-time audio constraints. However, there are several areas where code quality, performance, and maintainability can be improved.

**Overall Grade: B+**

---

## 1. Architecture & Design

### 1.1 Module Structure

**Strengths:**
- Clean separation between `dsp/`, `voice/`, `gui/`, and `params/` modules
- Good use of Rust's module system with proper visibility controls
- Exciters are modular and follow consistent patterns
- Parameter system is well-structured with nih_plug integration

**Concerns:**
- **File:** `src/voice/mod.rs` (lines 109-157)
  - `Voice` struct contains all 16 exciter instances simultaneously (lines 118-133)
  - This creates memory overhead even when only one exciter is active
  - Each Voice is ~3-4KB just for exciter storage
  - With MAX_VOICES=8, this wastes ~24-28KB per plugin instance

**Recommendation:** Consider using an enum-based approach or boxing to reduce memory footprint:
```rust
enum ExciterInstance {
    Scrape(ScrapeExciter),
    HandStrike(HandStrike),
    // ... etc
}
```

### 1.2 Plugin Architecture

**Strengths:**
- Proper implementation of nih_plug traits (`Plugin`, `ClapPlugin`, `Vst3Plugin`)
- Correct parameter exposure through `Arc<dyn Params>`
- State serialization using serde_json

**Concerns:**
- **File:** `src/lib.rs` (lines 75-140)
  - `handle_note_event` manually maps all parameters (lines 89-122)
  - This is brittle and error-prone when adding new parameters
  - Better to use a macro or derive-based approach

---

## 2. Real-Time Safety

### 2.1 Audio Thread Compliance

**Strengths:**
- ✓ No heap allocations in `process()` loop
- ✓ No mutexes or locks in audio thread
- ✓ No file I/O during processing
- ✓ No println! or logging in hot paths

**File:** `src/lib.rs` (lines 233-329)
```rust
fn process(&mut self, buffer: &mut Buffer, ...) -> ProcessStatus {
    // Clean real-time loop with no allocations
    for (sample_id, channel_samples) in buffer.iter_samples().enumerate() {
        // ... processing
    }
}
```

### 2.2 Concerns

**File:** `src/dsp/exciters/other_specialty.rs` (line 207)
```rust
particles: Vec::new(),  // Allocates in new() - OK if called pre-RT
```
- Vec allocations happen in `new()` which is acceptable if called before real-time
- However, `ParticleRain` and other exciters may resize during trigger

**File:** `src/voice/mod.rs` (line 865)
```rust
let mut output = Vec::with_capacity(total_frames);  // In test code - OK
```
- Test code allocates but that's acceptable

---

## 3. Code Quality Issues

### 3.1 Error Handling

**Critical Issue - File:** `src/voice/manager.rs` (line 82)
```rust
let peak_cmp = a.peak_hold().partial_cmp(&b.peak_hold()).unwrap();
```
- `unwrap()` on `partial_cmp()` which can return `None` for NaN values
- **Risk:** Panic if peak_hold becomes NaN
- **Fix:** Use `unwrap_or()` with a sensible default

**File:** `src/lib.rs` (lines 425-426)
```rust
Arc::get_mut(&mut pipe_plugin.params).unwrap().object = ...
```
- Test code uses unwrap() - acceptable but could be cleaner

### 3.2 Magic Numbers

**File:** `src/voice/mod.rs` - Extensive use of magic numbers
```rust
// Lines 443-587: Hardcoded exciter parameter mappings
2 => {  // HandStrike
    self.hand_strike.set_parameters(
        0.4 + pressure * 2.6,      // What do these mean?
        0.05 + speed * 0.75,       // Undocumented scaling factors
        0.3 + roughness * 1.5,     // No explanation
        0.85 + controls.env_release.clamp(0.0, 1.0) * 0.149,
    );
}
```

**Recommendation:** Document these scaling factors or extract to named constants:
```rust
const HAND_STRIKE_MASS_BASE: f32 = 0.4;
const HAND_STRIKE_MASS_SCALE: f32 = 2.6;
```

**File:** `src/lib.rs` (lines 37-72)
```rust
// Hardcoded thresholds in drive algorithm
let soft_threshold = 0.3;  // Why 0.3?
let hard_threshold = 0.8;  // Why 0.8?
```

### 3.3 Code Duplication

**Issue:** All exciters implement nearly identical boilerplate:
- `new()` method
- `set_parameters()` method
- `trigger()` method
- `is_active()` method
- `Default` trait implementation

**Example Pattern (repeated 17 times):**
```rust
impl Default for SomeExciter {
    fn default() -> Self {
        Self::new()
    }
}
```

**Recommendation:** Consider a macro or trait with default implementations to reduce boilerplate.

---

## 4. Performance Concerns

### 4.1 Branch Prediction

**File:** `src/voice/mod.rs` (lines 666-696)
```rust
let excitation = match self.exciter_type {
    1 => self.scrape_exciter.process_sample(res_vel),
    2 => self.hand_strike.process_sample(res_disp, res_vel),
    // ... 15 more branches
    _ => { /* default */ }
};
```
- Large match statement on every sample
- Branch predictor may struggle with 17+ arms
- Consider: Jump table or function pointer dispatch

### 4.2 Cache Efficiency

**File:** `src/voice/mod.rs`
- Voice struct is large (~500+ bytes) due to all exciter instances
- Exciters that aren't used still consume cache lines
- With 8 voices, this is 4KB+ of often-unused data

### 4.3 Inefficient Algorithms

**File:** `src/dsp/exciters/wire_brush.rs` (lines 114-157)
```rust
pub fn process_sample(...) -> f32 {
    // Iterates through ALL impulses every sample
    for impulse in &mut self.impulses {
        if !impulse.triggered && impulse.time_ms <= self.time_ms {
            // ...
        }
    }
}
```
- O(N) per sample where N = wire_density (up to 5000)
- Better: Use a priority queue or index pointer

**File:** `src/dsp/exciters/metal_chain.rs` (lines 112-159)
- Similar issue: iterates through all links every sample

---

## 5. Safety & Correctness

### 5.1 Numerical Stability

**Strengths:**
- ✓ Denormal handling present (file: `src/voice/mod.rs`, line 725-726)
```rust
const DENORMAL_FLUSH: f32 = 1e-20;
let flushed = clamped + DENORMAL_FLUSH - DENORMAL_FLUSH;
```
- ✓ NaN/Inf checks in output (lines 708-714)

### 5.2 Float Comparison

**File:** `src/dsp/exciters/scrape.rs` (lines 67-69)
```rust
pub fn is_active(&self) -> bool {
    self.pressure > 0.01 && self.target_speed > 0.001  // OK - not exact equality
}
```
- Generally OK but could use epsilon for clarity

### 5.3 Boundary Conditions

**File:** `src/dsp/resonator.rs` (lines 13-16)
```rust
let decay_seconds = mode.decay_seconds.max(f32::EPSILON);
let omega = 2.0 * PI * mode.frequency_hz / safe_sample_rate;
let r = (-1.0 / (decay_seconds * safe_sample_rate)).exp();
```
- Good use of `max(f32::EPSILON)` to prevent division by zero

**Concern - File:** `src/voice/mod.rs` (line 14)
```rust
pub fn midi_to_hz(note: u8) -> f32 {
    440.0 * 2_f32.powf((note as f32 - 69.0) / 12.0)
}
```
- No bounds checking on `note` parameter
- Extreme values could cause issues

---

## 6. Documentation

### 6.1 Documentation Strengths

**File:** `src/dsp/exciters/felt_mallet.rs` (lines 1-14)
```rust
/// Felt Mallet - Polynomial Non-linear Contact Model
///
/// DSP Model: Soft spring hits hard wall with polynomial compression
/// Vibecode: Soft spring that hits hard wall. Low vel = linear, high vel = exponential stiffness.
///
/// Mathematical State:
/// dx = max(0, x_h - x_m)
/// F(t) = K_soft * dx + K_hard * (dx)^p
///
/// Parameters:
/// - mallet_mass: Overall momentum
/// - felt_softness (K_soft): Low-velocity stiffness (boom/thud)
/// - core_hardness (K_hard): High-velocity stiffness multiplier
/// - compression_curve (p): 3.0-5.0, how suddenly felt bottoms out
```
- Excellent documentation with physics explanation!

### 6.2 Documentation Gaps

**File:** `src/voice/mod.rs` (lines 18-52)
```rust
pub struct VoiceControls {
    pub env_attack: f32,
    pub env_decay: f32,
    // ... 30+ fields with NO documentation
}
```
- No doc comments on what these controls do
- No indication of valid ranges

**File:** `src/dsp/mseg.rs` (lines 6-44)
- MSEG struct has 25+ fields with minimal documentation
- Complex looping behavior not fully explained

**File:** `src/params.rs` (lines 8-144)
- `CorrosionParams` has 70+ parameters
- Many lack meaningful documentation beyond parameter name

---

## 7. Testing

### 7.1 Test Coverage

**Strengths:**
- ✓ Unit tests in `src/lib.rs` (lines 343-451)
- ✓ Voice tests in `src/voice/mod.rs` (lines 813-1049)
- ✓ Manager tests in `src/voice/manager.rs` (lines 148-301)
- ✓ Tests for output clamping, natural decay, pitch retargeting

**Example Good Test:**
```rust
#[test]
fn output_clamped_to_unit_range() {
    let mut voice = Voice::new();
    voice.note_on(60, 127.0, ModalProfileId::Pipe, 0, 1.0, 0.0, 0.0, 0);
    for _ in 0..48_000 {
        let sample = voice.process_sample(sample_rate);
        assert!(sample >= -1.0 && sample <= 1.0);
    }
}
```

### 7.2 Test Gaps

**Missing:**
- No tests for exciter parameter edge cases
- No tests for MSEG looping behavior
- No tests for voice stealing edge cases
- No benchmarks for real-time performance
- No tests for denormal handling

---

## 8. Specific File Reviews

### 8.1 src/lib.rs

**Lines:** 469  
**Grade:** A-

**Good:**
- Clean plugin implementation
- Proper event handling
- Good limiter implementation

**Issues:**
- Lines 75-140: Parameter mapping is verbose and manual
- Line 184-185: unwrap_or_default() hides serialization errors
- Lines 37-72: Drive algorithm has asymmetric thresholds (soft_threshold differs for positive/negative)

### 8.2 src/voice/mod.rs

**Lines:** 1050  
**Grade:** B

**Good:**
- Comprehensive voice lifecycle management
- Good envelope selection logic
- Proper NaN/Inf handling

**Issues:**
- Lines 109-157: Excessive memory usage from storing all exciters
- Lines 443-587: Magic numbers in parameter mapping
- Lines 666-696: Large match statement hurts performance
- Line 995: unwrap() on partial_cmp

### 8.3 src/voice/manager.rs

**Lines:** 302  
**Grade:** B+

**Good:**
- Clean voice stealing implementation
- Good test coverage
- Proper frame counting

**Issues:**
- Line 82: Dangerous unwrap() on partial_cmp
- Lines 11-24: Hardcoded array initialization could be more flexible

### 8.4 src/dsp/mseg.rs

**Lines:** 523  
**Grade:** B+

**Good:**
- Comprehensive MSEG implementation
- Multiple envelope types (hit, scrape, tension_rise)
- Good curve interpolation

**Issues:**
- Lines 6-44: Struct has too many fields (25+)
- Lines 414+: Test coverage could be better
- Some complex looping logic is hard to follow

### 8.5 src/dsp/resonator.rs

**Lines:** 406  
**Grade:** A-

**Good:**
- Proper modal synthesis implementation
- Good sample rate caching
- Clean SecondOrderMode implementation

**Issues:**
- Line 76: `modes: Vec<SecondOrderMode>` - allocation during creation
- Lines 100-125: Complex iterator chain could be clearer

### 8.6 Exciter Files (src/dsp/exciters/*.rs)

**Average Grade:** B+

**Good:**
- Consistent API across all exciters
- Good physics documentation
- Proper state management

**Issues:**
- Excessive boilerplate (17 nearly identical implementations)
- Some exciters (WireBrush, MetalChain) have O(N) algorithms
- No trait abstraction for common exciter behavior

---

## 9. Recommendations Summary

### Critical (Fix ASAP)
1. **Replace unwrap() on partial_cmp** - `src/voice/manager.rs:82`
2. **Add NaN checks** to peak_hold comparisons

### High Priority
1. **Reduce Voice memory footprint** - Use enum for exciter instances
2. **Document magic numbers** - Extract to named constants
3. **Optimize exciter dispatch** - Consider function pointer table
4. **Improve WireBrush/MetalChain** - Use index pointers instead of full scans

### Medium Priority
1. **Add exciter trait** - Reduce boilerplate with trait + macro
2. **Document VoiceControls** - Add doc comments for all fields
3. **Add more edge case tests** - NaN, Inf, extreme parameter values
4. **Benchmark performance** - Ensure 8 voices run at 48kHz without issues

### Low Priority
1. **Refactor parameter mapping** - Use derive macros
2. **Add module-level documentation** - Explain overall DSP architecture
3. **Consider SIMD** - For modal processing if needed

---

## 10. Security Considerations

**Status:** Generally good for an audio plugin

**Concerns:**
- State deserialization (`src/lib.rs:188-191`) uses serde_json without validation
- Could potentially crash on malformed state from malicious preset files

**Recommendation:** Add validation layer before applying deserialized parameters.

---

## 11. Maintainability Score

| Aspect | Score | Notes |
|--------|-------|-------|
| Code Organization | 8/10 | Good module structure |
| Documentation | 6/10 | Good in places, missing in others |
| Testing | 7/10 | Good coverage, some gaps |
| Consistency | 7/10 | Exciters consistent but boilerplate heavy |
| Complexity | 6/10 | MSEG and Voice are complex but manageable |
| **Overall** | **6.8/10** | Good foundation, needs polish |

---

## 12. Final Verdict

**Recommendation:** **APPROVE with improvements**

The Corrosion VST is a well-architected, feature-complete physical modeling synthesizer with solid real-time safety and good test coverage. The code is production-ready but would benefit from:

1. Fixing the critical unwrap() issue
2. Reducing memory footprint of Voice struct
3. Better documentation of parameter mappings
4. Performance optimization for exciter dispatch

The codebase shows good understanding of audio programming constraints and Rust best practices. With the recommended improvements, this would be an excellent example of a professional audio plugin.

---

*Review completed by Atlas - Master Orchestrator*
*Date: May 5, 2026*

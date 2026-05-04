use std::f32::consts::PI;

#[derive(Clone, Debug)]
pub struct ScrapeExciter {
    pressure: f32,
    speed: f32,
    roughness: f32,
    bow_position: f32,
    bow_velocity: f32,
    stick_state: f32,
    target_speed: f32,
}

impl ScrapeExciter {
    pub fn new() -> Self {
        Self {
            pressure: 0.5,
            speed: 0.3,
            roughness: 0.2,
            bow_position: 0.0,
            bow_velocity: 0.0,
            stick_state: 0.0,
            target_speed: 0.0,
        }
    }

    pub fn set_parameters(&mut self, pressure: f32, speed: f32, roughness: f32) {
        self.pressure = pressure.clamp(0.0, 1.0);
        self.speed = speed.clamp(0.0, 1.0);
        self.roughness = roughness.clamp(0.0, 1.0);
    }

    pub fn trigger(&mut self, velocity: f32) {
        self.target_speed = self.speed * (0.5 + velocity * 0.5);
        self.bow_velocity = self.target_speed * 0.1;
        self.stick_state = 0.0;
        self.bow_position = 0.0;
    }

    pub fn process_sample(&mut self, resonator_velocity: f32) -> f32 {
        let v_rel = self.bow_velocity - resonator_velocity;
        let abs_v_rel = v_rel.abs();

        let static_mu = 0.4 + self.roughness * 0.3;
        let dynamic_mu = 0.2 + self.roughness * 0.1;
        let stribeck_speed = 0.01 + self.speed * 0.02;

        let mu = if abs_v_rel < stribeck_speed {
            let t = abs_v_rel / stribeck_speed;
            static_mu - (static_mu - dynamic_mu) * t * t * (3.0 - 2.0 * t)
        } else {
            dynamic_mu + (static_mu - dynamic_mu) * (-(abs_v_rel - stribeck_speed) * 10.0).exp()
        };

        let force = self.pressure * mu * v_rel.signum();

        self.bow_velocity += (self.target_speed - self.bow_velocity) * 0.001;
        self.bow_position += self.bow_velocity;

        if self.bow_position > 1.0 {
            self.bow_position -= 1.0;
        }

        let roughness_mod = 1.0 + self.roughness * (self.bow_position * 2.0 * PI).sin() * 0.3;
        force * roughness_mod * 2.0
    }

    pub fn is_active(&self) -> bool {
        self.pressure > 0.01 && self.target_speed > 0.001
    }
}

impl Default for ScrapeExciter {
    fn default() -> Self {
        Self::new()
    }
}

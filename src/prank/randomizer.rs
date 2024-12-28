use rand::Rng;

#[must_use]
pub fn should_trigger(threshold_percent: u32) -> bool {
    let mut rng = rand::thread_rng();
    let roll = rng.gen_range(1..=100);
    roll <= threshold_percent
}

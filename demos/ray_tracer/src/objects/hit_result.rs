use super::HitResultData;

#[derive(Debug)]
pub enum HitResult {
    Hit { data: HitResultData },
    NoHit,
}
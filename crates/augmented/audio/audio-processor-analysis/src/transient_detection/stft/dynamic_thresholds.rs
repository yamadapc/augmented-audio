use super::power_change::PowerOfChangeFrames;

/// This is `λ(frame, bin)` in the paper.
///
/// It holds dynamic thresholds for each frequency bin. Dynamic thresholds are defined as a
/// factor of how much neighbouring frequency bins power-of-change is.
#[derive(Debug)]
pub struct DynamicThresholds {
    pub buffer: Vec<Vec<f32>>,
}

pub struct DynamicThresholdsParams {
    // This is `tau` on the paper
    pub threshold_time_spread: usize,
    // This is `beta` on the paper (Controls strength of transients to be extracted)
    pub threshold_time_spread_factor: f32,
}

/// Calculates `λ(frame, bin)`
pub fn calculate_dynamic_thresholds(
    params: DynamicThresholdsParams,
    power_of_change_frames: &PowerOfChangeFrames,
) -> DynamicThresholds {
    let DynamicThresholdsParams {
        threshold_time_spread: time_spread,
        threshold_time_spread_factor: beta,
    } = params;
    let mut result = {
        let mut result = Vec::with_capacity(power_of_change_frames.len());
        for _i in 0..power_of_change_frames.len() {
            result.push({
                let mut v = Vec::with_capacity(power_of_change_frames.bins());
                v.resize(power_of_change_frames.bins(), 0.0);
                v
            });
        }
        result
    };

    for i in 0..power_of_change_frames.len() {
        for j in 0..power_of_change_frames.bins() {
            let mut sum = 0.0;

            {
                let i = i as i32;
                let time_spread = time_spread as i32;

                for l in i - time_spread..i + time_spread {
                    if l >= 0 && l < power_of_change_frames.len() as i32 {
                        sum += power_of_change_frames.buffer[l as usize][j];
                    }
                }
            }

            result[i][j] = beta * (sum / (2.0 * time_spread as f32 + 1.0));
        }
    }

    DynamicThresholds { buffer: result }
}

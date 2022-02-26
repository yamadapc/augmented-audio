use super::frame_deltas::DeltaFrames;

/// This is `F(frame, bin)` in the paper. Equation `5`.
///
/// This represents how much for each frame and each bin summed with `v`
/// neighbours has changed compared with previous and next frames.
pub struct PowerOfChangeFrames {
    pub buffer: Vec<Vec<f32>>,
}

impl PowerOfChangeFrames {
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn bins(&self) -> usize {
        if self.buffer.is_empty() {
            0
        } else {
            self.buffer[0].len()
        }
    }
}

pub struct PowerOfChangeParams {
    // This is `v` on the paper
    pub spectral_spread_bins: usize,
}

fn signal(f: f32) -> f32 {
    if f >= 0.0 {
        1.0
    } else {
        -1.0
    }
}

/// Calculates `F(frame, bin)`
pub fn calculate_power_of_change(
    params: PowerOfChangeParams,
    t_results: &DeltaFrames,
) -> PowerOfChangeFrames {
    // pre-allocate result
    let mut result = {
        let mut result = Vec::with_capacity(t_results.len());
        for _i in 0..t_results.len() {
            result.push({
                let mut v = Vec::with_capacity(t_results.bins());
                v.resize(t_results.bins(), 0.0);
                v
            });
        }
        result
    };

    let spectral_spread = params.spectral_spread_bins;

    for i in 0..t_results.len() {
        result.push({
            let mut v = Vec::with_capacity(t_results.bins());
            v.resize(t_results.bins(), 0.0);
            v
        });

        for j in 0..t_results.bins() {
            let mut sum = 0.0;
            let spectral_spread = spectral_spread as i32;

            {
                let j = j as i32;
                for k in j - spectral_spread..j + spectral_spread {
                    if k >= 0 && k < t_results.bins() as i32 {
                        let minus: f32 = t_results.minus[i][k as usize];
                        let plus: f32 = t_results.plus[i][k as usize];

                        sum += (1.0 + signal(minus)) * minus + (1.0 + signal(plus) * plus);
                    }
                }
            }

            result[i][j] = 0.5 * sum;
        }
    }

    PowerOfChangeFrames { buffer: result }
}

use std::sync::Arc;

use audio_parameter_store::{ParameterStore, PluginParameter};

use crate::constants::{DEPTH_PARAMETER_ID, PHASE_PARAMETER_ID, RATE_PARAMETER_ID};

pub fn build_parameters() -> ParameterStore {
    let mut store = ParameterStore::new();
    store.add_parameter(
        RATE_PARAMETER_ID,
        Arc::new(
            PluginParameter::builder()
                .name("Rate")
                .label("Hz")
                .initial_value(1.0)
                .value_precision(1)
                // Really fun sounds when the modulation is at audio rate (over 30Hz)
                .value_range(0.05, 10.0)
                .build(),
        ),
    );
    store.add_parameter(
        DEPTH_PARAMETER_ID,
        Arc::new(
            PluginParameter::builder()
                .name("Depth")
                .initial_value(100.0)
                .label("%")
                .value_precision(0)
                .value_range(0.0, 100.0)
                .build(),
        ),
    );
    store.add_parameter(
        PHASE_PARAMETER_ID,
        Arc::new(
            PluginParameter::builder()
                .name("Phase")
                .initial_value(0.0)
                .label("º")
                .value_precision(0)
                .value_range(0.0, 360.0)
                .build(),
        ),
    );
    store
}

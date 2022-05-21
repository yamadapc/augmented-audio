// Augmented Audio: Audio libraries and applications
// Copyright (c) 2022 Pedro Tacla Yamada
//
// The MIT License (MIT)
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
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

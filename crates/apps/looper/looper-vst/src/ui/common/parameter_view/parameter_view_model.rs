use iced_audio::{knob, IntRange, NormalParam};

pub struct ParameterViewModel<ParameterId> {
    pub id: ParameterId,
    pub name: String,
    pub suffix: String,
    pub value: f32,
    pub knob_state: knob::State,
    pub int_range: Option<IntRange>,
    pub range: (f32, f32),
}

impl<ParameterId> ParameterViewModel<ParameterId> {
    pub fn new(
        id: ParameterId,
        name: String,
        suffix: String,
        value: f32,
        range: (f32, f32),
    ) -> Self {
        ParameterViewModel {
            id,
            name,
            suffix,
            value,
            knob_state: knob::State::new(NormalParam::from(
                (value - range.0) / (range.1 - range.0),
            )),
            int_range: None,
            range,
        }
    }

    pub fn snap_int(mut self) -> Self {
        let range = IntRange::new(self.range.0 as i32, self.range.1 as i32);
        self.int_range = Some(range);
        self
    }
}

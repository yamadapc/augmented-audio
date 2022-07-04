use mockall::mock;

pub struct MockVecIterator<T> {
    inner: Vec<T>,
    cursor: usize,
}

impl<T: Clone> Iterator for MockVecIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cursor >= self.inner.len() {
            None
        } else {
            let item = T::clone(&self.inner[self.cursor]);
            self.cursor += 1;
            Some(item)
        }
    }
}

impl<T> From<Vec<T>> for MockVecIterator<T> {
    fn from(v: Vec<T>) -> Self {
        MockVecIterator {
            inner: v,
            cursor: 0,
        }
    }
}

mock! {
    #[derive(Debug)]
    pub Host {}

    impl cpal::traits::HostTrait for Host {
        type Devices = MockVecIterator<MockDevice>;
        type Device = MockDevice;

        fn is_available() -> bool;

        fn devices(&self) -> Result<<MockHost as cpal::traits::HostTrait>::Devices, cpal::DevicesError>;

        fn default_input_device(&self) -> Option<<MockHost as cpal::traits::HostTrait>::Device>;

        fn default_output_device(&self) -> Option<<MockHost as cpal::traits::HostTrait>::Device>;

        fn input_devices(&self) -> Result<cpal::InputDevices<<MockHost as cpal::traits::HostTrait>::Devices>, cpal::DevicesError>;

        fn output_devices(&self) -> Result<cpal::OutputDevices<<MockHost as cpal::traits::HostTrait>::Devices>, cpal::DevicesError>;
    }
}

mock! {
    #[derive(Debug)]
    pub Device {}

    impl Clone for Device {
        fn clone(&self) -> Self;
    }

    impl cpal::traits::DeviceTrait for Device {
        type SupportedInputConfigs = MockVecIterator<cpal::SupportedStreamConfigRange>;
        type SupportedOutputConfigs = MockVecIterator<cpal::SupportedStreamConfigRange>;
        type Stream = MockStream;

        fn name(&self) -> Result<String, cpal::DeviceNameError>;

        fn supported_input_configs(
            &self,
        ) -> Result<<MockDevice as cpal::traits::DeviceTrait>::SupportedInputConfigs, cpal::SupportedStreamConfigsError>;

        fn supported_output_configs(
            &self,
        ) -> Result<<MockDevice as cpal::traits::DeviceTrait>::SupportedOutputConfigs, cpal::SupportedStreamConfigsError>;

        fn default_input_config(&self) -> Result<cpal::SupportedStreamConfig, cpal::DefaultStreamConfigError>;

        fn default_output_config(&self) -> Result<cpal::SupportedStreamConfig, cpal::DefaultStreamConfigError>;

        fn build_input_stream_raw<D, E>(
            &self,
            config: &cpal::StreamConfig,
            sample_format: cpal::SampleFormat,
            data_callback: D,
            error_callback: E,
        ) -> Result<<MockDevice as cpal::traits::DeviceTrait>::Stream, cpal::BuildStreamError>
        where
            D: FnMut(&cpal::Data, &cpal::InputCallbackInfo) + Send + 'static,
            E: FnMut(cpal::StreamError) + Send + 'static;

        fn build_output_stream_raw<D, E>(
            &self,
            config: &cpal::StreamConfig,
            sample_format: cpal::SampleFormat,
            data_callback: D,
            error_callback: E,
        ) -> Result<<MockDevice as cpal::traits::DeviceTrait>::Stream, cpal::BuildStreamError>
        where
            D: FnMut(&mut cpal::Data, &cpal::OutputCallbackInfo) + Send + 'static,
            E: FnMut(cpal::StreamError) + Send + 'static;
    }
}

mock! {
    #[derive(Debug, Clone)]
    pub Stream {}

    impl cpal::traits::StreamTrait for Stream {
        fn play(&self) -> Result<(), cpal::PlayStreamError>;
        fn pause(&self) -> Result<(), cpal::PauseStreamError>;
    }
}

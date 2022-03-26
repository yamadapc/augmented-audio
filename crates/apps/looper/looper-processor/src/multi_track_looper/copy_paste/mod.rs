use crate::parameters::ParameterId;
use crate::{LooperId, MultiTrackLooperHandle};

/// Represents a path to something that can be copy-pasted
pub enum CopyPath {
    Parameter {
        looper_id: LooperId,
        parameter: ParameterId,
    },
    Step {
        looper_id: LooperId,
        step_id: usize,
    },
    Track {
        looper_id: LooperId,
    },
}

pub enum CopyPasteError {
    UnsupportedPaths,
}

pub fn copy_paste(
    handle: &MultiTrackLooperHandle,
    source: CopyPath,
    destination: CopyPath,
) -> Result<(), CopyPasteError> {
    match (source, destination) {
        // Copying a track will copy everything related to the track:
        // * Looper in-memory buffer
        // * All parameters
        // * All steps & all parameter locks
        // * All LFO mapping
        (
            CopyPath::Track {
                looper_id: source_looper,
            },
            CopyPath::Track {
                looper_id: destination_looper,
            },
        ) => Err(CopyPasteError::UnsupportedPaths),
        // Copy step into another step will create a step with all locks in the source at destination
        (
            CopyPath::Step {
                looper_id: source_looper,
                step_id: source_step,
            },
            CopyPath::Step {
                looper_id: destination_looper,
                step_id: destination_step,
            },
        ) => {
            if let (Some(source_voice), Some(destination_voice)) = (
                handle.voices().get(source_looper.0),
                handle.voices().get(destination_looper.0),
            ) {
                let source_trigger_model = source_voice.trigger_model();
                let source_triggers = source_trigger_model.triggers();
                if let Some(source_trigger) =
                    source_trigger_model.find_step(&source_triggers, source_step)
                {
                    destination_voice
                        .trigger_model()
                        .remove_trigger(source_trigger.step());
                    destination_voice
                        .trigger_model()
                        .add_trigger(source_trigger.clone());
                }
                Ok(())
            } else {
                Err(CopyPasteError::UnsupportedPaths)
            }
        }
        // Copy from parameter to track will copy this parameter into the target tracks parameter
        // with the same ID
        (
            CopyPath::Parameter {
                looper_id: source_looper,
                parameter: source_parameter,
            },
            CopyPath::Track {
                looper_id: destination_looper,
            },
        ) => {
            if let Some(value) = handle.get_parameter(source_looper, &source_parameter) {
                handle.set_parameter(destination_looper, source_parameter, value);
            }
            Ok(())
        }
        // Copy from parameter to parameter
        (
            CopyPath::Parameter {
                looper_id: source_looper,
                parameter: source_parameter,
            },
            CopyPath::Parameter {
                looper_id: destination_looper,
                parameter: destination_parameter,
            },
        ) => {
            if let Some(value) = handle.get_parameter(source_looper, &source_parameter) {
                handle.set_parameter(destination_looper, destination_parameter, value);
            }
            Ok(())
        }
        _ => Err(CopyPasteError::UnsupportedPaths),
    }
}

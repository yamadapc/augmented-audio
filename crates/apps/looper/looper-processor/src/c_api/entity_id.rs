pub use crate::parameters::{EntityId, ParameterId};
use crate::LooperId;

#[no_mangle]
pub extern "C" fn looper_engine__entity_id__looper_parameter(
    looper_id: usize,
    parameter_id: ParameterId,
) -> EntityId {
    EntityId::EntityIdLooperParameter(LooperId(looper_id), parameter_id)
}

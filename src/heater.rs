use stop_handle::StopHandle;

pub trait Heater {
    fn start(&self);
    fn get_stop_handle(&self) -> StopHandle;
}

pub struct PodStatus {
    breaks_on   : bool,
    speed       : f32,
}

#[unsafe(no_mangle)]
pub fn retrieve_pod_status(pod_status:&mut PodStatus) {

}

#[unsafe(no_mangle)]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}


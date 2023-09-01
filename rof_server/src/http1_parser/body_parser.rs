pub struct Body<MediaType> {
    size: usize,
    payload: MediaType
}

pub enum MediaType {
    JSON
}

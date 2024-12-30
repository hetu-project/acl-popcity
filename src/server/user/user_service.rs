use rand::{distributions::Alphanumeric, Rng};
use uuid::Uuid;

pub fn gen_uid() -> String {
    Uuid::new_v4().to_string()
}

pub fn gen_invite_code(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

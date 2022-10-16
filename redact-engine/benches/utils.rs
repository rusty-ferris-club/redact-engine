use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub fn get_random_string(take: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(take)
        .map(char::from)
        .collect()
}

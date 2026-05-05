use std::sync::LazyLock;

use argon2::Argon2;

const MAX_PASSWORD_LENGTH: usize = 64;
const MIN_PASSWORD_LENGTH: usize = 8;

static ARGON2: LazyLock<Argon2<'static>> = LazyLock::new(|| { 
    Argon2::default()
});

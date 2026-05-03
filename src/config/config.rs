#[derive(Debug, Clone)]
pub struct Config {
    pub db_url: String,
    pub jwt_secret: String,
    pub jwt_maxage: i64,
    pub port: u16, 
}

impl Config {
    pub fn init() -> Self {
        let db_url = std::env::var("DATABASE_URL").expect("failed to load db url");
        let jwt_secret = std::env::var("JWT_SECRET").expect("failed to load db url");
        let jwt_maxage = std::env::var("JWT_MAXAGE")
            .unwrap_or_else(|_| "86400".to_string())
            .parse::<i64>()
            .expect("failed to load db url");
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "8000".to_string())
            .parse::<u16>()
            .expect("failed to load db url");

        Self {
            db_url,
            jwt_secret,
            jwt_maxage,
            port
        }
    }
}
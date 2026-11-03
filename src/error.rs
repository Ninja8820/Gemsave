macro_rules! errs {
    (
        $(
            $name:ident $( : $error_type:ty )?
        ),* $(,)?
    ) => {
        #[derive(Debug)]
        pub enum GemError {
            $(
                $name $(($error_type))?
            ),*
        }

        $(
            $(
                impl From<$error_type> for GemError {
                    fn from(value: $error_type) -> Self {
                        GemError::$name(value)
                    }
                }
            )?
        )*
    };
}

errs!(
    HomeNotFound,
    GameNotFound,
    SaveNotFound,
    NoGamesConfigured,
    Json: serde_json::Error,
    Io: std::io::Error,
);

impl std::fmt::Display for GemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GemError::HomeNotFound => write!(f, "Home directory not located"),
            GemError::GameNotFound => write!(f, "Game not found"),
            GemError::SaveNotFound => write!(f, "Save not found"),
            GemError::NoGamesConfigured => write!(f, "No games configured yet"),
            GemError::Io(error) => write!(f, "IOError: {}", error),
            GemError::Json(error) => write!(f, "JSONError: {}", error),
        }
    }
}

pub type Result<T> = std::result::Result<T, GemError>;
use std::{env, path::Path};

#[derive(Clone, Debug)]
pub struct GoogleOAuth {
    pub client_id: String,
    pub client_secret: String,
    pub client_redirects: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct OAuth {
    pub google: GoogleOAuth,
}

#[derive(Clone, Debug)]
pub struct Anthropic {
    pub api_keys: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct OpenAI {
    pub api_keys: Vec<String>,
    pub base_url: String,
}

#[derive(Clone, Debug)]
pub struct Llm {
    pub anthropic: Anthropic,
    pub openai: OpenAI,
}

#[derive(Clone, Debug)]
pub struct Enviroment {
    pub production: bool,
    pub host: String,
    pub port: u16,
    pub dev_port: u16,
    pub postgres_url: String,
    pub logger_level_filter: String,
    pub oauth: OAuth,
    pub llm: Llm,
}

// This is a simple function that loads the enviroment variables from the .env file
pub fn load_enviroment_vars() -> Result<Enviroment, Box<dyn std::error::Error>> {
    match env::current_dir() {
        Ok(dir) => println!("Current directory: {}", dir.display()),
        Err(e) => println!("Error getting current directory: {}", e),
    }

    let env_path = Path::new(".env");
    dotenvy::from_path(env_path)?;

    let client_redirects = dotenvy::var("GOOGLE_CLIENT_REDIRECTS")?;
    let client_redirects = client_redirects.split(",").into_iter().map(|t| t.to_string()).collect::<Vec<String>>();
    if client_redirects.len() != 2 {
        panic!("Invalid GOOGLE_CLIENT_REDIRECTS format. Should be: http://localhost:3000,https://example.com");
    }

    let anthropic_api_keys = dotenvy::var("ANTHROPIC_API_KEYS")?;
    let anthropic_api_keys = anthropic_api_keys.split(",").into_iter().map(|t| t.to_string()).collect::<Vec<String>>();
    if anthropic_api_keys.len() < 1 {
        panic!("Invalid ANTHROPIC_API_KEYS format. Should be: key1,key2");
    }

    let openai_api_keys = dotenvy::var("OPENAI_KEYS")?;
    let openai_api_keys = openai_api_keys.split(",").into_iter().map(|t| t.to_string()).collect::<Vec<String>>();
    if openai_api_keys.len() < 1 {
        panic!("Invalid OPENAI_KEYS format. Should be: key1,key2");
    }
    
    let env = Enviroment {
        production: dotenvy::var("PRODUCTION")?.parse()?,
        host: dotenvy::var("HOST")?,
        port: dotenvy::var("PORT")?.parse()?,
        dev_port: dotenvy::var("DEV_PORT")?.parse()?,
        postgres_url: dotenvy::var("POSTGRES_URL")?,
        logger_level_filter: dotenvy::var("LOGGER_LEVEL_FILTER")?,
        oauth: OAuth {
            google: GoogleOAuth {
                client_id: dotenvy::var("GOOGLE_CLIENT_ID")?,
                client_secret: dotenvy::var("GOOGLE_CLIENT_SECRET")?,
                client_redirects,
            },
        },
        llm: Llm {
            anthropic: Anthropic {
                api_keys: anthropic_api_keys,
            },
            openai: OpenAI {
                api_keys: openai_api_keys,
                base_url: dotenvy::var("OPENAI_BASE_URL")?,
            },
        },
    };
    Ok(env)
}
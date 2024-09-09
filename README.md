# Grindless

An app to grind less, performance better, with a set of specialized AI, like summary texts.

Learning and show purporse.

You are free to use and modify this :p

## Components

* `./apis`: The API code, written in Rust and Actix-Web
    * [`nervio-limiter`](https://github.com/JeanVydes/nervio-limiter) for rate limit (written by me)
    * `postgres` for db
    * `actix-web` as framework
    * `google` as auth method

* `./grindless`: The client, written in NextJS.
    * `shadcn` for components
    * `axios` for api requests
    * `tailwind` for styling
    * `google` as auth method

## Install

client can run instantly (consider change config in `./grindless/src/config.json`)

for backend you need to create certs and `.env`

### Create Certs

This is to sign tokens

`cd ./apis/grindless-core-api && chmod +x ./generate_certs.sh && ./generate_certs.sh`

### Create .env in `grindless-core-api`

`touch ./apis/grindless-core-api/.env && nano ./apis/grindless-core-api/.env`

```bash
PRODUCTION=false
HOST=0.0.0.0
PORT=80
DEV_PORT=8080
POSTGRES_URL=not_transactional_postgres
LOGGER_LEVEL_FILTER=debug

GOOGLE_CLIENT_ID=***.apps.googleusercontent.com 
GOOGLE_CLIENT_SECRET=GOCSPX-********
GOOGLE_CLIENT_REDIRECTS=https://your_production_domain.com/oauth/google/callback,http://localhost:3000/oauth/google/callback

# anthropic is used by default by api
ANTHROPIC_API_KEYS=your_keys
OPENAI_KEYS=your_keys
OPENAI_BASE_URL=https://api.openai.com/v1/
```

## Screenshots

Landing Page

### Landing Page

![landing](https://github.com/JeanVydes/grindless/blob/main/images/landing-desktop.png?raw=true)
![landing](https://github.com/JeanVydes/grindless/blob/main/images/landing-phone.png?raw=true)

### Launchpad

![launch](https://github.com/JeanVydes/grindless/blob/main/images/launchpad-desktop.png?raw=true)
![launch](https://github.com/JeanVydes/grindless/blob/main/images/launchpad-phone.png?raw=true)

### Summary Service

![ss](https://github.com/JeanVydes/grindless/blob/main/images/summary-desktop.png?raw=true)
![ss](https://github.com/JeanVydes/grindless/blob/main/images/summary-phone.png?raw=true)
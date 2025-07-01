# Async

Den nuværende implementation bruger ikke en asynkron driver til databasen. I produktion vil det være en fordel da det gør det nemmere at køre flere asynkrone jobs samtidig.

# OpenAPI/Swagger

Eftersom at Shipmondo apien exposer et openapi spec ville det i de fleste tilfælde give mening at generere http clienten ud fra det. Det har jeg dog valgt at lade være med eftersom at det ville gøre opgaven for "let".

# Secrets

I en "rigtig" applikation ville jeg foretrække at secrets var gemt I en service der gør det muligt at roterere uden at skulle redeploy,

Eventuelt [AWS Secret Manager](https://aws.amazon.com/secrets-manager/) eller [Google Secret Manager](https://cloud.google.com/security/products/secret-manager).

# Error beskeder fra externe pakker

Jeg har wrappet en del errors fra externe pakker uden at indspecificere dem (`std::io::Error`, `inquire::InquireError`, `reqwest::Error`, etc.).

Mange af disse error typer dækker over et hav af forkellige fejl. Derfor vil man ofte i "production" kode ind-specificere hvilket fejl man forventer.

---
title: Setup
desc: Setup instructions
layout: ../../layouts/MainLayout.astro
---

If you have completed the [installation steps](installation) and setup the `cargo prisma <command>` alias,
you are ready to add the Prisma Client Rust generator to your [Prisma schema](https://www.prisma.io/docs/concepts/components/prisma-schema).

A common file layout is to give the schema and migrations their own folder:

```
Cargo.toml
src/
    main.rs
prisma/
    schema.prisma
    migrations/
```

Below is an example of a schema located at `prisma/schema.prisma`.
It uses a SQLite database and generates the client at `src/prisma.rs`:

```prisma
datasource db {
    provider = "sqlite"
    url      = "file:dev.db"
}

generator client {
    // Corresponds to the cargo alias created earlier
    provider      = "cargo prisma"
    // The location to generate the client. Is relative to the position of the schema
    output        = "../src/prisma.rs"
}

model User {
    id          String  @id
    displayName String
}
```

Next, run `cargo prisma generate` to generate the client that will be used in your Rust code.
If you have `rustfmt` installed,
the generated code will be formatted for easier exploration and debugging.

NOTE: The generated client must not be checked into source control.
It cannot be transferred between devices or operating systems.
You will need to re-generate it wherever you build your project.
If using git, add it to your `.gitignore` file.

## Creating the Client

First, make sure you are using the [Tokio](https://github.com/tokio-rs/tokio) async runtime.
Other runtimes have not been tested, but since the [Prisma Engines](https://github.com/prisma/prisma-engines) use it there is likely no other option.

`PrismaClient::_builder()` provides a builder API for customising the generated client.
The simplest use of this is calling `.build()` directly on the builder.
Using the above schema for reference,
this builder creates a client in a `main.rs` file right next to `prisma.rs`:

```rust
mod prisma;

use prisma::PrismaClient;
use prisma_client_rust::NewClientError;

#[tokio::main]
async fn main() {
    let client: Result<PrismaClient, NewClientError> = PrismaClient::_builder().build().await;
}
```

The `with_url` builder method can be used to customise which database the client connects to.
In most cases it is recommended to control this with an environment variable in your schema,
but for some cases (eg. desktop apps with multiple databases) environment variables cannot be customised.

## Naming Clashes

Rust has a [reserved set of keywords](https://doc.rust-lang.org/reference/keywords.html) that cannot be used as names in your code.
If you name a model or field something that after conversion to `snake_case` will be a restricted keyword,
the generator will like give you an error.
While this is annoying, it is an unavoidable consequence of using Rust.

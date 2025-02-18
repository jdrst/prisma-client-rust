---
title: rspc Integration
layout: ../../layouts/MainLayout.astro
---

Prisma Client Rust has first-class support for [rspc](https://github.com/oscartbeaumont/rspc),
a server-agnostic router that can automatically generate TypeScript bindings.

The examples use the following schema:

```prisma
model Post {
    id        String   @id @default(cuid())
    title     String

    comments Comment[]
}

model Comment {
    id        String   @id @default(cuid())
    content   String

    post   Post   @relation(fields: [postID], references: [id])
    postID String
}
```

## Setup

When the `rspc` feature is enabled for both `prisma-client-rust` and `prisma-client-rust-cli`,
all data types generated by PCR will implement `rspc::Type`,
and a `From<queries::Error>` implementation is generated for `rspc::Error`.

## Queries

The following is possible when combining PCR and rspc:

```rust
use rspc::Router;

fn main() -> Router {
    Router::<prisma::PrismaClient>::new()
        .query("posts", |db, _: ()| async move {
            let posts = db
                .post()
                .find_many(vec![])
                .exec()
                .await?;

            Ok(posts)
        })
        .build()
}
```

When TypeScript is generated it will look something like this:

```ts
export type Operations = {
    queries: { key: ["posts"], result: Array<Post> }
};

// rspc can export types that aren't even directly used in any operations!
export interface Comment { id: string, content: string, postID: string }

export interface Post { id: string, title: string }
```

This also works for [select & include](/select-include):

```rust
use rspc::Router;

fn main() -> Router {
    Router::<prisma::PrismaClient>::new()
        .query("posts", |db, _: ()| async move {
            let posts = db
                .post()
                .find_many(vec![])
                .select(post::select!({
                    id
                    title
                }))
                .exec()
                .await?;

            Ok(posts)
        })
        .build()
}
```

The generated TypeScript will look something like this:

```ts
export type Operations = {
    // select and include types are generated as inline objects
    // rather than having a dedicated type, not even for named selections
    // since TypeScript can just infer return types
    queries: { key: ["posts"], result: Array<{ id: string, title: string}> }
};
```

### Relation types

It is important to note that the types generated for models do not have fields for relations.
This is done to provide a better experience when using `include`,
at the expense of a worse experience using `with`/`fetch` for fetching relations.

This tradeoff was made as a result of our experience building [Spacedrive](https://spacedrive.com).
Instead of all our TypeScript functions having to verify at runtime whether a relation had been loaded or not,
as is the case when using `with/fetch`,
each function has to explicitly specify the relations it wants loaded:

```ts
import { User, Post } from "./exported-types"

// Only recieves scalar fields of User
function needsOneUser(user: User) {}

// Receives scalar fields + posts relation of User
function needsOneUserWithPosts(user: User & { posts: Post[] }) {}
```

There are a few downsides to this approach:
- It is necessary to know the name and type of the relation you are including in TypeScript
  (though this is being worked on)
- `with/fetch` can be modified dynamically
- `with/fetch` provides better editor autocomplete

However, we've found that these downsides are outweighed by the benefits of having full type-safety across your backend and frontend.

## Error Handling

Since `rspc::Error` can implement `From<queries::Error>`,
all of Rust's regular error handling conventions are available.

The `?` operator can be used to extract success values and return early if an error is encountered,
automatically converting the Prisma error to an rspc error.
This has the problem of needing to wrap the extracted value in `Ok` before return from a resolver.

Another approach is adding `.map_err(Into::into)` before returning from a resolver.
This causes the Prisma error to be converted to an rspc error while keeping the Result intact,
as demonstrated below:

```rust
use rspc::Router;

fn main() -> Router {
    Router::<prisma::PrismaClient>::new()
        .query("posts", |db, _: ()| async move {
            db
                .post()
                .find_many(vec![])
                .exec()
                .await
                .map_err(Into::into)
        })
        .build()
}
```

For specifics on error handling in rpsc, see [the documentation](https://rspc.dev/server/error-handling/).

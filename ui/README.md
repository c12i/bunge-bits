# Bunge Bits

Frontend for the **Bunge Bits** project.

Built with [Remix](https://remix.run), [Tailwind CSS](https://tailwindcss.com) and [ShadCN UI](https://ui.shadcn.com).

---

## Tech Stack

- **Remix** – SSR-first React framework
- **TypeScript**
- **Tailwind CSS**
- **ShadCN UI**

## Local Development Setup

### Install Dependencies

Requires a [pnpm](https://pnpm.io/installation) installation.

```bash
pnpm install
```

### Environment Variables

Create a `.env` from the `.env.example` template

```bash
cp .env.example .env
```

### Setting up the database

Bunge Bits schema is quite fragmented at the moment. However, you can follow the following steps to
recreate the database in any environment

1. Run the [init query](../crates/stream_datastore/src/store.rs#L25-L34) from `stream_data_store`.
2. Run all the follow up migrations in [`/sql`](../sql)

With this, you should have the current `bunge_bits` database schema. Update the `DATABASE_URL` env var with yours.

### Generate Prisma Client

```bash
npx prisma generate
```

### Start the Development Server

```bash
pnpm dev
```

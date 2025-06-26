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

### Database Setup

1. Ensure you have PostgreSQL running
2. Install SQLx CLI:
   ```bash
   cargo install sqlx-cli
   ```
3. Run migrations from project root:

   ```bash
   cd crates/stream_datastore
   sqlx migrate run
   ```

   This applies migrations in `crates/stream_datastore/migrations`

4. Initialize Prisma client and run seed script:
   ```bash
   cd ui
   npx prisma generate
   pnpm seed
   ```

### Start the Development Server

```bash
pnpm dev
```

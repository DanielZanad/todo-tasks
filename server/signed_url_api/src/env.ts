import { z } from "zod";

const envSchema = z.object({
  PORT: z.coerce.number().default(3333),
  CLOUDFLARE_ENDPOINT: z.url(),
  CLOUDFLARE_ACCESS_KEY_ID: z.string(),
  CLOUDFLARE_SECRET_ACCESS_KEY: z.string(),
  DATABASE_URL: z.string(),
});

export const env = envSchema.parse(process.env);

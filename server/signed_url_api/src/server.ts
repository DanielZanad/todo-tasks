import fastify from "fastify";
import { env } from "./env.ts";
import { GetObjectCommand, PutObjectCommand } from "@aws-sdk/client-s3";
import { r2 } from "./lib/cloudflare.ts";
import { getSignedUrl } from "@aws-sdk/s3-request-presigner";
import { z } from "zod";

const server = fastify();

server.post("/uploads", async (req, rep) => {
  console.log(env);
  const uploadBodySchema = z.object({
    fileKey: z.string(),
    contentType: z.string().regex(/\w+\/[-+.\w]+/),
  });

  const { fileKey, contentType } = uploadBodySchema.parse(req.body);

  const signedUrl = await getSignedUrl(
    r2,
    new PutObjectCommand({
      Bucket: "todo-tasks",
      Key: fileKey,
      ContentType: contentType,
    }),
    { expiresIn: 600 }
  );

  return { url: signedUrl };
});

server.get("/uploads/:fileKey", async (req, rep) => {
  const getFileParamsSchema = z.object({
    fileKey: z.string(),
  });

  const { fileKey } = getFileParamsSchema.parse(req.params);

  const signedUrl = await getSignedUrl(
    r2,
    new GetObjectCommand({
      Bucket: "todo-tasks",
      Key: fileKey,
    }),
    { expiresIn: 60000 }
  );

  return { url: signedUrl };
});

server.listen({ port: env.PORT }).then(() => {
  console.log(`Server listening on port ${env.PORT}`);
});

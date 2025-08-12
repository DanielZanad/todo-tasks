# Presigned URL API

This API provides endpoints to generate secure, time-limited presigned URLs for uploading and downloading files directly to/from a Cloudflare R2 bucket. This allows clients to interact with files without needing direct access to R2 credentials.

## Endpoints

### Generate Upload URL

`POST /uploads`

Generates a presigned URL for a file upload.

#### Request Body

The endpoint expects a JSON body with the following properties:

| Field         | Type   | Description                                | Example        |
| ------------- | ------ | ------------------------------------------ | -------------- |
| `fileKey`     | string | The key (name) of the file to be uploaded. | `my-image.png` |
| `contentType` | string | The MIME type of the file.                 | `image/png`    |

#### Example Request

Here's an example of how to call the endpoint using `curl`:

```bash
curl -X POST \
  http://localhost:3000/uploads \
  -H 'Content-Type: application/json' \
  -d '{
    "fileKey": "my-profile-picture.jpg",
    "contentType": "image/jpeg"
  }'
```

### Example Success Response

A successful request will return a JSON object containing the presigned URL.

```json
{
  "url": "https://<your-r2-bucket-url>/my-profile-picture.jpg?X-Amz-Algorithm=...&X-Amz-Credential=...&X-Amz-Date=...&X-Amz-Expires=...&X-Amz-Signature=...&X-Amz-SignedHeaders=..."
}
```

### Generate Download URL

`GET /uploads/:fileKey`

Generates a presigned URL for downloading an existing file.

#### URL Parameters

| Parameter | Type   | Description                             | Example        |
| --------- | ------ | --------------------------------------- | -------------- |
| `fileKey` | string | The key (name) of the file to download. | `my-image.png` |

#### Example Request

```bash
curl http://localhost:3000/uploads/my-profile-picture.jpg
```

#### Example Success Response

A successful request will return a JSON object containing the presigned URL for the download.

```json
{
  "url": "https://<your-r2-bucket-url>/my-profile-picture.jpg?X-Amz-Algorithm=...&X-Amz-Credential=...&X-Amz-Date=...&X-Amz-Expires=...&X-Amz-Signature=...&X-Amz-SignedHeaders=..."
}
```

```

```

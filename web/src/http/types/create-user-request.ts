export type CreateUserRequest = {
  username: string;
  email: string;
  password: string;
  file_key?: File;
  mime_type?: string;
};

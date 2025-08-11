import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { useState } from "react";
import { useForm } from "react-hook-form";
import z from "zod";

const createUserSchema = z.object({
  username: z.string().min(3, { message: "min 3 characters" }),
  email: z.email(),
  password: z.string().min(3, { message: "min 3 characters" }),
  avatar: z
    .instanceof(File, { message: "You must upload an image " })
    .refine((file) => file.size > 0, { message: "file is required" }),
});

type createUserData = z.infer<typeof createUserSchema>;

export const SignUp = () => {
  const form = useForm<createUserData>();
  const [avatarPreview, setAvatarPreview] = useState<string | null>(null);

  function handleSignUpUser(data: createUserData) {
    console.log(data);
  }

  return (
    <div className="flex items-center justify-center min-h-screen">
      <Card className="w-full max-w-3xl">
        <CardHeader>
          <CardTitle>sign up</CardTitle>
          <CardDescription>
            Crie seu usuário para salvar suas tasks
          </CardDescription>
        </CardHeader>
        <CardContent>
          <Form {...form}>
            <form
              onSubmit={form.handleSubmit(handleSignUpUser)}
              className="flex justify-center  flex-col gap-4"
            >
              <div className="grid gap-8 grid-cols-2 items-start">
                <div className="flex flex-col gap-4">
                  <FormField
                    control={form.control}
                    name="email"
                    render={({ field }) => {
                      return (
                        <FormItem>
                          <FormLabel>Email da conta:</FormLabel>
                          <FormControl>
                            <Input {...field} placeholder="Digite seu email" />
                          </FormControl>
                        </FormItem>
                      );
                    }}
                  />
                  <FormField
                    control={form.control}
                    name="username"
                    render={({ field }) => {
                      return (
                        <FormItem>
                          <FormLabel>Username da conta:</FormLabel>
                          <FormControl>
                            <Input
                              {...field}
                              placeholder="Digite seu username"
                            />
                          </FormControl>
                        </FormItem>
                      );
                    }}
                  />
                  <FormField
                    control={form.control}
                    name="password"
                    render={({ field }) => {
                      return (
                        <FormItem>
                          <FormLabel>Senha:</FormLabel>
                          <FormControl>
                            <Input
                              {...field}
                              placeholder="Digite sua senha"
                              type="password"
                            />
                          </FormControl>
                        </FormItem>
                      );
                    }}
                  />
                </div>

                <div className="flex flex-col gap-2 items-center">
                  {/* Avatar Upload */}
                  <FormField
                    control={form.control}
                    name="avatar"
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>Avatar:</FormLabel>
                        <FormControl>
                          <Input
                            type="file"
                            accept="image/*"
                            name={field.name}
                            ref={field.ref}
                            onBlur={field.onBlur}
                            onChange={(e) => {
                              const file = e.target.files?.[0];
                              field.onChange(file);
                              if (file) {
                                setAvatarPreview(URL.createObjectURL(file));
                              } else {
                                setAvatarPreview(null);
                              }
                            }}
                          />
                        </FormControl>
                      </FormItem>
                    )}
                  />
                  {avatarPreview && (
                    <img
                      src={avatarPreview}
                      alt="Avatar Preview"
                      className="w-32 h-32 rounded-full object-cover border"
                    />
                  )}
                </div>
              </div>
              <Button type="submit">Criar usuário</Button>
            </form>
          </Form>
        </CardContent>
      </Card>
    </div>
  );
};

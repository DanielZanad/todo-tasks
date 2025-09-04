import { Button } from "@/components/ui/button";
import {
  Card,
  CardAction,
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
import z from "zod";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { Input } from "@/components/ui/input";
import { Link, useNavigate } from "react-router-dom";
import { useCreateSession } from "@/http/use-create-session";

const loginUserSchema = z.object({
  email: z.email(),
  password: z.string().min(3, { message: "min 3 characters" }),
});

type loginUserSchemaData = z.infer<typeof loginUserSchema>;

export const Login = () => {
  const navigate = useNavigate();
  const { mutateAsync: createSession } = useCreateSession();
  const form = useForm<loginUserSchemaData>({
    resolver: zodResolver(loginUserSchema),
    defaultValues: {
      email: "",
      password: "",
    },
  });

  async function handleUserLogin({ email, password }: loginUserSchemaData) {
    await createSession({ email, password });
    navigate("/");
  }

  return (
    <div className="flex items-center justify-center min-h-screen">
      <Card className="w-full max-w-sm">
        <CardHeader>
          <CardTitle>Login</CardTitle>
          <CardDescription>Entre com seu email</CardDescription>
          <CardAction>
            <Button variant="link">
              <Link to="/sign-up">Sign-up</Link>
            </Button>
          </CardAction>
        </CardHeader>
        <CardContent>
          <Form {...form}>
            <form
              className=" flex flex-col gap-4"
              onSubmit={form.handleSubmit(handleUserLogin)}
            >
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
                name="password"
                render={({ field }) => {
                  return (
                    <FormItem>
                      <FormLabel>Senha: </FormLabel>
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
              <Button type="submit" className="cursor-pointer">
                Login
              </Button>
            </form>
          </Form>
        </CardContent>
      </Card>
    </div>
  );
};

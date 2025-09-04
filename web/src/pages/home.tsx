import { useGetUserProfile } from "@/http/use-get-user-profile";

export const Home = () => {
  const { data, isLoading } = useGetUserProfile();

  return <div>Home{isLoading ? <p>Carregando</p> : <pre>{data}</pre>}</div>;
};

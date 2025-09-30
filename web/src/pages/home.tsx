import { Calendar } from "@/components/calendar";
import { Header } from "@/components/header";
import { useGetUserProfile } from "@/http/use-get-user-profile";

export const Home = () => {
  const { data, isLoading } = useGetUserProfile();
  console;
  return (
    <div className="">
      {isLoading ? (
        <p>Carregando...</p>
      ) : (
        data && (
          <div>
            <Header avatar_url={data.avatar_url} />
            <Calendar />
          </div>
        )
      )}
    </div>
  );
};

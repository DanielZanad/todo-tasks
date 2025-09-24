import { useGetUserProfile } from "@/http/use-get-user-profile";

export const Home = () => {
  const { data, isLoading } = useGetUserProfile();
  console;
  return (
    <div>
      Home
      {isLoading ? (
        <p>Carregando</p>
      ) : (
        <div>
          <p>{data?.username}</p>
          <img src={`${data?.avatar_url}`} alt="" />
        </div>
      )}
    </div>
  );
};

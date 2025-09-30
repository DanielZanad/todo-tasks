import { Avatar, AvatarImage } from "@radix-ui/react-avatar";
import { LogOut } from "lucide-react";

interface HeaderProps {
  avatar_url: string;
}

export const Header = ({ avatar_url }: HeaderProps) => {
  console.log(avatar_url);

  return (
    <header className=" flex p-2 m-2 justify-center items-center shadow-lg  ">
      <div>
        <Avatar>
          <AvatarImage
            className="w-14 h-14 rounded-full object-cover border"
            src={avatar_url}
          ></AvatarImage>
        </Avatar>
      </div>
    </header>
  );
};

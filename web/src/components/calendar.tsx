import { dayjs } from "@/lib/dayjs";
import {
  Carousel,
  CarouselContent,
  CarouselItem,
  CarouselNext,
  CarouselPrevious,
} from "./ui/carousel";
import { useListUserTasks } from "@/http/use-list-user-tasks";
import { DayCard } from "./dayCard";

export const Calendar = () => {
  const { data } = useListUserTasks();
  console.log(data);

  const currentYear = dayjs().year();
  const currentMonthIndex = dayjs().month();

  const months = Array.from({ length: 12 }, (_, i) => {
    return dayjs().year(currentYear).month(i);
  });

  const weekdays = ["D", "S", "T", "Q", "Q", "S", "S"];

  return (
    <div className="p-8 md:p-12">
      <Carousel
        opts={{
          startIndex: currentMonthIndex,
        }}
        className="w-full max-w-4xl mx-auto" // Center the carousel
      >
        <CarouselContent>
          {months.map((monthDate, index) => {
            const daysInMonth = monthDate.daysInMonth();
            const days = Array.from({ length: daysInMonth }, (_, i) => i + 1);
            const firstDayOfWeek = monthDate.startOf("month").day();

            return (
              <CarouselItem key={index}>
                <div className="p-4">
                  <h2 className="text-2xl font-bold text-center mb-4">
                    {monthDate.format("MMMM YYYY")}
                  </h2>

                  <div className="grid grid-cols-7 gap-2 mb-2 text-center font-semibold text-gray-500">
                    {weekdays.map((day, i) => (
                      <div key={`weekday-${i}`}>{day}</div>
                    ))}
                  </div>

                  <div className="grid grid-cols-7 gap-2 ">
                    {Array.from({ length: firstDayOfWeek }).map((_, i) => (
                      <div key={`empty-${i}`} />
                    ))}
                    {data &&
                      days.map((day) => {
                        const dayObject = monthDate.date(day);
                        const dayLetter = dayObject.format("dddd").charAt(0);

                        const dayTasks = data.filter((task) =>
                          dayjs(task.task_date).isSame(dayObject, "day")
                        );

                        const displayLimit = 9;
                        const visibleTasks = dayTasks.slice(0, displayLimit);
                        const remainingCount =
                          dayTasks.length - visibleTasks.length;

                        return (
                          <DayCard
                            allTasks={data}
                            day={day}
                            dayLetter={dayLetter}
                            visibleTasks={visibleTasks}
                            remainingCount={remainingCount}
                          />
                        );
                      })}
                  </div>
                </div>
              </CarouselItem>
            );
          })}
        </CarouselContent>
        <CarouselPrevious className="hidden sm:flex" />
        <CarouselNext className="hidden sm:flex" />
      </Carousel>
    </div>
  );
};

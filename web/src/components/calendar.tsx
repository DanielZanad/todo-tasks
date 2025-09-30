import React from "react";
import { dayjs } from "@/lib/dayjs";
import {
  Carousel,
  CarouselContent,
  CarouselItem,
  CarouselNext,
  CarouselPrevious,
} from "./ui/carousel";

export const Calendar = () => {
  const currentYear = dayjs().year();
  const currentMonthIndex = dayjs().month();

  const months = Array.from({ length: 12 }, (_, i) => {
    return dayjs().year(currentYear).month(i);
  });

  const weekdays = ["D", "S", "T", "Q", "Q", "S", "S"];

  // Mock event data
  const events = Array.from({ length: 14 }, (_, i) => ({
    id: i,
    color: "bg-green-500",
  }));
  const displayLimit = 9;
  const visibleEvents = events.slice(0, displayLimit);
  const remainingCount = events.length - displayLimit;

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
                    {weekdays.map((day) => (
                      <div key={day}>{day}</div>
                    ))}
                  </div>

                  <div className="grid grid-cols-7 gap-2">
                    {Array.from({ length: firstDayOfWeek }).map((_, i) => (
                      <div key={`empty-${i}`} />
                    ))}

                    {days.map((day) => {
                      const dayObject = monthDate.date(day);
                      const dayLetter = dayObject.format("dddd").charAt(0);

                      return (
                        <div
                          key={day}
                          className="flex flex-col gap-2 w-full h-36 p-2 md:p-4 border shadow-lg rounded-xl"
                        >
                          <div className="flex flex-row justify-between items-center">
                            <span className="text-lg font-bold">{day}</span>

                            <span className="text-sm font-semibold text-gray-400">
                              {dayLetter}
                            </span>
                          </div>

                          <div className="grid grid-cols-3 md:grid-cols-5 h-full gap-1">
                            {visibleEvents.map((event) => (
                              <div
                                key={event.id}
                                className={`w-3 h-3 rounded-full ${event.color}`}
                              ></div>
                            ))}
                            {remainingCount > 0 && (
                              <div className="flex items-center justify-center w-3 h-3 text-xs font-semibold">
                                +{remainingCount}
                              </div>
                            )}
                          </div>
                        </div>
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

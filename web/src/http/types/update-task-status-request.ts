export type UpdateTaskStatusRequest = {
  action: "next" | "previous";
  task_id: string;
};

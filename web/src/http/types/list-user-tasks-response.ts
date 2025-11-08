export type TaskResponse = {
  id: string;
  user_id: string;
  content: string;
  status: string;
  task_date: string;
  created_at: string;
};

export type ListUserTasksResponse = { tasks: Array<TaskResponse> };

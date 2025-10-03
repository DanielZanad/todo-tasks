#[derive(Debug, sqlx::Type, Clone, Copy)]
#[sqlx(type_name = "t_status", rename_all = "PascalCase")]
pub enum TaskStatus {
    ToStart,
    Started,
    Completed,
}

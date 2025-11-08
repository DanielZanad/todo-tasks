use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::Type, Clone, Copy)]
#[sqlx(type_name = "t_status", rename_all = "PascalCase")]
#[derive(Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    ToStart,
    Started,
    Completed,
}

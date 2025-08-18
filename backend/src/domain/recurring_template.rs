use sqlx::postgres::types::PgInterval;
use time::{Date, OffsetDateTime};
use uuid::Uuid;

use super::TodoName;

#[derive(Debug, Clone)]
pub struct NewRecurringTemplateRequest {
    pub todo_name: TodoName,
    pub title: String,
    pub recurrence_interval: RecurrenceInterval,
    pub start_date: Date,
    pub end_date: Option<Date>,
}

#[derive(Debug, Clone)]
pub struct UpdateRecurringTemplateRequest {
    pub title: String,
    pub recurrence_interval: RecurrenceInterval,
    pub start_date: Date,
    pub end_date: Option<Date>,
    pub is_active: bool,
}

#[derive(Debug, Clone)]
pub struct RecurringTemplate {
    pub todo_name: TodoName,
    pub template_id: Uuid,
    pub title: String,
    pub recurrence_interval: RecurrenceInterval,
    pub start_date: Date,
    pub end_date: Option<Date>,
    pub last_generated_date: Option<Date>,
    pub is_active: bool,
    pub create_time: OffsetDateTime,
    pub update_time: OffsetDateTime,
}

#[derive(Debug, Clone)]
pub struct ListRecurringTemplate {
    pub items: Vec<ListRecurringTemplateSingle>,
}

#[derive(Debug, Clone)]
pub struct ListRecurringTemplateSingle {
    pub todo_name: TodoName,
    pub template_id: Uuid,
    pub title: String,
    pub recurrence_interval: RecurrenceInterval,
    pub start_date: Date,
    pub end_date: Option<Date>,
    pub last_generated_date: Option<Date>,
    pub is_active: bool,
    pub create_time: OffsetDateTime,
    pub update_time: OffsetDateTime,
}

impl From<&RecurringTemplate> for ListRecurringTemplateSingle {
    fn from(template: &RecurringTemplate) -> Self {
        Self {
            todo_name: template.todo_name.clone(),
            template_id: template.template_id,
            title: template.title.clone(),
            recurrence_interval: template.recurrence_interval.clone(),
            start_date: template.start_date,
            end_date: template.end_date,
            last_generated_date: template.last_generated_date,
            is_active: template.is_active,
            create_time: template.create_time,
            update_time: template.update_time,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RecurrenceInterval {
    pub months: i32,
    pub days: i32,
    pub microseconds: i64,
}

impl From<&sqlx::postgres::types::PgInterval> for RecurrenceInterval {
    fn from(value: &sqlx::postgres::types::PgInterval) -> Self {
        if value.months < 0 {
            panic!("SHOULD NOT HAPPEN");
        }
        if value.days < 0 {
            panic!("SHOUD NOT HAPPEN");
        }
        if value.microseconds < 0 {
            panic!("Should not happen");
        }

        Self {
            months: value.months,
            days: value.days,
            microseconds: value.microseconds,
        }
    }
}

impl From<RecurrenceInterval> for PgInterval {
    fn from(value: RecurrenceInterval) -> Self {
        PgInterval {
            months: value.months,
            days: value.days,
            microseconds: value.microseconds,
        }
    }
}

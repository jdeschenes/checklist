use axum::{extract, http::StatusCode, Json};
use eyre::Context;
use serde::{Deserialize, Serialize};
use sqlx::Acquire;
use time::{Date, OffsetDateTime};
use tracing::info;
use uuid::Uuid;

use crate::{
    domain::{
        self, ListRecurringTemplate, NewRecurringTemplateRequest, RecurringTemplate, TodoName,
    },
    error::APIError,
    extractors::{AppRecurringSettings, DatabaseConnection},
    repos::{
        create_recurring_template, delete_recurring_template, get_recurring_template,
        list_recurring_templates, update_recurring_template,
    },
    services::generate_advance_todos_for_template,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct RecurrenceInterval {
    pub months: Option<i32>,
    pub days: Option<i32>,
    pub microseconds: Option<i64>,
}

impl From<RecurrenceInterval> for domain::RecurrenceInterval {
    fn from(value: RecurrenceInterval) -> Self {
        Self {
            months: value.months.unwrap_or(0),
            days: value.days.unwrap_or(0),
            microseconds: value.microseconds.unwrap_or(0),
        }
    }
}

impl From<domain::RecurrenceInterval> for RecurrenceInterval {
    fn from(value: domain::RecurrenceInterval) -> Self {
        Self {
            months: Some(value.months),
            days: Some(value.days),
            microseconds: Some(value.microseconds),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRecurringTemplateRequest {
    pub title: String,
    pub recurrence_interval: RecurrenceInterval,
    pub start_date: Option<Date>,
    pub end_date: Option<Date>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRecurringTemplateRequestJson {
    pub title: String,
    pub recurrence_interval: RecurrenceInterval,
    pub start_date: Option<Date>,
    pub end_date: Option<Date>,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecurringTemplateResponse {
    pub todo_name: String,
    pub template_id: Uuid,
    pub title: String,
    pub recurrence_interval: RecurrenceInterval,
    pub start_date: Date,
    pub end_date: Option<Date>,
    pub last_generated_date: Option<Date>,
    pub is_active: bool,
    #[serde(with = "time::serde::rfc3339")]
    pub create_time: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub update_time: OffsetDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListRecurringTemplatesResponse {
    pub templates: Vec<RecurringTemplateResponse>,
}

impl From<RecurringTemplate> for RecurringTemplateResponse {
    fn from(template: RecurringTemplate) -> Self {
        Self {
            todo_name: template.todo_name.into(),
            template_id: template.template_id,
            title: template.title,
            recurrence_interval: template.recurrence_interval.into(),
            start_date: template.start_date,
            end_date: template.end_date,
            last_generated_date: template.last_generated_date,
            is_active: template.is_active,
            create_time: template.create_time,
            update_time: template.update_time,
        }
    }
}

impl From<ListRecurringTemplate> for ListRecurringTemplatesResponse {
    fn from(list: ListRecurringTemplate) -> Self {
        Self {
            templates: list
                .items
                .into_iter()
                .map(|template| RecurringTemplateResponse {
                    todo_name: template.todo_name.into(),
                    template_id: template.template_id,
                    title: template.title,
                    recurrence_interval: template.recurrence_interval.into(),
                    start_date: template.start_date,
                    end_date: template.end_date,
                    last_generated_date: template.last_generated_date,
                    is_active: template.is_active,
                    create_time: template.create_time,
                    update_time: template.update_time,
                })
                .collect(),
        }
    }
}

#[tracing::instrument(
    name = "Create recurring template",
    skip(conn),
    fields(
        todo_name = %todo_name,
        title = %req.title,
    )
)]
pub async fn create_recurring_template_handler(
    DatabaseConnection(mut conn): DatabaseConnection,
    AppRecurringSettings(recurring_settings): AppRecurringSettings,
    extract::Path(todo_name): extract::Path<String>,
    Json(req): Json<CreateRecurringTemplateRequest>,
) -> Result<Json<RecurringTemplateResponse>, APIError> {
    let todo_name = TodoName::try_from(todo_name)?;

    let new_template_request = NewRecurringTemplateRequest {
        todo_name,
        title: req.title,
        recurrence_interval: req.recurrence_interval.into(),
        start_date: req
            .start_date
            .unwrap_or_else(|| OffsetDateTime::now_utc().date()),
        end_date: req.end_date,
    };

    let mut transaction = conn
        .begin()
        .await
        .context("Failed to acquire a Postgres connection from the pool")?;

    let template = create_recurring_template(&mut transaction, &new_template_request).await?;

    // Generate any todos that should be created within the advance window
    let template_single = (&template).into();
    let generated_count = generate_advance_todos_for_template(
        &mut transaction,
        &template_single,
        recurring_settings.look_ahead_duration,
    )
    .await
    .context("Failed to generate advance todos for newly created template")?;

    info!(
        "Created recurring template {} and generated {} advance todos",
        template.template_id, generated_count
    );

    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to create recurring template")?;

    Ok(Json(template.into()))
}

#[tracing::instrument(
    name = "Get recurring template",
    skip(conn),
    fields(
        todo_name = %todo_name,
        template_id = %template_id
    )
)]
pub async fn get_recurring_template_handler(
    DatabaseConnection(mut conn): DatabaseConnection,
    extract::Path((todo_name, template_id)): extract::Path<(String, Uuid)>,
) -> Result<Json<RecurringTemplateResponse>, APIError> {
    let todo_name = TodoName::try_from(todo_name)?;

    let mut transaction = conn
        .begin()
        .await
        .context("Failed to acquire a Postgres connection from the pool")?;

    let template = get_recurring_template(&mut transaction, &todo_name, &template_id).await?;

    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to get recurring template")?;

    Ok(Json(template.into()))
}

#[tracing::instrument(
    name = "Update recurring template",
    skip(conn),
    fields(
        todo_name = %todo_name,
        template_id = %template_id
    )
)]
pub async fn update_recurring_template_handler(
    DatabaseConnection(mut conn): DatabaseConnection,
    AppRecurringSettings(recurring_settings): AppRecurringSettings,
    extract::Path((todo_name, template_id)): extract::Path<(String, Uuid)>,
    Json(req): Json<UpdateRecurringTemplateRequestJson>,
) -> Result<Json<RecurringTemplateResponse>, APIError> {
    let todo_name = TodoName::try_from(todo_name)?;

    let update_request = crate::domain::UpdateRecurringTemplateRequest {
        title: req.title,
        recurrence_interval: req.recurrence_interval.into(),
        start_date: req
            .start_date
            .unwrap_or_else(|| OffsetDateTime::now_utc().date()),
        end_date: req.end_date,
        is_active: req.is_active,
    };

    let mut transaction = conn
        .begin()
        .await
        .context("Failed to acquire a Postgres connection from the pool")?;

    let template =
        update_recurring_template(&mut transaction, &todo_name, &template_id, &update_request)
            .await?;

    // Generate any todos that should be created within the advance window
    // after the template update
    let template_single = (&template).into();
    let generated_count = generate_advance_todos_for_template(
        &mut transaction,
        &template_single,
        recurring_settings.look_ahead_duration,
    )
    .await
    .context("Failed to generate advance todos for updated template")?;

    info!(
        "Updated recurring template {} and generated {} advance todos",
        template.template_id, generated_count
    );

    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to update recurring template")?;

    Ok(Json(template.into()))
}

#[tracing::instrument(
    name = "List recurring templates",
    skip(conn),
    fields(todo_name = %todo_name)
)]
pub async fn list_recurring_templates_handler(
    DatabaseConnection(mut conn): DatabaseConnection,
    extract::Path(todo_name): extract::Path<String>,
) -> Result<Json<ListRecurringTemplatesResponse>, APIError> {
    let todo_name = TodoName::try_from(todo_name)?;

    let mut transaction = conn
        .begin()
        .await
        .context("Failed to acquire a Postgres connection from the pool")?;

    let templates = list_recurring_templates(&mut transaction, &todo_name).await?;

    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to list recurring templates")?;

    Ok(Json(templates.into()))
}

#[tracing::instrument(
    name = "Delete recurring template",
    skip(conn),
    fields(
        todo_name = %todo_name,
        template_id = %template_id
    )
)]
pub async fn delete_recurring_template_handler(
    DatabaseConnection(mut conn): DatabaseConnection,
    extract::Path((todo_name, template_id)): extract::Path<(String, Uuid)>,
) -> Result<StatusCode, APIError> {
    let todo_name = TodoName::try_from(todo_name)?;

    let mut transaction = conn
        .begin()
        .await
        .context("Failed to acquire a Postgres connection from the pool")?;

    delete_recurring_template(&mut transaction, &todo_name, &template_id).await?;

    transaction
        .commit()
        .await
        .context("Failed to commit SQL transaction to delete recurring template")?;

    Ok(StatusCode::NO_CONTENT)
}

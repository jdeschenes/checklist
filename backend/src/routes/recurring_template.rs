use axum::{extract, http::StatusCode, Json};
use eyre::Context;
use serde::{Deserialize, Serialize};
use time::{Date, OffsetDateTime};
use tracing::info;
use uuid::Uuid;

use crate::{
    domain::{
        self, ListRecurringTemplate, NewRecurringTemplateRequest, RecurringTemplate, TodoName,
    },
    error::APIError,
    extractors::{AppRecurringSettings, AuthenticatedUser},
    repos::{
        create_recurring_template, delete_recurring_template, get_recurring_template,
        list_recurring_templates, update_recurring_template,
    },
    services::process_single_template,
};
use crate::tx::tx::Tx;

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
    skip(tx),
    fields(
        todo_name = %todo_name,
        title = %req.title,
    )
)]
pub async fn create_recurring_template_handler(
    mut tx: Tx,
    AppRecurringSettings(recurring_settings): AppRecurringSettings,
    AuthenticatedUser { user_id, .. }: AuthenticatedUser,
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

    let template =
        create_recurring_template(&mut tx, &new_template_request, user_id).await?;

    // Generate any todos that should be created within the advance window
    let template_single = (&template).into();
    process_single_template(
        &mut tx,
        &template_single,
        recurring_settings.look_ahead_duration,
        user_id,
    )
    .await
    .context("Failed to generate advance todos for newly created template")?;

    info!("Created recurring template {}", template.template_id);

    Ok(Json(template.into()))
}

#[tracing::instrument(
    name = "Get recurring template",
    skip(tx),
    fields(
        todo_name = %todo_name,
        template_id = %template_id
    )
)]
pub async fn get_recurring_template_handler(
    mut tx: Tx,
    AuthenticatedUser { user_id, .. }: AuthenticatedUser,
    extract::Path((todo_name, template_id)): extract::Path<(String, Uuid)>,
) -> Result<Json<RecurringTemplateResponse>, APIError> {
    let todo_name = TodoName::try_from(todo_name)?;

    let template =
        get_recurring_template(&mut tx, &todo_name, &template_id, user_id).await?;

    Ok(Json(template.into()))
}

#[tracing::instrument(
    name = "Update recurring template",
    skip(tx),
    fields(
        todo_name = %todo_name,
        template_id = %template_id
    )
)]
pub async fn update_recurring_template_handler(
    mut tx: Tx,
    AppRecurringSettings(recurring_settings): AppRecurringSettings,
    AuthenticatedUser { user_id, .. }: AuthenticatedUser,
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

    let template = update_recurring_template(
        &mut tx,
        &todo_name,
        &template_id,
        &update_request,
        user_id,
    )
    .await?;

    // Generate any todos that should be created within the advance window
    // after the template update
    let template_single = (&template).into();
    process_single_template(
        &mut tx,
        &template_single,
        recurring_settings.look_ahead_duration,
        user_id,
    )
    .await
    .context("Failed to generate advance todos for updated template")?;

    info!("Updated recurring template {}", template.template_id);

    Ok(Json(template.into()))
}

#[tracing::instrument(
    name = "List recurring templates",
    skip(tx),
    fields(todo_name = %todo_name)
)]
pub async fn list_recurring_templates_handler(
    mut tx: Tx,
    AuthenticatedUser { user_id, .. }: AuthenticatedUser,
    extract::Path(todo_name): extract::Path<String>,
) -> Result<Json<ListRecurringTemplatesResponse>, APIError> {
    let todo_name = TodoName::try_from(todo_name)?;

    let templates = list_recurring_templates(&mut tx, &todo_name, user_id).await?;

    Ok(Json(templates.into()))
}

#[tracing::instrument(
    name = "Delete recurring template",
    skip(tx),
    fields(
        todo_name = %todo_name,
        template_id = %template_id
    )
)]
pub async fn delete_recurring_template_handler(
    mut tx: Tx,
    AuthenticatedUser { user_id, .. }: AuthenticatedUser,
    extract::Path((todo_name, template_id)): extract::Path<(String, Uuid)>,
) -> Result<StatusCode, APIError> {
    let todo_name = TodoName::try_from(todo_name)?;

    delete_recurring_template(&mut tx, &todo_name, &template_id, user_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

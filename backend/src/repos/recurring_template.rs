use eyre::eyre;
use sqlx::{postgres::types::PgInterval, PgTransaction};
use std::time::Duration;
use time::Date;
use uuid::Uuid;

use crate::{
    domain::{
        ListRecurringTemplate, ListRecurringTemplateSingle, NewRecurringTemplateRequest,
        RecurringTemplate, TodoName, UpdateRecurringTemplateRequest,
    },
    error::APIError,
};

use super::get_todo_by_name;

#[derive(Debug)]
struct GetTemplateQuery {
    todo_name: String,
    template_id: Uuid,
    title: String,
    recurrence_period: sqlx::postgres::types::PgInterval,
    start_date: sqlx::types::time::Date,
    end_date: Option<sqlx::types::time::Date>,
    last_generated_date: Option<sqlx::types::time::Date>,
    is_active: bool,
    create_time: sqlx::types::time::OffsetDateTime,
    update_time: sqlx::types::time::OffsetDateTime,
}

impl TryFrom<GetTemplateQuery> for RecurringTemplate {
    type Error = APIError;

    fn try_from(value: GetTemplateQuery) -> Result<Self, Self::Error> {
        Ok(Self {
            todo_name: value.todo_name.try_into()?,
            template_id: value.template_id,
            title: value.title,
            recurrence_interval: (&value.recurrence_period).into(),
            start_date: value.start_date,
            end_date: value.end_date,
            last_generated_date: value.last_generated_date,
            is_active: value.is_active,
            create_time: value.create_time,
            update_time: value.update_time,
        })
    }
}

#[tracing::instrument(
    name = "Create recurring template in the database",
    skip(transaction, req)
)]
pub async fn create_recurring_template(
    transaction: &mut PgTransaction<'_>,
    req: &NewRecurringTemplateRequest,
) -> Result<RecurringTemplate, APIError> {
    let todo = get_todo_by_name(transaction, &req.todo_name).await?;
    let template_id = Uuid::new_v4();

    match sqlx::query_as!(
        GetTemplateQuery,
        r#"WITH insert_qry AS (
            INSERT INTO recurring_template (template_id, todo_id, title, recurrence_period, start_date, end_date)
           VALUES ($1, $2, $3, $4::interval, $5, $6)
           RETURNING template_id, todo_id, title, recurrence_period, start_date, end_date, last_generated_date, 
                     is_active, create_time, update_time)
            SELECT i.template_id, t.name as todo_name, i.title, i.recurrence_period, i.start_date, i.end_date, i.last_generated_date,
              i.is_active, i.create_time, i.update_time
            FROM insert_qry as i
            INNER JOIN todo as t ON t.todo_id = i.todo_id
            "#,
        template_id,
        todo.todo_id,
        req.title,
        Into::<PgInterval>::into(req.recurrence_interval.clone()),
        req.start_date,
        req.end_date,
    )
    .fetch_one(&mut **transaction)
    .await
    {
        Ok(result) => Ok(result.try_into()?),
        Err(sqlx::Error::RowNotFound) => Err(APIError::AlreadyExists(format!(
            "template: {} is not found",
            req.todo_name.as_ref()
        ))),
        Err(err) => Err(APIError::Internal(err.into())),
    }
}

#[tracing::instrument(
    name = "Get recurring template in the database",
    skip(transaction, todo_name, template_id)
)]
pub async fn get_recurring_template(
    transaction: &mut PgTransaction<'_>,
    todo_name: &TodoName,
    template_id: &Uuid,
) -> Result<RecurringTemplate, APIError> {
    let _ = get_todo_by_name(transaction, todo_name).await?;

    match sqlx::query_as!(
        GetTemplateQuery,
        r#"SELECT r.template_id, t.name as todo_name, r.title, r.recurrence_period, r.start_date, r.end_date, r.last_generated_date,
                  r.is_active, r.create_time, r.update_time
           FROM recurring_template as r
           INNER JOIN todo as t ON t.todo_id = r.todo_id
           WHERE t.name = $1 AND template_id = $2"#,
        todo_name.as_ref(),
        template_id,
    )
    .fetch_optional(&mut **transaction)
    .await
    {
        Ok(Some(result)) => Ok(result.try_into()?),
        Ok(None) => Err(APIError::NotFound(format!(
            "Recurring template {} not found",
            template_id
        ))),
        Err(err) => Err(APIError::Internal(err.into())),
    }
}

#[tracing::instrument(
    name = "Update recurring template in the database",
    skip(transaction, todo_name, template_id, req)
)]
pub async fn update_recurring_template(
    transaction: &mut PgTransaction<'_>,
    todo_name: &TodoName,
    template_id: &Uuid,
    req: &UpdateRecurringTemplateRequest,
) -> Result<RecurringTemplate, APIError> {
    let todo = get_todo_by_name(transaction, todo_name).await?;

    match sqlx::query_as!(
        GetTemplateQuery,
        r#"WITH update_qry as (
            UPDATE recurring_template SET
               title = $3,
               recurrence_period = $4::interval,
               start_date = $5,
               end_date = $6,
               is_active = $7
           WHERE todo_id = $1 AND template_id = $2
           RETURNING template_id, todo_id, title, recurrence_period, start_date, end_date, last_generated_date,
                     is_active, create_time, update_time)
        SELECT u.template_id, t.name as todo_name, u.title, u.recurrence_period, u.start_date, u.end_date, u.last_generated_date,
                  u.is_active, u.create_time, u.update_time
        FROM update_qry as u
        INNER JOIN todo as t ON t.todo_id = u.todo_id
        "#,
        todo.todo_id,
        template_id,
        req.title,
        Into::<PgInterval>::into(req.recurrence_interval.clone()),
        req.start_date,
        req.end_date,
        req.is_active,
    )
    .fetch_optional(&mut **transaction)
    .await
    {
        Ok(Some(result)) => Ok(result.try_into()?),
        Ok(None) => Err(APIError::NotFound(format!(
            "Recurring template {} not found",
            template_id
        ))),
        Err(err) => Err(APIError::Internal(err.into())),
    }
}

impl TryFrom<Vec<GetTemplateQuery>> for ListRecurringTemplate {
    type Error = APIError;

    fn try_from(value: Vec<GetTemplateQuery>) -> Result<Self, Self::Error> {
        let items: Result<Vec<ListRecurringTemplateSingle>, Self::Error> =
            value.into_iter().map(|i| i.try_into()).collect();
        Ok(Self { items: items? })
    }
}

impl TryFrom<GetTemplateQuery> for ListRecurringTemplateSingle {
    type Error = APIError;

    fn try_from(value: GetTemplateQuery) -> Result<Self, Self::Error> {
        Ok(Self {
            todo_name: value.todo_name.try_into()?,
            template_id: value.template_id,
            title: value.title,
            recurrence_interval: (&value.recurrence_period).into(),
            start_date: value.start_date,
            end_date: value.end_date,
            last_generated_date: value.last_generated_date,
            is_active: value.is_active,
            create_time: value.create_time,
            update_time: value.update_time,
        })
    }
}

#[tracing::instrument(
    name = "List recurring templates in the database",
    skip(transaction, todo_name)
)]
pub async fn list_recurring_templates(
    transaction: &mut PgTransaction<'_>,
    todo_name: &TodoName,
) -> Result<ListRecurringTemplate, APIError> {
    let _ = get_todo_by_name(transaction, todo_name).await?;

    match sqlx::query_as!(
        GetTemplateQuery,
        r#"SELECT r.template_id, t.name as todo_name, r.title, r.recurrence_period, r.start_date, r.end_date, r.last_generated_date,
                  r.is_active, r.create_time, r.update_time
           FROM recurring_template as r
           INNER JOIN todo as t on t.todo_id = r.todo_id
           WHERE t.name = $1
           ORDER BY create_time DESC"#,
        todo_name.as_ref(),
    )
    .fetch_all(&mut **transaction)
    .await
    {
        Ok(result) => Ok(result.try_into()?),
        Err(err) => Err(APIError::Internal(err.into())),
    }
}

#[tracing::instrument(
    name = "Delete recurring template in the database",
    skip(transaction, todo_name, template_id)
)]
pub async fn delete_recurring_template(
    transaction: &mut PgTransaction<'_>,
    todo_name: &TodoName,
    template_id: &Uuid,
) -> Result<(), APIError> {
    let todo = get_todo_by_name(transaction, todo_name).await?;

    let result = sqlx::query!(
        r#"DELETE FROM recurring_template
           WHERE todo_id = $1 AND template_id = $2"#,
        todo.todo_id,
        template_id,
    )
    .execute(&mut **transaction)
    .await?;

    match result.rows_affected() {
        0 => Err(APIError::NotFound(format!(
            "Recurring template {} not found",
            template_id
        ))),
        1 => Ok(()),
        _ => Err(APIError::Internal(
            eyre!("Multiple rows affected by delete operation").into(),
        )),
    }
}

#[tracing::instrument(name = "Get templates due for generation", skip(transaction))]
pub async fn get_templates_due_for_generation(
    transaction: &mut PgTransaction<'_>,
    current_date: Date,
    advance_duration: Duration,
) -> Result<ListRecurringTemplate, APIError> {
    // Convert Duration to PgInterval
    let advance_interval =
        PgInterval::try_from(std::time::Duration::from_secs(advance_duration.as_secs()))
            .map_err(|e| APIError::Internal(eyre!(e).into()))?;

    match sqlx::query_as!(
        GetTemplateQuery,
        r#"SELECT r.template_id, t.name as todo_name, r.title, r.recurrence_period, r.start_date, r.end_date, r.last_generated_date,
                  r.is_active, r.create_time, r.update_time
           FROM recurring_template as r
           INNER JOIN todo as t ON t.todo_id = r.todo_id
           WHERE r.is_active = TRUE
             AND (r.end_date IS NULL OR r.end_date >= ($1::date + $2::interval)::date)
             AND (r.last_generated_date IS NULL OR $1::date >= (r.last_generated_date + r.recurrence_period - $2::interval)::date)
             AND $1::date >= (r.start_date - $2::interval)::date"#,
        current_date,
        advance_interval,
    )
    .fetch_all(&mut **transaction)
    .await
    {
        Ok(result) => Ok(result.try_into()?),
        Err(err) => Err(APIError::Internal(err.into())),
    }
}

#[tracing::instrument(name = "Check if active todo exists for template", skip(transaction))]
pub async fn check_active_todo_exists_for_template(
    transaction: &mut PgTransaction<'_>,
    template_id: &Uuid,
) -> Result<bool, APIError> {
    match sqlx::query!(
        r#"SELECT EXISTS(
            SELECT 1 FROM todo_item ti
            WHERE ti.recurring_template_id = $1
            AND ti.is_complete = FALSE
        ) as exists"#,
        template_id,
    )
    .fetch_one(&mut **transaction)
    .await
    {
        Ok(result) => Ok(result.exists.unwrap_or(false)),
        Err(err) => Err(APIError::Internal(err.into())),
    }
}

#[tracing::instrument(name = "Update last generated date", skip(transaction, template_id))]
pub async fn update_last_generated_date(
    transaction: &mut PgTransaction<'_>,
    template_id: &Uuid,
    generated_date: Date,
) -> Result<(), APIError> {
    let result = sqlx::query!(
        r#"UPDATE recurring_template SET last_generated_date = $2
           WHERE template_id = $1"#,
        template_id,
        generated_date,
    )
    .execute(&mut **transaction)
    .await?;

    match result.rows_affected() {
        0 => Err(APIError::NotFound(format!(
            "Recurring template {} not found",
            template_id
        ))),
        1 => Ok(()),
        _ => Err(APIError::Internal(
            eyre!("Multiple rows affected by update operation").into(),
        )),
    }
}

use eyre::{Context, Result};
use sqlx::PgPool;
use time::OffsetDateTime;
use tracing::{error, info, warn};

use crate::{
    domain::{ListRecurringTemplateSingle, NewTodoItemRequest},
    repos::{create_todo_item, get_templates_due_for_generation, update_last_generated_date},
};

#[tracing::instrument(name = "Process recurring templates", skip(pool))]
pub async fn process_recurring_templates(pool: &PgPool) -> Result<()> {
    let current_date = OffsetDateTime::now_utc().date();

    info!(
        "Starting recurring template processing for date: {}",
        current_date
    );

    let mut transaction = pool
        .begin()
        .await
        .context("Failed to acquire database transaction")?;

    let templates = get_templates_due_for_generation(&mut transaction, current_date)
        .await
        .context("Failed to get templates due for generation")?;

    if templates.items.is_empty() {
        info!("No recurring templates due for generation");
        transaction.commit().await?;
        return Ok(());
    }

    info!(
        "Found {} templates due for generation",
        templates.items.len()
    );

    let mut generated_count = 0;
    let mut error_count = 0;

    for template in templates.items {
        match process_single_template(&mut transaction, &template).await {
            Ok(_) => {
                generated_count += 1;
                info!(
                    "Successfully generated todo item for template: {} ({})",
                    template.title, template.template_id
                );
            }
            Err(e) => {
                error_count += 1;
                error!(
                    "Failed to generate todo item for template: {} ({}): {}",
                    template.title, template.template_id, e
                );
            }
        }
    }

    transaction
        .commit()
        .await
        .context("Failed to commit recurring template processing transaction")?;

    info!(
        "Recurring template processing completed: {} generated, {} errors",
        generated_count, error_count
    );

    if error_count > 0 {
        warn!("{} templates failed to generate items", error_count);
    }

    Ok(())
}

#[tracing::instrument(
    name = "Process single recurring template",
    skip(transaction),
    fields(template_id = %template.template_id, title = %template.title)
)]
pub async fn process_single_template(
    transaction: &mut sqlx::PgTransaction<'_>,
    template: &ListRecurringTemplateSingle,
) -> Result<()> {
    let current_date = OffsetDateTime::now_utc().date();

    if current_date < template.start_date {
        warn!(
            "Template {} hasn't started yet (start_date: {}), skipping generation",
            template.template_id, template.start_date
        );
        return Ok(());
    }

    if let Some(end_date) = template.end_date {
        if current_date > end_date {
            warn!(
                "Template {} has expired (end_date: {}), skipping generation",
                template.template_id, end_date
            );
            return Ok(());
        }
    }

    let new_item_request = NewTodoItemRequest {
        title: template.title.clone(),
        due_date: current_date,
    };

    create_todo_item(transaction, &template.todo_name, &new_item_request)
        .await
        .context("Failed to create todo item from template")?;

    update_last_generated_date(transaction, &template.template_id, current_date)
        .await
        .context("Failed to update last generated date")?;

    Ok(())
}

use bareops_error::BareopsError;
use bareops_inventory::Target;
use bareops_lang::Task;

pub fn execute_tasks_on_targets(tasks: &[Task], targets: &[Target]) -> Result<(), BareopsError> {
    Ok(())
}

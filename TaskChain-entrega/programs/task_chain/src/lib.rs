use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111111");

const MAX_TITLE_LENGTH: usize = 80;
const MAX_DESCRIPTION_LENGTH: usize = 500;

#[program]
pub mod task_chain {
    use super::*;

    pub fn create_task(
        ctx: Context<CreateTask>,
        task_id: u64,
        title: String,
        description: String,
    ) -> Result<()> {
        require!(!title.is_empty(), TaskError::EmptyTitle);
        require!(title.len() <= MAX_TITLE_LENGTH, TaskError::TitleTooLong);
        require!(
            description.len() <= MAX_DESCRIPTION_LENGTH,
            TaskError::DescriptionTooLong
        );

        let task = &mut ctx.accounts.task;
        task.owner = ctx.accounts.user.key();
        task.task_id = task_id;
        task.title = title;
        task.description = description;
        task.completed = false;
        task.bump = ctx.bumps.task;

        msg!("Task created successfully");
        Ok(())
    }

    pub fn update_task(
        ctx: Context<UpdateTask>,
        title: String,
        description: String,
    ) -> Result<()> {
        require!(!title.is_empty(), TaskError::EmptyTitle);
        require!(title.len() <= MAX_TITLE_LENGTH, TaskError::TitleTooLong);
        require!(
            description.len() <= MAX_DESCRIPTION_LENGTH,
            TaskError::DescriptionTooLong
        );
        require!(!ctx.accounts.task.completed, TaskError::TaskAlreadyCompleted);

        let task = &mut ctx.accounts.task;
        task.title = title;
        task.description = description;

        msg!("Task updated successfully");
        Ok(())
    }

    pub fn complete_task(ctx: Context<CompleteTask>) -> Result<()> {
        require!(!ctx.accounts.task.completed, TaskError::TaskAlreadyCompleted);
        ctx.accounts.task.completed = true;
        msg!("Task completed");
        Ok(())
    }

    pub fn delete_task(_ctx: Context<DeleteTask>) -> Result<()> {
        msg!("Completed task deleted");
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(task_id: u64)]
pub struct CreateTask<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 8 + 4 + MAX_TITLE_LENGTH
            + 4 + MAX_DESCRIPTION_LENGTH + 1 + 1,
        seeds = [b"task", user.key().as_ref(), &task_id.to_le_bytes()],
        bump
    )]
    pub task: Account<'info, Task>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateTask<'info> {
    #[account(
        mut,
        seeds = [b"task", owner.key().as_ref(), &task.task_id.to_le_bytes()],
        bump = task.bump,
        has_one = owner
    )]
    pub task: Account<'info, Task>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct CompleteTask<'info> {
    #[account(
        mut,
        seeds = [b"task", owner.key().as_ref(), &task.task_id.to_le_bytes()],
        bump = task.bump,
        has_one = owner
    )]
    pub task: Account<'info, Task>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct DeleteTask<'info> {
    #[account(
        mut,
        close = owner,
        seeds = [b"task", owner.key().as_ref(), &task.task_id.to_le_bytes()],
        bump = task.bump,
        has_one = owner,
        constraint = task.completed @ TaskError::TaskNotCompleted
    )]
    pub task: Account<'info, Task>,
    #[account(mut)]
    pub owner: Signer<'info>,
}

#[account]
pub struct Task {
    pub owner: Pubkey,
    pub task_id: u64,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub bump: u8,
}

#[error_code]
pub enum TaskError {
    #[msg("The title cannot be empty")]
    EmptyTitle,
    #[msg("The title exceeds 80 characters")]
    TitleTooLong,
    #[msg("The description exceeds 500 characters")]
    DescriptionTooLong,
    #[msg("The task is already completed")]
    TaskAlreadyCompleted,
    #[msg("Only completed tasks can be deleted")]
    TaskNotCompleted,
}

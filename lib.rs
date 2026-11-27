use anchor_lang::prelude::*;

declare_id!("F3Ds44hs3F95kjMjjisBVUuLQaeseNagLDvqLJBVT7FP");

#[program]
pub mod proof_of_ai {
    use super::*;

    // y esta registra el hash de un contenido el q sea pero puede ser generado por IA
    pub fn register_content(
        ctx: Context<RegisterContent>,
        content_hash: [u8; 32],
        model: String,
    ) -> Result<()> {
        require!(model.len() <= 32, ProofError::ModelTooLong);

        let record = &mut ctx.accounts.record;
        record.author = ctx.accounts.author.key();
        record.content_hash = content_hash;
        record.model = model;
        record.timestamp = Clock::get()?.unix_timestamp;

        msg!("Contenido registrado por {}", record.author);
        Ok(())
    }

    // este de aca berifica que un registro existe y muestra su info
    pub fn verify_content(ctx: Context<VerifyContent>) -> Result<()> {
        let record = &ctx.accounts.record;
        msg!("Autor: {}", record.author);
        msg!("Modelo: {}", record.model);
        msg!("Fecha (unix): {}", record.timestamp);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(content_hash: [u8; 32])]
pub struct RegisterContent<'info> {
    #[account(
        init,
        payer = author,
        space = 8 + 32 + 32 + 4 + 32 + 8,
        seeds = [b"record", content_hash.as_ref()],
        bump
    )]
    pub record: Account<'info, Record>,
    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VerifyContent<'info> {
    pub record: Account<'info, Record>,
}

#[account]
pub struct Record {
    pub author: Pubkey,
    pub content_hash: [u8; 32],
    pub model: String,
    pub timestamp: i64,
}

#[error_code]
pub enum ProofError {
    #[msg("El nombre del modelo es muy largo (máx. 32 caracteres)")]
    ModelTooLong,
}

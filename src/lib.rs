use anchor_lang::prelude::*;

declare_id!("T5aJX9gswtEiUKqLP7mzhgRoZvEf7jg228N3mUDxwoS");

#[program]
pub mod instrument_registry {
    use super::*;

    pub fn inicializar_registro(ctx: Context<InicializarRegistro>) -> Result<()> {
        let registro = &mut ctx.accounts.registro;
        registro.owner = *ctx.accounts.owner.key;
        registro.inventario = Vec::new();
        Ok(())
    }

    pub fn agregar_instrumento(
        ctx: Context<GestionarInstrumento>,
        nombre: String,
        modelo: String,
        gama: String,
        color: String,
        precio: u64,
        anio: u16,
    ) -> Result<()> {
        let registro = &mut ctx.accounts.registro;

        // Validación: No registrar el mismo modelo dos veces (puedes cambiar esto por un ID único si prefieres)
        if registro.inventario.iter().any(|i| i.modelo == modelo && i.nombre == nombre) {
            return Err(ErrorCode::InstrumentoDuplicado.into());
        }

        // Validación de año (Actualizado a 2026)
        if anio < 1900 || anio > 2026 {
            return Err(ErrorCode::AnioInvalido.into());
        }

        let nuevo_instrumento = Instrumento { 
            nombre, 
            modelo, 
            gama, 
            color, 
            precio, 
            anio 
        };
        
        registro.inventario.push(nuevo_instrumento);
        msg!("Instrumento registrado exitosamente.");
        Ok(())
    }

    pub fn ver_registros(ctx: Context<VerRegistros>) -> Result<()> {
        let registro = &ctx.accounts.registro;

        if registro.inventario.is_empty() {
            msg!("El inventario de instrumentos está vacío.");
        } else {
            msg!("--- INVENTARIO DE INSTRUMENTOS ---");
            for (i, inst) in registro.inventario.iter().enumerate() {
                msg!(
                    "Instr #{}: {} {} | Gama: {} | Color: {} | Precio: ${} Dolares | Año: {}",
                    i + 1,
                    inst.nombre,
                    inst.modelo,
                    inst.gama,
                    inst.color,
                    inst.precio,
                    inst.anio
                );
            }
        }
        Ok(())
    }

    pub fn actualizar_precio(
        ctx: Context<GestionarInstrumento>, 
        modelo: String, 
        nuevo_precio: u64
    ) -> Result<()> {
        let registro = &mut ctx.accounts.registro;
        
        if let Some(inst) = registro.inventario.iter_mut().find(|i| i.modelo == modelo) {
            inst.precio = nuevo_precio;
            msg!("Precio actualizado para el modelo {}: ${}", modelo, nuevo_precio);
            Ok(())
        } else {
            Err(ErrorCode::InstrumentoNoEncontrado.into())
        }
    }

    pub fn eliminar_instrumento(ctx: Context<GestionarInstrumento>, modelo: String) -> Result<()> {
        let registro = &mut ctx.accounts.registro;
        if let Some(pos) = registro.inventario.iter().position(|i| i.modelo == modelo) {
            registro.inventario.swap_remove(pos);
            msg!("Instrumento {} eliminado del sistema.", modelo);
            Ok(())
        } else {
            Err(ErrorCode::InstrumentoNoEncontrado.into())
        }
    }
}

#[account]
#[derive(InitSpace)]
pub struct RegistroInstrumentos {
    pub owner: Pubkey,
    #[max_len(15)] // Capacidad para 15 instrumentos
    pub inventario: Vec<Instrumento>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Instrumento {
    #[max_len(30)] pub nombre: String,
    #[max_len(30)] pub modelo: String,
    #[max_len(20)] pub gama: String,
    #[max_len(20)] pub color: String,
    pub precio: u64,
    pub anio: u16,
}

#[derive(Accounts)]
pub struct InicializarRegistro<'info> {
    #[account(
        init, 
        payer = owner, 
        space = 8 + RegistroInstrumentos::INIT_SPACE, 
        seeds = [b"registro", owner.key().as_ref()], 
        bump
    )]
    pub registro: Account<'info, RegistroInstrumentos>,
    #[account(mut)] pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GestionarInstrumento<'info> {
    #[account(mut, has_one = owner)]
    pub registro: Account<'info, RegistroInstrumentos>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct VerRegistros<'info> {
    pub registro: Account<'info, RegistroInstrumentos>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("El instrumento especificado no existe.")]
    InstrumentoNoEncontrado,
    #[msg("Este instrumento ya está registrado en el inventario.")]
    InstrumentoDuplicado,
    #[msg("El año debe estar entre 1900 y 2026.")]
    AnioInvalido,
}

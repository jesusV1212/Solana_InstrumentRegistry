# 🎷 Solana Instrument Registry


Proyecto desarrollado en Rust utilizando el framework Anchor para crear un registro descentralizado de instrumentos musicales. Este sistema permite a músicos y tiendas gestionar sus activos de forma segura, inmutable y transparente dentro de la blockchain de Solana.

A diferencia de un sistema tradicional, este registro garantiza que la información no pueda ser alterada sin autorización y que siempre esté disponible para su consulta.


# 🛠️ Herramientas y Tecnologías

- Lenguaje: Rust (enfocado en seguridad de memoria).

- Framework: Anchor (estándar para Smart Contracts en Solana).

- Entorno: Solana Playground.

- Red: Devnet / Localhost.

# 📂 Estructura de Archivos

- src/lib.rs: Contiene la lógica del Smart Contract (instrucciones y validaciones).

- README.md: Guía general del proyecto y documentación para el usuario.

# 📌 Instrucciones de Uso

## 1. Inicializar el Sistema de Registro

Antes de agregar instrumentos, se debe crear el espacio de almacenamiento en la blockchain ejecutando la función inicializar_registro.

- PDA (Program Derived Address): El sistema utiliza una dirección derivada basada en la semilla "registro" y la publicKey del administrador (owner). Esto asegura que cada usuario tenga su propio inventario único.

## 2. Registrar un Nuevo Instrumento

Para dar de alta un equipo, utiliza la función agregar_instrumento. El sistema validará que el año sea coherente y que no existan duplicados.

- Datos requeridos: * nombre: (Ej. Saxofón)

- modelo: (Ej. Yamaha)

- gama: (Ej. Profesional)

- color: (Ej. Dorado)

- precio: (ej. 1000)

- anio: (ej. 2000)

## 3. Gestión y Mantenimiento (CRUD)

- Actualizar Precio: Mediante la función actualizar_precio, se puede modificar el valor de mercado de un instrumento buscando específicamente por su modelo.

- Eliminar Registro: La función eliminar_instrumento permite remover un equipo del inventario si este ha sido vendido o ya no requiere seguimiento.

## 📋 Ejemplo de Resultado (JSON)

Cuando consultas las cuentas (Accounts) de tu programa tras un registro exitoso, verás la información organizada de esta manera:

```json

{
  "owner": "5gCBRqTzWsgyXD5WEewheGtm8LLZG6mH5oB2kk2xZ4vh",
  "inventario": [
    {
      "nombre": "saxofon",
      "modelo": "yamaha",
      "gama": "profesional",
      "color": "dorado",
      "precio": "1000",
      "anio": 2000
    },
    {
      "nombre": "clariente",
      "modelo": "bundi",
      "gama": "estudiante",
      "color": "negro",
      "precio": "10000",
      "anio": 2010
    }
  ]
}
```
  

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
  

# 🗂️ Arquitectura de Persistencia Descentralizada

Este sistema trasciende el uso de bases de datos convencionales (SQL/NoSQL) para implementar un modelo de Cuentas Derivadas del Programa (PDAs). Esta arquitectura garantiza que cada nodo de información (el inventario de instrumentos musicales) esté mapeado de forma biyectiva a la identidad criptográfica del administrador. Esto permite que instrumentos como saxofones, guitarras o teclados queden vinculados permanentemente a su dueño en la blockchain.

# 🔑 Program Derived Address (PDA)

La localización de los activos en la red no depende de un ID incremental (como en una base de datos tradicional), sino de una derivación criptográfica compuesta por:

- Semilla Operativa: El prefijo constante "registro", que define el espacio de nombres del contrato y organiza el almacenamiento.

- Firmante Autorizado: La publicKey del owner, lo que asegura que solo el creador del registro tenga privilegios de escritura y modificación sobre el inventario de instrumentos.

# ⚙️ Administración de Espacio y Memoria

- Para optimizar el costo de almacenamiento (renta en Lamports) y asegurar la eficiencia de la red, el programa utiliza especificaciones técnicas rigurosas:

- Tipado Estricto: Se utilizan cadenas de texto (String) con límites de longitud predefinidos (max_len) para los campos de nombre, modelo, gama y color. Esto evita el desperdicio de espacio en la cuenta de Solana.

- Eficiencia Numérica: El uso de u64 para el precio y u16 para el año permite manejar valores precisos ocupando el mínimo de bytes posible.

- Inventario Dinámico: Se implementa un vector (Vec) con una capacidad máxima de 15 unidades (definida mediante InitSpace). Esto permite agrupar múltiples instrumentos en una sola cuenta, optimizando significativamente los costos de transacción y almacenamiento al evitar la creación de múltiples cuentas individuales.


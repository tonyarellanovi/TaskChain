# TaskChain

Aplicación descentralizada para administrar tareas en Solana Devnet.

## Funcionalidades del programa
- Crear tareas asociadas a la wallet del usuario.
- Guardar título, descripción y estado en una cuenta PDA.
- Editar título y descripción mientras la tarea esté pendiente.
- Marcar tareas como completadas.
- Cerrar/eliminar únicamente tareas completadas.
- Validar que solamente el propietario pueda modificar su tarea.

## Tecnologías
- Solana
- Rust
- Anchor
- TypeScript para pruebas
- Solana Playground para compilar y desplegar desde el navegador

## Estado de esta entrega
Este repositorio contiene el código fuente y la documentación base. **No afirma que el programa ya esté desplegado en Devnet.** Para completar esa evidencia se debe compilar, ejecutar las pruebas y desplegar desde Solana Playground. El saldo de SOL de prueba y los límites del faucet son externos al repositorio.

## Estructura
```text
TaskChain/
├── Anchor.toml
├── Cargo.toml
├── README.md
├── programs/
│   └── task_chain/
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs
└── tests/
    └── task_chain.ts
```

## Programa en Solana Playground
El código principal está en `programs/task_chain/src/lib.rs`. En Solana Playground, el archivo equivalente es `src/lib.rs`.

El identificador que aparece en `declare_id!` debe coincidir con el Program ID generado por el keypair del programa. No inventes ni copies el identificador de otra persona. Después de generar el keypair, actualiza `declare_id!` y `Anchor.toml` con el mismo valor.

## Compilar y desplegar
En Solana Playground:
1. Abre el proyecto Anchor.
2. Pega el contenido de `programs/task_chain/src/lib.rs` en `src/lib.rs`.
3. Usa **Build** y revisa que termine correctamente.
4. Selecciona **Devnet** y conecta la wallet de desarrollo.
5. Cuando tengas SOL de prueba suficiente, usa **Deploy**.
6. Guarda el Program ID y el enlace de la transacción/explorador como evidencia.

## Seguridad
- Nunca publiques archivos de keypair, frases de recuperación, claves privadas ni credenciales.
- No subas carpetas `target/`, `node_modules/` ni archivos `.env`.
- Usa únicamente SOL de prueba en Devnet.

## Evidencias que faltan por agregar
- Captura del Build exitoso.
- Captura del resultado de las pruebas.
- Program ID desplegado en Devnet.
- Enlace de Solana Explorer a la transacción de despliegue.
- Capturas de las operaciones crear, editar, completar y eliminar.

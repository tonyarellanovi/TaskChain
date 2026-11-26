# TaskChain

## Descripción

TaskChain es una aplicación descentralizada de gestión de tareas desarrollada sobre la blockchain de Solana. Su propósito es permitir que los usuarios creen, consulten, editen, completen y eliminen sus tareas mediante un programa inteligente desarrollado con Rust y Anchor.

## Objetivo

Desarrollar una aplicación que utilice tecnología blockchain para gestionar tareas de manera descentralizada, segura y transparente.

## Tecnologías utilizadas

- **Solana:** plataforma blockchain.
- **Rust:** lenguaje utilizado para desarrollar el programa.
- **Anchor:** framework para desarrollar programas en Solana.
- **TypeScript:** lenguaje utilizado para las pruebas.
- **Solana Playground:** entorno de desarrollo y compilación.

## Funcionalidades

1. **Crear tareas:** permite registrar una tarea con título y descripción.
2. **Editar tareas:** permite modificar el título y la descripción.
3. **Completar tareas:** permite cambiar el estado de una tarea a completada.
4. **Eliminar tareas:** permite eliminar únicamente tareas que ya estén completadas.

## Estructura del proyecto

```text
TaskChain-entrega/
├── programs/
│   └── task_chain/
│       ├── src/
│       │   └── lib.rs
│       └── Cargo.toml
├── tests/
│   └── task_chain.ts
├── Anchor.toml
├── Cargo.toml
├── package.json
└── tsconfig.json
```

## Desarrollo y pruebas

El programa fue desarrollado utilizando Rust y Anchor en Solana Playground.

La compilación del programa se realizó correctamente (`Build successful`).

Se prepararon pruebas para verificar las operaciones de creación, edición, finalización y eliminación de tareas.

**Estado de las pruebas:** las transacciones no pudieron completarse debido a que la wallet de Devnet no contaba con SOL de prueba suficiente.

## Estado del proyecto

- Código del programa: desarrollado.
- Compilación: exitosa.
- Pruebas: preparadas, pendientes de ejecución completa.
- Despliegue en Solana Devnet: pendiente.
- Interfaz web: pendiente de desarrollo e integración.

## Identificador del programa

```text
C2P3ExfWbpq7scVKJXKxFWhwM8TAnQpDz5xMXJoNmUjK
```

Este identificador está configurado en el código fuente. El despliegue todavía está pendiente de realizarse en Dev

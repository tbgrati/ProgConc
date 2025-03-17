# TP1 - Programacion Concurrente

### 1. ¿Qué sucede con dos requests simultáneas que tardan en procesarse?
Cuando dos requests llegan al servidor al mismo tiempo. Primero se procesa una y despues la siguente. Si la primera tarda mucho en ejecutarse la segunda request se vera demorada

### 2. ¿Por qué se observa este comportamiento?
Este comportamiento ocurre porque las requests se manejan de manera secuencial.
Cada request se atiende una por una, bloqueando la ejecución de nuevas requests hasta que la actual finalice.

### 3. ¿Cómo solucionar usando solo librerías estándar de Rust?
Se puede solucionar utilizando múltiples hilos (`std::thread`) para manejar cada conexión en su propio hilo de ejecución.
Esto permite que varias requests se procesen simultáneamente.


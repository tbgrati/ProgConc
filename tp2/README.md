# TP2 - Programacion Concurrente

### Resultados de Apache Benchmark

500 requests, variando la concurrencia

#### Concurrency 1 (-c 1)
Requests per second: 63.34
Mean request time: 15.787 ms
#### Concurrency 5 (-c 5)
Requests per second: 127.05
Mean request time: 39.356 ms
#### Concurrency 10 (-c 10)
Requests per second: 112.14
Mean request time: 89.178 ms
#### Concurrency 20 (-c 20)
Requests per second: 99.16
Mean request time: 201.7 ms

A medida que la concurrencia aumenta la velocidad (mean request time) y los request time mejoran hasta cierto punto, luego decae debido al intentar manejar varios threads 


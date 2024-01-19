# Project 1

Make sure to read the [problem](./problem.pdf) for more information about what the program solves.

## How to run
To compile and run the project you can seperately
```bash
make build
make run
```

## How to test
Benchmarking was required for the [report](report.pdf) you can configure the [config](config.txt) file to specify number of instances, starting size, etc...
Then run
```bash
make generate
```
This will generate random `.in` files with the desired settings with the help of the [generator](data/gen_supermarble.cpp).
To benchmark each instance run
```bash
make measure
```
A `.in.time` file will be generated for each instance containing useful information using `perf stat`
> [!NOTE]  
> The measure target will ask for sudo permission to run the benchmarks with max priority

# Project 1

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
This will generate random `.in` files with the desired settings with the help of the [generator](data/generator.cpp).
To benchmark each instance run
```bash
make measure
```
A `.in.time` file will be generated for each instance containing useful information using `perf stat`
> [!NOTE]  
> The measure target will ask for sudo permission to run the benchmarks with max priority

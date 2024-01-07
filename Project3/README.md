# Project 2 

## How to run
Make sure to install the PIP dependencies and then run `make run`
```bash
pip install -r requirements.txt
make run
```

## How to test
Benchmarking was required for the [report](report.pdf) you can configure the [config](config.txt) file to specify number of instances, starting size, etc...
Then run
```bash
make generate
```
This will generate random `.in` files with the desired settings with the help of the [generator](data/gen_ubiquity.cpp).
To benchmark each instance run
```bash
make measure
```
A `.in.time` file will be generated for each instance containing useful information using `perf stat`
> [!NOTE]  
> The measure target will ask for sudo permission to run the benchmarks with max priority

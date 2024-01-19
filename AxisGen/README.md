# AxisGen
## What is it?
Simple python tool that helps collect benchmarking information.

Reads all .time files in the specified directory and parses the total elapsed time and input parameter for each instance. <br>
Collects the data and prints in this format:
```
Time Array: t = [t1, t2, ..., tn]
InputParameters Array: s = [s1, s2, ..., sn]
```

> [Desmos](https://www.desmos.com/calculator) was chosen for the graph generation. You can simply copy-paste the output and it will create those two arrays in the app.

## How to run
Simply run:
```bash
python3 gen_axis.py <instances_dir_path>
```
- `instances_dir_path` -> Path for the instances we want to collect 

> [!Warning]
> A **Makefile** is provided for each project. It automatically runs this script when the `measure` target is ran

import os
import re
import sys


def extract_elapsed_time(file_path):
    with open(file_path, 'r') as file:
        lines = file.readlines()

    elapsed_time_line = lines[17]
    match = re.search(r'([\d.]+) seconds time elapsed', elapsed_time_line)
    
    if match:
        return float(match.group(1))
    else:
        return None
    
def extract_size(filename):
    match = re.search(r'_(\d+)\.in.time$', filename)
    if match:
        return int(match.group(1))
    else:
        return None

def main(instance_dir):
    elapsed_times = []
    sizes = []

    files = sorted(os.listdir(instance_dir), key=lambda x: int(x.split('_')[1]))

    for filename in files:
        if filename.endswith(".time"):
            file_path = os.path.join(instance_dir, filename)
            elapsed_time = extract_elapsed_time(file_path)
            size = extract_size(filename)

            if elapsed_time is not None and size is not None:
                elapsed_times.append(elapsed_time)
                sizes.append(size)

    print("t =", elapsed_times)
    print("s =", sizes)

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python3 gen_axis.py <INSTANCE_DIR>")
    else:
        inst_dir = sys.argv[1]
        main(inst_dir)

import argparse

parser = argparse.ArgumentParser()
parser.add_argument("day", help="day number", type=int)

if __name__ == "__main__":
    args = parser.parse_args()
    with open("include.txt", "r") as f:
        txt = f.read().replace("$DAY", str(args.day))
        with open(f"../include/day_{args.day}.h", "w") as f:
            f.write(txt)
    with open("src.txt", "r") as f:
        txt = f.read().replace("$DAY", str(args.day))
        with open(f"../src/day_{args.day}.cpp", "w") as f:
            f.write(txt)
    print("finished.")

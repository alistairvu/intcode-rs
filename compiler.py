import sys


def main():
    if len(sys.argv) < 1:
        print("Usage: python compiler.py <filename>")

    filename = sys.argv[1]

    with open(filename, 'r') as f:
        lines = [x.strip() for x in f.readlines()]
        values = []

        for line in lines:
            values += [int(x) for x in line.split(',') if len(x)]

        output_filename = filename.replace(".in", ".bin")
        output_file = open(output_filename, 'wb')

        for byte in values:
            try:
                output_file.write(byte.to_bytes(
                    8, byteorder="little", signed=True))
            except OverflowError:
                print(f"Overflow: {byte}")
                return

        output_file.close()
        print(output_filename)


if __name__ == '__main__':
    main()

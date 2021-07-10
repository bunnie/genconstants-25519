#! /usr/bin/env python3

import argparse
import subprocess
import sys

def main():
    parser = argparse.ArgumentParser(description="Sign binary images for Precursor")
    parser.add_argument(
        "--input", required=False, help="file to convert", type=str, nargs='?', const='constants.rs'
    )
    args = parser.parse_args()

    if args.input == None:
        args.input = 'constants.rs'

    with open(args.input, "r") as input:
        with open('constants_gen.rs', "wb") as output:
            consts = [0, 0, 0, 0, 0]
            getting_consts = False
            const_index = 0
            for line in input:
                if getting_consts:
                    consts[const_index] = line.strip().strip(',')
                    const_index += 1
                    if const_index == 5:
                        getting_consts = False
                        const_index = 0
                        array = subprocess.check_output(['./target/debug/genconstants.exe', consts[0], consts[1], consts[2], consts[3], consts[4], ]).decode('utf8')
                        a = array.replace('[', '')
                        b = a.replace(']', '')
                        output.write(b.encode('utf-8'))
                else:
                    if "FieldElement51" in line:
                        r = line.replace("FieldElement51", "Engine25519")
                        output.write(r.encode('utf-8'))
                        if "super" not in line:
                            getting_consts = True
                    else:
                        output.write(line.encode('utf-8'))

if __name__ == "__main__":
    main()
    exit(0)

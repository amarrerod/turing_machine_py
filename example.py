from turing_machine_py import *


if __name__ == "__main__":
    tm = load_from_instance("example1.tm", "example1.tape")
    tape = tm.run()
    print(tape)
    print(tm)

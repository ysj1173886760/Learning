##############################################################
# Do not modify! (But feel free to use the functions provided)
##############################################################
import os, subprocess, unittest, tempfile
from collections import defaultdict
from pathlib import Path
from typing import List, Optional, Set

a_regs = {f"a{i}" for i in range(8)}

# find venus jar
_script_dir = Path(os.path.dirname(__file__)).resolve()
_root_dir = _script_dir / '..'
_venus_jar = _root_dir / 'tools' / 'venus.jar'
assert _venus_jar.is_file(), f"Could not find venus.jar at {_venus_jar}"

# --immutableText: immutable text, ensures that code cannot be modified
# --maxsteps -1: no upper bound on the number of cycles
_venus_default_args = ['--immutableText', '--maxsteps', '-1']

def run_venus(filename: str, check_calling_convention: bool = True, extra_flags: Optional[List[str]] = None, args: Optional[List[str]] = None, verbose: bool = False):
    assert os.path.isfile(filename), f"{filename} not found, cannot run venus"
    # print(filename)
    cmd = ['java', '-jar', _venus_jar] + _venus_default_args
    if check_calling_convention:
        cmd += ['--callingConvention']
    # print(" ".join((str(c) for c in cmd)))

    with tempfile.TemporaryDirectory() as tmp_dir:
        coverage_file = Path(tmp_dir) / 'coverage'
        cmd +=  ['--coverageFile', coverage_file.absolute()]
        if extra_flags is not None: cmd += extra_flags
        cmd += [filename]
        if args is not None: cmd += args
        if verbose: print("Executing: " +" ".join(str(c) for c in cmd))
        r = subprocess.run(cmd, stdout=subprocess.PIPE, cwd=_root_dir, stderr=subprocess.PIPE)
        try:
            with open(coverage_file) as c:
                coverage = c.read()
        except FileNotFoundError:
            coverage = ""
    return r, coverage

# global coverage dictionary
# maps filename -> line -> count
_global_coverage = defaultdict(lambda: defaultdict(lambda : 0))

def _process_coverage(coverage: str, file_filter: str):
    for line in coverage.split('\n'):
        if len(line.strip()) == 0: continue
        p = line.strip().split(' ')
        assert len(p) == 3, f"Unexpected coverage line {line}. Do you have a space in the filename or path?"
        import_path, line = p[1].split(':')
        filename = os.path.basename(import_path)
        if filename != file_filter: continue
        _global_coverage[filename][int(line)] += int(p[2])

def print_coverage(filename: str, verbose: bool = True):
    if filename not in _global_coverage:
        print(f"No coverage numbers found for `{filename}`")
    else:
        cov = _global_coverage[filename]
        line_count = len(cov)
        covered_count = sum(c > 0 for c in cov.values())
        print()
        print(f"Coverage for `{filename}`: {covered_count}/{line_count}")
        if verbose:
            for line, count in cov.items():
                print(f"{filename}:{line}\t{count}")

# all test files will be under unittests/assembly
_test_dir = _script_dir / 'assembly'
_test_suffix = '.s'

def save_assembly(name: str, src: str, verbose: bool) -> str:
    # create test directory if it does not already exist
    if not _test_dir.is_dir():
        os.mkdir(_test_dir)
    filename = _test_dir / (name + _test_suffix)
    with open(filename, 'w') as f:
        f.write(src)
    if verbose: print(f"Wrote test to file: {filename}")
    return filename

def _indent(lines: List[str]) -> List[str]:
    return [f"    {l}" if len(l.strip()) > 0 else l for l in lines]

_source_dir = (_root_dir / 'src').resolve()

def _read_lines(filename: str) -> List[str]:
    with open(filename) as _f:
        _res = _f.read().split('\n')
    return _res

class ArrayData:
    # represents an array in the data section
    def __init__(self, name: str, init: List[int]):
        self.name = name
        self.init = init

    def __len__(self):
        return len(self.init)

_inputs_dir = _root_dir / 'inputs'
assert _inputs_dir.is_dir()
_outputs_dir = _root_dir / 'outputs'
assert _outputs_dir.is_dir()

class FileName:
    # represents an input or output filename, relative to the unittests directory
    def __init__(self, name: str, is_input: bool):
        self.name = name
        self.is_input = is_input

def _test_id_to_name(test: unittest.TestCase) -> str:
    parts = test.id().split('.')
    assert len(parts) == 3, f"Unexpected test id: {test.id()}"
    return f"{parts[1]}_{parts[2]}"

class AssemblyTest:
    """ represents a single assembly test """
    def __init__(self, test: unittest.TestCase, assembly: str, check_calling_convention: bool = True, no_utils: bool = False):
        self.name = _test_id_to_name(test)
        self._test = test
        self.data: List[str] = []
        self._checks: List[str] = []
        self._args: List[str] = []
        self._call: Optional[str] = None
        self._imports: List[str] = []
        self._array_count: int = 0
        self._msg_count: int = 0
        self._labels: Set[str] = set()
        self._output_regs: Set[int] = set()
        self._arrays: dict = {}
        self._assembly = assembly
        self._program_executed = False
        self._write_files: Set[str] = set()
        self._std_out: Optional[str] = None
        self.check_calling_convention = check_calling_convention

        if not no_utils: self.include('utils.s')
        self.include(assembly)

    def include(self, name: str):
        filename = _source_dir / name
        assert filename.is_file(), f"{filename} does not exist"
        self._imports.append(name)

    def call(self, function: str):
        """ Specifies which function to call. Remember to provide any input with the `input` method. """
        assert self._call is None, f"Can only call one function per test! Already called {self._call}"
        self._call = function

    # This function puts the arguments into the unittest which is nice because then you can just
    # copy the test to venus, however we recommend students use the optional `args` argument to
    # the `execute` method instead.
    def _input_args(self, args: List[str]):
        """ Provides command line arguments through the a0 (argc) and a1 (argv) registers. """
        assert self._call is None, f"You need to specify all inputs before calling `{self._call}`"
        assert isinstance(args, list), f"{args} is a {type(args)}, expected a list of strings!"
        assert len(args) > 0, f"Expected a non-empty argument list!"
        assert all(isinstance(a, str) for a in args), f"Expected a list of strings, not {[type(a) for a in args]}!"
        # all arguments could potentially be filenames that we write to, so let's just add them
        self._write_files |= set(args)
        # add dummy argument zero
        args = [""] + args
        # allocate args in memory
        arg_strings = [self._str(a, "arg") for a in args]
        # allocate a pointer array for argv
        self.data += [f"argv: .word " + " ".join("0" for _ in range(len(args)))]
        # load argc and argv
        self._args += ["", "# argument count in a0", f"li a0, {len(args)}"]
        self._args += ["", "# load pointers to argument strings into argv", f"la a1, argv"]
        for ii, aa in enumerate(arg_strings):
            self._args += [f"la t1, {aa}", f"sw t1, {ii * 4}(a1)"]

    def input_scalar(self, register: str, value: int):
        """ Provides a scalar input through an "a" register """
        assert self._call is None, f"You need to specify all inputs before calling `{self._call}`"
        assert register in a_regs, f"Register {register} must be one of the a registers!"
        assert isinstance(value, int), f"{value} is a {type(value)}, expected an int!"
        self._args += ["", f"# load {value} into {register}", f"li {register} {value}"]

    def input_array(self, register: str, value: ArrayData):
        """ Provides an array input through an "a" register """
        assert self._call is None, f"You need to specify all inputs before calling `{self._call}`"
        assert register in a_regs, f"Register {register} must be one of the a registers!"
        assert isinstance(value, ArrayData), f"{value} is a {type(value)}, expected an array (created with the array([..]) method!"
        name = self._lookup_array(value)
        self._args += ["", f"# load address to array {name} into {register}", f"la {register} {name}"]

    def input_read_filename(self, register: str, filename: str):
        """ Provides a filename string input through an "a" register """
        full_path = _root_dir / filename
        if not full_path.is_file():
            print(f"WARN: Input file {full_path} does not exist.")
        self._input_filename(register, filename)

    def input_write_filename(self, register: str, filename: str):
        """ Provides a filename string input through an "a" register """
        dir_path = (_root_dir / filename).parent
        if not dir_path.is_dir():
            print(f"Creating directory: {dir_path}")
            dir_path.mkdir(parents=True, exist_ok=True)
        self._write_files.add(filename)
        self._input_filename(register, filename)

    def _input_filename(self, register: str, filename: str):
        assert self._call is None, f"You need to specify all inputs before calling `{self._call}`"
        assert register in a_regs, f"Register {register} must be one of the a registers!"
        path = self._str(filename)
        self._args += ["", f"# load filename {filename} into {register}", f"la {register} {path}"]

    def check_scalar(self, register: str, value: int):
        """ checks the the value of register """
        assert self._call is not None, f"You must first call a function before checking its return values!"
        assert isinstance(value, int), f"{value} is a {type(value)}, expected an int!"
        exit_code = 8
        saved_register = self._parse_register(register)
        lbl = self._make_lbl(f"{register}_eq_{value}")
        msg = f"msg{self._msg_count}"
        self._msg_count += 1
        self.data += [f"{msg}: .asciiz \"expected {register} to be {value} not: \""]
        self._checks += [
            "", f"# check that {register} == {value}",
            f"li t0 {value}", f"beq {saved_register} t0 {lbl}",
            "# print error and exit",
            f"la a1, {msg}", "jal print_str",
            f"mv a1 {saved_register}", "jal print_int",
            "# Print newline", "li a1 '\\n'", "jal ra print_char",
            f"# exit with code {exit_code} to indicate failure",
            f"li a1 {exit_code}", "jal exit2",
            f"{lbl}:", ""
        ]

    def check_array(self, array: ArrayData, value: List[int]):
        """ checks the the value of an array in memory """
        assert self._call is not None, f"You must first call a function before checking its return values!"
        assert len(value) > 0, "Array to compare against has to contain at least one element."
        assert len(value) <= len(array), "Array to compare against must contain a smaller or equal amount of elements."
        expected = self.array(value).name
        actual = "la a2, " + self._lookup_array(array)
        self._compare_int_array(array.name, actual, expected, value, exit_code = 2)

    def check_array_pointer(self, register: str, value: List[int]):
        """ check the memory region pointed to by the register content """
        assert self._call is not None, f"You must first call a function before checking its return values!"
        assert len(value) > 0, "Array to compare against has to contain at least one element."
        saved_register = self._parse_register(register)
        array_name = f"array pointed to by {register}"
        expected = self.array(value).name
        actual = f"mv a2 {saved_register}"
        self._compare_int_array(array_name, actual, expected, value, exit_code = 2)

    def check_file_output(self, actual: str, expected: str):
        """ compares the actual file to the expected file """
        assert self._program_executed, f"You first need to `execute` the program before checking its outputs!"
        assert actual in self._write_files, f"Unknown output file {actual}. Did you forget to provide it to the program by calling input_write_filename?"
        full_expected = _root_dir / expected
        assert full_expected.is_file(), f"Reference file {full_expected} does not exist!"
        # check to make sure the output file exists
        full_actual = _root_dir / actual
        self._test.assertTrue(full_actual.is_file(), f"It seems like the program never created the output file {full_actual}")
        # open and compare the files
        with open(full_actual, 'rb') as a:
            actual_bin = a.read()
        with open(full_expected, 'rb') as e:
            expected_bin = e.read()
        self._test.assertEqual(actual_bin, expected_bin, f"Bytes of {actual} and {expected} did not match!")

    def check_stdout(self, expected: str):
        """ compares the output of the program """
        assert self._std_out is not None, f"You first need to `execute` the program before checking stdout!"
        self._test.assertEqual(self._std_out.strip(), expected.strip())

    def _parse_register(self, register: str) -> str:
        assert register in a_regs, "Only a registers can be checked"
        register_index = int(register[1:])
        assert register_index not in self._output_regs, f"Register {register} was already checked!"
        self._output_regs.add(register_index)
        return f"s{register_index}"

    def _compare_int_array(self, array_name: str, actual: str, expected: str, value: List[int], exit_code: int):
        value_str = " ".join(str(v) for v in value)
        msg = self._str(f"expected {array_name} to be:\\n{value_str}\\nInstead it is:\\n")
        self._checks += [
            "",
            "##################################",
            f"# check that {array_name} == {value}",
            "##################################",
            "# a0: exit code", f"li a0, {exit_code}",
            "# a1: expected data", f"la a1, {expected}",
            "# a2: actual data", actual,
            "# a3: length", f"li a3, {len(value)}",
            "# a4: error message", f"la a4, {msg}",
            "jal compare_int_array",
        ]

    _can_fail = {'fopen', 'fclose', 'fread', 'fwrite', 'malloc', ''}
    def execute(self, code: int = 0, args: Optional[List[str]] = None, fail: str = "", verbose: bool = False):
        """ Assembles the test and runs it through the venus simulator. """
        assert fail in AssemblyTest._can_fail, f"Invalid fail={fail}. Can only fail: {list(AssemblyTest._can_fail)}"

        # turn function to fail into a define
        if len(fail) == 0:
            defines = []
        else:
            ret = 0 if fail == 'malloc' else -1
            defines = ["--def", f"#{fail.upper()}_RETURN_HOOK=li a0 {ret}"]

        # check arguments
        if args is not None:
            # TODO: check to see if any args clash with venus arguments
            assert len(args) > 0, "use None if you don't want to pass any arguments"
            for a in args:
                assert not a.startswith('-'), f"argument '{a}' starting with '-' is not allowed"
            # all arguments could potentially be filenames that we write to, so let's just add them
            self._write_files |= set(args)
        else:
            # ensure that args is always a list
            args = []

        lines = []

        lines += [f".import ../../src/{i}" for i in self._imports]
        lines += ["", ".data"] + self.data
        lines += ["", ".globl main_test", ".text", "# main_test function for testing", "main_test:"]

        # prologue
        if len(self._output_regs) > 0:
            assert len(self._output_regs) < 13, f"Too many output registers: {len(self._output_regs)}!"
            p = ["# Prologue", f"addi sp, sp, -{4 * (len(self._output_regs) + 1)}", "sw ra, 0(sp)"]
            p += [f"sw s{i}, {(i+1) * 4}(sp)" for i in range(len(self._output_regs))]
            lines += _indent(p + [""])


        lines += _indent(self._args)

        assert self._call is not None, "No function was called!"
        foo_call = ["", f"# call {self._call} function", f"jal ra {self._call}"]
        lines += _indent(foo_call)

        if len(self._output_regs) > 0:
            lines += _indent(["", "# save all return values in the save registers"])
            lines += _indent([f"mv s{i} a{i}" for i in self._output_regs] + [""])

        lines += _indent(self._checks)
        if code != 0:
            lines += _indent([f"# we expect {self._call} to exit early with code {code}"])

        lines += _indent(["", "# exit normally"])
        # epilogue
        if len(self._output_regs) > 0:
            p = ["# Epilogue", "lw ra, 0(sp)"]
            p += [f"lw s{i}, {(i + 1) * 4}(sp)" for i in range(len(self._output_regs))]
            p += [f"addi sp, sp, {4 * (len(self._output_regs) + 1)}"]
            lines += _indent(p + [""])
        # lines += _indent(["mv a0, zero", "ret"])
        lines += _indent(["jal exit"])
        lines += [""]

        if verbose: print()
        filename = save_assembly(self.name, '\n'.join(lines), verbose=verbose)
        r, coverage = run_venus(filename, self.check_calling_convention, defines, args, verbose=verbose)
        _process_coverage(coverage, self._assembly)
        self._program_executed = True
        self._std_out = r.stdout.decode('UTF-8')
        if r.returncode != code:
            self._print_failure(r, code)

    def _print_failure(self, r, expected_code):
        venus_out = r.stdout.decode('UTF-8') + "\n" + r.stderr.decode('UTF-8')
        if expected_code == 0:
            self._test.fail(f"Unexpected results from venus ({r.returncode}):\n{venus_out}")
        else:
            self._test.fail(f"Venus returned exit code {r.returncode} not {expected_code}.\n{venus_out}")

    def _make_lbl(self, prefix: str) -> str:
        name = prefix
        ii = 0
        while name in self._labels:
            name = f"{prefix}_{ii}"
            ii += 1
        self._labels.add(name)
        return name

    def _lookup_array(self, a: ArrayData) -> str:
        assert a.name in self._arrays, f"Unknown array {a.name}. Did you declare it for this test?"
        assert self._arrays[a.name] is a, f"Array {a.name} was declared with a different test!"
        return a.name

    def array(self, data: List[int]) -> ArrayData:
        name = f"m{self._array_count}"
        self._array_count += 1
        self.data += [f"{name}: .word " + " ".join((str(v) for v in data))]
        a = ArrayData(name, data)
        self._arrays[a.name] = a
        return a

    def _str(self, data: str, prefix: str ="msg") -> str:
        name = f"{prefix}{self._msg_count}"
        self._msg_count += 1
        self.data += [f"{name}: .asciiz \"{data}\""]
        return name

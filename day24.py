import z3

WORD = 32
ZERO = z3.BitVecVal(0, WORD)
ONE = z3.BitVecVal(1, WORD)

def build_solver(program):
    solver = z3.Optimize()
    inputs = []
    regfile = {r: ZERO for r in "wxyz"}
    for i, instr in enumerate(program):
        op, dst, *src = instr
        if op == "inp":
            inp = z3.BitVec(f"in_{len(inputs)}", WORD)
            solver.add(z3.And(1 <= inp, inp <= 9))
            regfile[dst] = inp
            inputs.append(inp)
            continue

        ret = z3.BitVec(f"ret_{i}", WORD)
        lhs, rhs = regfile[dst], regfile[src[0]] if src[0] in regfile else int(src[0])
        if op == "add":
            solver.add(ret == lhs + rhs)
        elif op == "mul":
            solver.add(ret == lhs * rhs)
        elif op == "mod":
            solver.add(lhs >= 0)
            solver.add(rhs > 0)
            solver.add(ret == lhs % rhs)
        elif op == "div":
            solver.add(rhs != 0)
            solver.add(ret == lhs / rhs)
        elif op == "eql":
            solver.add(ret == z3.If(lhs == rhs, ZERO, ONE))
        else:
            raise RuntimeError(f"unknown instruction {op}")
        regfile[dst] = ret

    solver.add(regfile["z"] == 0)
    return solver, inputs


if __name__ == "__main__":
    with open("inputs/day24.txt") as f:
        program = [l.strip().split() for l in f]
        solver, inputs = build_solver(program)
        valid_input = sum(10**k * z3.ZeroExt(64, d) for k, d in enumerate(inputs[::-1]))
        for i, opt in enumerate([solver.maximize, solver.minimize]):
            solver.push()
            solver.maximize(valid_input)
            solver.check()
            print(f"part {i+1}:", solver.model().eval(valid_input))
            solver.pop()

from dataclasses import dataclass
import copy

@dataclass
class State:
    a: int
    b: int
    c: int

def combo(state, operand):
    match operand:
        case x if 0 <= x <= 3:
            return x
        case 4:
            return state.a
        case 5:
            return state.b
        case 6:
            return state.c
        case 7:
            raise Exception('bad combo operand')

def run(state, program):
    state = copy.copy(state)
    out = []
    ip = 0
    while ip < len(program):
        opcode = program[ip]
        operand = program[ip+1]
        # print(state)
        # print(opcode)
        match opcode:
            case 0:
                # adv: a = a / (2^combo)
                divisor = 2**combo(state, operand)
                state.a = state.a // divisor
            case 1:
                # bxl: b = b ^ literal
                state.b = state.b ^ operand
            case 2:
                # bst: b = combo & 0b111
                state.b = combo(state, operand) & 0b111
            case 3:
                # jnz: if a == 0: pass; else ip = literal; jump
                if state.a == 0:
                    pass
                else:
                    ip = operand
                    continue
            case 4:
                # bxc: state.b = state.b ^ state.c
                state.b = state.b ^ state.c
            case 5:
                # out: out combo & 0b111
                out.append(combo(state, operand) & 0b111)
            case 6:
                # bdv: b = a / combo
                divisor = 2**combo(state, operand)
                state.b = state.a // divisor
            case 7:
                # cdv: c = a / combo
                divisor = 2**combo(state, operand)
                state.c = state.a // divisor
        ip = ip + 2
    return (state, out)

ex = '''Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0'''

s0 = State(729, 0, 0)
p0 = [0,1,5,4,3,0]

# print(run(State(12,0,0), [0,2]))
# print(run(State(0,0,9), [2,6]))
print(run(State(10,0,0), [5,0,5,1,5,4]))
#print(run(s0, p0))
s1 = State(30886132, 0, 0)
p1 = [2,4,1,1,7,5,0,3,1,4,4,4,5,5,3,0]
#print(run(s1, p1))
_, out = run(s1, p1)
print(','.join(map(str, out)))

def p2(s, p):
    a = 0
    s2 = copy.copy(s)
    while True:
        s2.a = a
        _, out = run(s2, p)
        if out == p:
            break
        a = a + 1
    return a

# print(p2(State(2024,0,0), [0,3,5,4,3,0]))
#print(p2(s1, p1))


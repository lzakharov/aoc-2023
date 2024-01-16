from sympy import Symbol, solve_poly_system

x = Symbol('x')
y = Symbol('y')
z = Symbol('z')
vx = Symbol('vx')
vy = Symbol('vy')
vz = Symbol('vz')

equations = []
ts = []

with open('input/day24.txt') as f:
    for i in range(3):
        position, velocity = f.readline().strip().split(" @ ")
        (x_i, y_i, z_i) = map(int, position.split(", "))
        (vx_i, vy_i, vz_i) = map(int, velocity.split(", "))

        t = Symbol(f't_{i}')
        ts.append(t)

        equations.append(x + vx*t - x_i - vx_i*t)
        equations.append(y + vy*t - y_i - vy_i*t)
        equations.append(z + vz*t - z_i - vz_i*t)

result = solve_poly_system(equations, x, y, z, vx, vy, vz, *ts)
print(sum(result[0][:3])) 

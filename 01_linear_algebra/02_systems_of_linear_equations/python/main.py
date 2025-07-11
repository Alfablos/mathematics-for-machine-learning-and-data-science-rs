import cupy as np
import matplotlib.pyplot as plt


print('Solve:\n{\n  -x + 3y = 7;\n  3x + 2y = 1;\n}')

A = np.array([[-1, 3], [3, 2]], dtype=np.dtype(float))
b = np.array([7, 1], dtype=np.dtype(float))

print('Matrix A:')
print(A)
print('Matrix B:')
print(b)


result = np.linalg.solve(A, b)
print('Solution: x, y = ' + str(result))


# Since the matrix is 2x2 (square) let's calculate the determinant:
det = np.linalg.det(A)
if det != 0:
    print('The system has, in fact, exactly one solution')
else:
    print('The system does not have a unique solution')



# Elimination method  (Gauss)

reshaped_b = b.reshape((2,1))

whole_matrix = np.hstack((A, reshaped_b))

print('Whole matrix:')
print('Row 1: ' + str(whole_matrix[0]))
print('Row 2: ' + str(whole_matrix[1]))


result_matrix = whole_matrix.copy()

# R2 = 3 * R1 + R2
print('Executing: R2 = 3 * R1 + R2')
result_matrix[1] = 3 * whole_matrix[0] + whole_matrix[1]

print('Intermediate result:')
print(result_matrix)

# R2 = 1/11 * R2
print('Executing R2 = 1/11 * R2')
result_matrix[1] = 1/11 * result_matrix[1]

print('Solution:')
print(result_matrix)



def plot_lines(M):
    x_1 = np.linspace(-10,10,100)
    x_2_line_1 = (M[0,2] - M[0,0] * x_1) / M[0,1]
    x_2_line_2 = (M[1,2] - M[1,0] * x_1) / M[1,1]
    
    _, ax = plt.subplots(figsize=(10, 10))
    ax.plot(x_1, x_2_line_1, '-', linewidth=2, color='#0075ff',
        label=f'$x_2={-M[0,0]/M[0,1]:.2f}x_1 + {M[0,2]/M[0,1]:.2f}$')
    ax.plot(x_1, x_2_line_2, '-', linewidth=2, color='#ff7300',
        label=f'$x_2={-M[1,0]/M[1,1]:.2f}x_1 + {M[1,2]/M[1,1]:.2f}$')

    A = M[:, 0:-1]
    b = M[:, -1::].flatten()
    d = np.linalg.det(A)

    if d != 0:
        solution = np.linalg.solve(A,b) 
        ax.plot(solution[0], solution[1], '-o', mfc='none', 
            markersize=10, markeredgecolor='#ff0000', markeredgewidth=2)
        ax.text(solution[0]-0.25, solution[1]+0.75, f'$(${solution[0]:.0f}$,{solution[1]:.0f})$', fontsize=14)
    ax.tick_params(axis='x', labelsize=14)
    ax.tick_params(axis='y', labelsize=14)
    ax.set_xticks(np.arange(-10, 10))
    ax.set_yticks(np.arange(-10, 10))

    plt.xlabel('$x_1$', size=14)
    plt.ylabel('$x_2$', size=14)
    plt.legend(loc='upper right', fontsize=14)
    plt.axis([-10, 10, -10, 10])

    plt.grid()
    plt.gca().set_aspect("equal")

    plt.show()
    
    
plot_lines(result_matrix)


print('-------------------')


print('Solve:\n{\n  -x + 3y = 7;\n  3x - 9y = 1;\n}')

A_2 = np.array([
        [-1, 3],
        [3, -9]
    ], dtype=np.dtype(float))

b_2 = np.array([7, 1], dtype=np.dtype(float))

d_2 = np.linalg.det(A_2)

print(f"Determinant of matrix A_2: {d_2:.2f}")

try:
  solutions = np.linalg.solve(A_2, b_2)
  print('Solutions:' + str(solutions))
except:
  print('The system does not have a unique solution.')


import cupy as np





onedim = np.array([1, 2, 3])
print(onedim)

onedim_range = np.arange(4) # 0, 1, 2, 3
print(onedim_range)

onedim_range_from1 = np.arange(1, 4) # 0, 1, 2, 3
print(onedim_range_from1)

onedim_range_with_step = np.arange(1, 30, 2)    # step is 2; all odds < 30
print(onedim_range_with_step)


# linspace defaults to float as type, forcing it to int
onedim_equally_spaced = np.linspace(0, 20, 5, dtype=int) # Print 5 numbers from 0 to 20 so that they're all equally distant from the preceding
print(onedim_equally_spaced)

onedim_zeros = np.zeros(5)
print(onedim_zeros)
onedim_ones = np.ones(5, dtype=int)
print(onedim_ones)
onedim_empty = np.empty(5)
print(onedim_empty)
onedim_random = np.random.rand(5)
print(onedim_random)


## Multidimentional ##
twodim = np.array([[1, 2, 3], [4, 5, 6]])
print(twodim)

initial_onedim = np.array([1, 2, 3, 4, 5, 6])
twodim = np.reshape(initial_onedim, (2,3))  # create a 2x3 from 1D array
print(twodim)

print(f'Twodim is a {twodim.shape} ({twodim.ndim}-dimensional) array with {twodim.size} elements.')


## Arrays math operations ##
arr1 = np.array([2, 4, 6])
arr2 = np.array([1, 3, 5])

addition = arr1 + arr2
subtraction = arr1 - arr2
multiplication = arr1 * arr2

print(f'{arr1} + {arr2} = {addition}')
print(f'{arr1} - {arr2} = {subtraction}')
print(f'{arr1} * {arr2} = {multiplication}')

arr3 = np.array([2, 5, 7])
vect_by_scalar = 1.4 * arr3
print(f'{arr3} * 1.4 = {vect_by_scalar}')


## Indexing in 1D ##
print(f'{arr3} index 2 = {arr3[2]}')

## Indexing in 2D
tw = np.array(([1, 2, 3],
          [4, 5, 6],
          [7, 8, 9]))
print(f'{tw} index 1 and then 2 = {tw[1][2]}')
print(f'Last two elements of {tw} = {tw[1:]}')


## Array stacking ##
a1 = np.array([[1,1],
               [2,2]])
a2 = np.array([[3,3],
              [4,4]])
print(f'original a1:\n{a1}')
print(f'original a2:\n{a2}')

vstack = np.vstack((a1, a2))
print(f'Vertically stacked: {vstack}')

hstack = np.hstack((a1, a2))
print(f'Horizontally stacked: {hstack}')


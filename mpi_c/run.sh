mpicc -o integral_mpi integral_mpi.c -lm

rm output.txt

mpirun -np 1 ./integral_mpi 100
mpirun -np 2 ./integral_mpi 100
mpirun -np 3 ./integral_mpi 100
mpirun -np 4 ./integral_mpi 100
mpirun -np 5 ./integral_mpi 100
mpirun -np 6 ./integral_mpi 100

mpirun -np 1 ./integral_mpi 1000
mpirun -np 2 ./integral_mpi 1000
mpirun -np 3 ./integral_mpi 1000
mpirun -np 4 ./integral_mpi 1000
mpirun -np 5 ./integral_mpi 1000
mpirun -np 6 ./integral_mpi 1000

mpirun -np 1 ./integral_mpi 10000
mpirun -np 2 ./integral_mpi 10000
mpirun -np 3 ./integral_mpi 10000
mpirun -np 4 ./integral_mpi 10000
mpirun -np 5 ./integral_mpi 10000
mpirun -np 6 ./integral_mpi 10000

mpirun -np 1 ./integral_mpi 100000
mpirun -np 2 ./integral_mpi 100000
mpirun -np 3 ./integral_mpi 100000
mpirun -np 4 ./integral_mpi 100000
mpirun -np 5 ./integral_mpi 100000
mpirun -np 6 ./integral_mpi 100000

mpirun -np 1 ./integral_mpi 1000000
mpirun -np 2 ./integral_mpi 1000000
mpirun -np 3 ./integral_mpi 1000000
mpirun -np 4 ./integral_mpi 1000000
mpirun -np 5 ./integral_mpi 1000000
mpirun -np 6 ./integral_mpi 1000000

mpirun -np 1 ./integral_mpi 10000000
mpirun -np 2 ./integral_mpi 10000000
mpirun -np 3 ./integral_mpi 10000000
mpirun -np 4 ./integral_mpi 10000000
mpirun -np 5 ./integral_mpi 10000000
mpirun -np 6 ./integral_mpi 10000000

mpirun -np 1 ./integral_mpi 100000000
mpirun -np 2 ./integral_mpi 100000000
mpirun -np 3 ./integral_mpi 100000000
mpirun -np 4 ./integral_mpi 100000000
mpirun -np 5 ./integral_mpi 100000000
mpirun -np 6 ./integral_mpi 100000000
 
mpirun -np 1 ./integral_mpi 1000000000
mpirun -np 2 ./integral_mpi 1000000000
mpirun -np 3 ./integral_mpi 1000000000
mpirun -np 4 ./integral_mpi 1000000000
mpirun -np 5 ./integral_mpi 1000000000
mpirun -np 6 ./integral_mpi 1000000000

mpirun -np 1 ./integral_mpi 10000000000
mpirun -np 2 ./integral_mpi 10000000000
mpirun -np 3 ./integral_mpi 10000000000
mpirun -np 4 ./integral_mpi 10000000000
mpirun -np 5 ./integral_mpi 10000000000
mpirun -np 6 ./integral_mpi 10000000000

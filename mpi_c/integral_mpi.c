/* C Example */
#include <mpi.h>
#include <math.h>
#include <stdio.h>
#include <time.h>
#include <stdlib.h>

double fct(double x)
{
      return cos(x);
}

/* Prototype */
double integral(double a, int n, double h);

void main(argc,argv)
int argc;
char *argv[];
{
/***********************************************************************
 *                                                                     *
 * This is one of the MPI versions on the integration example          *
 * It demonstrates the use of :                                        *
 *                                                                     *
 * 1) MPI_Init                                                         *
 * 2) MPI_Comm_rank                                                    *
 * 3) MPI_Comm_size                                                    *
 * 4) MPI_Recv                                                         *
 * 5) MPI_Send                                                         *
 * 6) MPI_Finalize                                                     *
 * 7) MPI_allreduce													   *
 *                                                                     *
 ***********************************************************************/
      long int n, i, j, ierr,num;
      double h, result, a, b, pi, global_sum;
      double my_a, my_range;
      time_t start, end;

      int myid, source, dest, tag, p, duration;
      MPI_Status status;
      double my_result;

      pi = acos(-1.0);  /* = 3.14159... */
      a = 0.;           /* lower limit of integration */
      b = pi*1./2.;     /* upper limit of integration */
      if (argc == 2) {
        n = atol(argv[1]);
      } else if (argc == 1) {
        n = 100000000;          /* number of increment within each process */
      }

      dest = 0;         /* define the process that computes the final result */
      tag = 123;        /* set the tag to identify this particular job */

/* Starts MPI processes ... */

      MPI_Init(&argc,&argv);              /* starts MPI */
      MPI_Comm_rank(MPI_COMM_WORLD, &myid);  /* get current process id */
      MPI_Comm_size(MPI_COMM_WORLD, &p);     /* get number of processes */

      h = (b-a)/n;    /* length of increment */
      num = n/p;	/* number of intervals calculated by each process*/
      my_range = (b-a)/p;
      my_a = a + myid*my_range;
		
	  start = clock();  
      my_result = integral(my_a,num,h);
      printf("Process %d / %d has the partial result of %f\n", myid+1, p, my_result);

      if(myid == 0) {
        result = my_result;
        printf("integral = %f\n", my_result);
        for (i=1;i<p;i++) {
          source = i;           /* MPI process number range is [0,p-1] */
          MPI_Recv(&my_result, 1, MPI_DOUBLE, source, tag,
                        MPI_COMM_WORLD, &status);
          result += my_result;
          printf("integral (Master) = %f\n", result);
        //printf("The result =%f\n",result);
       }
      }
      else
      {
        MPI_Send(&my_result, 1, MPI_DOUBLE, dest, tag,
                      MPI_COMM_WORLD);      /* send my_result to intended dest.
                      */
        
        printf("integral (Process %d) = %f\n", myid, my_result);
	}
	  
	  end = clock();
	  printf("durasi (Process %d) = %f \n", myid, (double)(end-start)/CLOCKS_PER_SEC); 
      //MPI_Reduce(&my_result, &global_sum, 1, MPI_DOUBLE, MPI_SUM, 0, MPI_COMM_WORLD);	
      //the parameters are = send_var, recv_var, no_array, data_type, operation, send_to, world
 
      //if (myid == 0)
      //{
		  //end = clock();
		  //duration = end - start;
		  //printf("The result = %f and time = %d\n", global_sum, duration);
	  //}
	  
      MPI_Finalize();                       /* let MPI finish up ... */
      if (myid == 0) {  
            FILE *file = fopen("output.txt", "a");
            if (file == NULL) {
                  perror("Error opening file");
            } else {
                  fprintf(file, "%d %ld %d %f %f\n", p, n, myid, (double)(end-start)/CLOCKS_PER_SEC, result);
                  fclose(file);
            }
      }
}

double integral(double a, int n, double h)
{
      int j;
      double h2, aij, integ;

      integ = 0.0;                 /* initialize integral */
      h2 = h/2.;
      for (j=0;j<n;j++) {          /* sum over all "j" integrals */
        aij = a + j*h;      	   /* lower limit of "j" integral */
        integ += fct(aij+h2)*h;
      }
      return (integ);
}

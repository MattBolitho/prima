// An example to illustrate the use of BOBYQA.

#include "prima/prima.h"
#include <stdio.h>
#include <math.h>

static void fun(const double x[], double *f, const void *data)
{
  const double x1 = x[0];
  const double x2 = x[1];
  *f = pow(x1-5, 2) + pow(x2-4, 2);
  (void)data;
}

static void callback(int n, const double x[], double f, int nf, int tr, double cstrv, const int m_nlcon, const double nlconstr[], bool *terminate)
{
  (void)n;
  (void)cstrv;
  (void)m_nlcon;
  (void)nlconstr;
  printf("best point so far: x={%g, %g} f=%g nf=%d tr=%d\n", x[0], x[1], f, nf, tr);
  *terminate = 0;
}

int main(int argc, char * argv[])
{
  (void)argc;
  (void)argv;
  const int n = 2;
  double x0[2] = {0.0, 0.0};
  // set up the problem
  prima_problem_t problem;
  prima_init_problem(&problem, n);
  problem.x0 = x0;
  problem.calfun = &fun;
  // Define the lower and upper bounds. We define an upper bound that will be active
  // in order to demonstrate the usage of bounds.
  double xl[] = {-1.0, -1.0};
  double xu[] = {4.5, 4.5};
  problem.xl = xl;
  problem.xu = xu;
  // set up the options
  prima_options_t options;
  prima_init_options(&options);
  options.iprint = PRIMA_MSG_EXIT;
  options.rhoend= 1e-3;
  options.maxfun = 200*n;
  options.callback = &callback;
  // initialize the result
  prima_result_t result;
  // run the solver
  const int rc = prima_minimize(PRIMA_BOBYQA, &problem, &options, &result);
  printf("x*={%g, %g} rc=%d msg='%s' evals=%d\n", result.x[0], result.x[1], rc, result.message, result.nf);
  int success = (fabs(result.x[0]-4.5)>2e-2 || fabs(result.x[1]-4)>2e-2);
  prima_free_result(&result);
  return success;
}

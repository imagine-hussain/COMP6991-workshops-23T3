#include <stdio.h>
int main(int argc, char *argv[]) {
  int x[10] = {};
  int r = x[1000000000000];
  printf("r = %d\n", r);

  return 1;
}

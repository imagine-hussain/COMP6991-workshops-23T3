
int i = 0;
void inc() {
  for (int j = 0; j < 100000; j++) {
    //
    //  (tmp, i)  | (tmp, i)
    //  (0, 0)    | (0, 0)
    //
    // LOCK STASRTS HRE - ATOMIC ZONE
    // zone of "mutual exclusion"
    int temp = i;
    //  (1, 0)    | (1, 0)
    temp = i + 1;
    //  (1, 1)    | (1, 1)
    i = temp;
    // LOCK ENDS HERE
    //
    // effectively
    // i = i + 1  | i = i + 1
    // i = t + 1  | i = t + 1
    // i = 0 + 1  | i = 0 + 1

    //  (0, 0)    | (0, 0)
    //  (0, 0)    | (-1, 0)
    //  (0, 0)    | (-1, -1)
    temp = i;
    temp = i - 1;
    i = temp;
  }
}

int main(int argc, char *argv[]) {

  inc();
  inc();

  return 0;
}

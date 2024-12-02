#include <stdio.h>
#include <stdlib.h>
#include "derp/include/ddebug.h"
#include "derp/include/derp.h"
#include "derp/include/dstring.h"

DECL_VEC_TYPE(String*, String);

VecString *split_lines(File * f) {
  VecString *sv = new_vec_String(8);

  for (
  char * c = f->data.cptr; c-f->data.cptr != f->data.len; c++) {
    char * start = c;
    while( *c != '\n' ) {
      c++;
    }
    size len = (size)(c-start);
    String *s = str_new(len);
    s->cptr = start;
    vec_push_String(sv, s);
  }

  return sv;
}

// for each line
// go over each number, parse into ints
// compare each number with the last, applying the following rules:
// if n < x <= n+3 || n > x >= n-3 then the line is safe
// if not, then the line is NOT safe
// count up all the safe lines and output count

/*


  for each line:
    go over each number, parse into ints
    store current number and last number, compare both at end of number parsing
    if they fail, increment errors, abort current line and move on to the next one
*/


int
main(void) {
  #ifdef FINAL
    str path = "./input.txt";
  #else
    str path = "./test.txt";
  #endif
  File* f = read_file(path);
  VecString *lines = split_lines(f);

  int errs = 0; // number of errors found
  for( int i = 0; i < lines->current; i++ ) {

    String * line = lines->data[i];
    if(lines->data[i]) printf("[%i]: %.*s\n", i, line->len, line->cptr);
    int n = 0;
    int last_n = 0;
    int idiff = 0;

    for (int i = 0; i <= line->len; i++) {
      if (line->cptr[i] == ' ' || line->cptr[i] == '\n') { 
        printf("n: %i, c: %c\n", n, line->cptr[i]);
        // printf("n: %i, last_n: %i\n", n, last_n);
        // if this is the first number, just skip; we have nothing to compare against
        if (last_n == 0) {
          last_n = n;
          n = 0;
          continue; 
        }

        int diff = n - last_n;
        if (idiff == 0) { // first two numbers
          idiff = diff;
        } 
        else {
          if ((idiff & (1 << (32 - 1))) != 
              (diff & (1 << (32-1)))) {
            errs++;
            log_error("sign mismatch! n: %i, ln: %i, diff: %i, idiff: %i", n, last_n, diff, idiff);
            break;
          } 
        }

        if (diff < 0) { 
          diff *= -1;
        }
        if (diff >= 1 && diff <= 3) {
          // log_info("safe! n: %i, ln: %i, diff: %i", n, last_n, diff);
          last_n = n;
          n = 0;
          continue;
        } else {
          log_error("unsafe! n: %i, ln: %i, diff: %i", n, last_n, diff);
          errs++;
          break;
        }
      }

      // parse number
      if (line->cptr[i] >= '0' || line->cptr[i] <= '9') {
        if (n == 0) n = line->cptr[i] - '0';
        else n = (n * 10) + (line->cptr[i] - '0');
      }
    }
  }
  printf("ans: %i\n", (int) lines->current - errs);

  free(f);
  free(lines);
}

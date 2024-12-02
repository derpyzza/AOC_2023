#include <stdio.h>
#include <stdlib.h>
#include "derp/include/ddebug.h"
#include "derp/include/derp.h"
#include "derp/include/dstring.h"

DECL_VEC_TYPE(String*, String)

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
    int idiff = 6440055; // magick number, initial difference, just to store whether or not the trend line should be positive or negative

    bool had_err = false;

    for (int i = 0; i <= line->len; i++) {

      if (line->cptr[i] == ' ' || line->cptr[i] == '\n') { 
        // if this is the first number, just skip; we have nothing to compare against
        if (last_n == 0) {
          last_n = n;
          n = 0;
          continue; 
        }

        int diff = n - last_n;
        log_debug("n: %i, ln: %i, diff: %i", n, last_n, diff);

        if (idiff == 6440055) { // first two numbers
          idiff = diff;
        } 

        // same number
        if (diff == 0) {
          if (had_err) {
            log_error("same number! c: %c, n: %i, ln: %i, diff: %i, idiff: %i", line->cptr[i], n, last_n, diff, idiff);
            errs++;
            break;
          }
          if (!had_err) {
            log_info("had error, ln: %i, n: %i, diff: %i", last_n, n, diff);
            had_err = true;
            n = 0;
            continue;
          }
        }

        // check if the difference has changed signs
        if ( ( idiff > 0 ) != ( diff > 0 ) ) {
          if (had_err) {
            log_error("sign mismatch! c: %c, n: %i, ln: %i, diff: %i, idiff: %i", line->cptr[i], n, last_n, diff, idiff);
            errs++;
            break;
          }

          if (!had_err) {
            log_info("had error, ln: %i, n: %i, diff: %i", last_n, n, diff);
            had_err = true;
            n = 0;
            continue;
          }
        } 

        if (diff < 0) { 
          diff *= -1;
        }

        if (diff >= 1 && diff <= 3) {
          last_n = n;
          n = 0;
          continue;
        } else {
          log_warn("bug! n: %i, ln: %i, diff: %i", n, last_n, diff);
          if (had_err) {
              log_error("unsafe! n: %i, ln: %i, diff: %i, idiff: %i", n, last_n, diff, idiff);
            errs++;
            break;
          }
          if (!had_err) {
            log_info("had error, ln: %i, n: %i, diff: %i", last_n, n, diff);
            had_err = true;
            n = 0;
            continue;
          }
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

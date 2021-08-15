#include <time.h>

size_t _strftime(char *restrict s, size_t max, const char *restrict format, const struct tm *restrict tm) {
    return strftime(s, max, format, tm);
}
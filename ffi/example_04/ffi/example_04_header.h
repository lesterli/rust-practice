typedef struct c_tuple {
  unsigned int integer;
  bool boolean;
} c_tuple;

unsigned int fibonacci(unsigned int index);

unsigned int count_char(const char *s);

struct c_tuple handle_tuple(struct c_tuple tup);

int sum_of_even(const int *ptr, size_t len);

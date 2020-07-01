#include<stdio.h>

typedef void (*SumSquareCB)(int result, void *user_data);

void sum_square_cb(int a, int b, SumSquareCB cb, void *user_data) {
	int result = a*a + b*b;
	cb(result, user_data);
}
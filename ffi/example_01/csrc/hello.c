#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include <stdlib.h>
#include "example_01.h"   

int main(void) {
  // basic string - char array
  char hello1[6] = {'H', 'e', 'l', 'l', 'o', '\0'};
  printf("C hello 1: %s\n", hello1);
  char hello2[6] = "Hello";
  printf("C hello 2: %s\n", hello2);
  if (strcmp(hello1, hello2) ==0)
  {
    printf("hello 1 and 2 are equal\n");
  } else
  {
    printf("hello 1 and 2 are different\n");
  }
  
  // basic string - char pointer
  // char *str;
  // str = "hello";  // Stored in read only part of data segment
  // *(str+1) = 'i'; // Segmentation fault error:  trying to modify read only memory

  char hello_s[] = "hello"; // Stored in stack segment
  *hello_s = 'H';         // No problem: String is now Hello
  printf("new string in stack is %s\n", hello_s);
  
  int size = 6;
  char *hello_h = (char *)malloc(sizeof(char)*size); // Stored in heap segment
  *(hello_h+0) = 'h';  
  *(hello_h+1) = 'e';   
  *(hello_h+2) = 'l';
  *(hello_h+3) = 'l'; 
  *(hello_h+4) = 'o'; 
  *(hello_h+5) = '\0';       
  *(hello_h+0) = 'H';  // No problem: String is now Hello
  printf("new string in heap is: %s\n", hello_h);

  // C generate strings
  char *c_hello = (char *)malloc(sizeof(char)*size); // Stored in heap segment
  *(c_hello+0) = 'H';  
  *(c_hello+1) = 'e';   
  *(c_hello+2) = 'l';
  *(c_hello+3) = 'l'; 
  *(c_hello+4) = 'o'; 
  *(c_hello+5) = '\0';       
  printf("C side generate: %s\n", c_hello);

  print_str(c_hello);

  c_hello = change_str(c_hello);
  printf("C side result: %s\n", c_hello);
  
  free(c_hello);
  
  char *c_ping; 
  c_ping = generate_str();
  printf("C side print: %s\n", c_ping);
  free_str(c_ping);
}
#include<stdio.h>

void into_rust();

void into_c() {
  puts("hello from C");
  into_rust();
}
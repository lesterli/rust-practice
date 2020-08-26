#include <stdio.h>
#include <string.h>
#include "example_03.h"

int main(void) {
    Student *c_ptr = student_alice();
    printf("Student Num: %d\t Total: %d\t Name: %s\t\n", c_ptr->num, c_ptr->total, c_ptr->name);
    student_free(c_ptr);
    

    Student *stu = student_new();
    printf("Before fill data: Student Num: %d\t Total: %d\t Name: %s\t Scores: %.1f\t%.1f\t%.1f\n",
    stu->num, stu->total, stu->name, 
    stu->scores[0], stu->scores[1], stu->scores[2]);
    stu->num = 2;
    stu->total = 212;
    strcpy(stu->name, "Bob");
    stu->scores[0] = 60.6;
    stu->scores[1] = 70.7;
    stu->scores[2] = 80.8;
    printf("After fill data: Student Num: %d\t Total: %d\t Name: %s\t Scores: %.1f\t%.1f\t%.1f\n",
        stu->num, stu->total, stu->name, 
        stu->scores[0], stu->scores[1], stu->scores[2]);
    student_free(stu);

    return 0;
}




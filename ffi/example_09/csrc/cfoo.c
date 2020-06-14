#include <stdlib.h>
#include <stdio.h>
#include <string.h>

typedef struct Student
{
    int num;
    int total;
    char name[20];
    float scores[3];
} Student;

Student* fill_data(Student *stu)
{
    stu->num = 2;
    stu->total = 100;
    strcpy(stu->name, "Bob");
    stu->scores[0] = 60.6;
    stu->scores[1] = 70.7;
    stu->scores[2] = 80.8;

    printf("C side print: %d %s %d %.2f %.2f %.2f\n",
           stu->num, stu->name, stu->total, stu->scores[0], stu->scores[1], stu->scores[2]);
    return stu;
}

void print_students(Student *stu)
{
    printf("C side print: %d %s %d %.2f %.2f %.2f\n",
            stu->num,
            stu->name,
            stu->total,
            stu->scores[0],
            stu->scores[1],
            stu->scores[2]);
    
}
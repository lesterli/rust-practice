#include <stdlib.h>
#include <stdio.h>

typedef struct Student
{
    int num;
    int total;
    char name[20];
    float scores[3];
} Student;

// Student *create_students(int n)
// {
//     if (n <= 0)
//         return NULL;

//     Student *stu = NULL;
//     stu = (Student *)malloc(sizeof(Student) * n);

//     return stu;
// }

// void release_students(Student *stu)
// {
//     if (stu != NULL)
//         free(stu);
// }

void print_students(Student *stu, int n)
{
    int i;
    for (i = 0; i < n; i++)
    {
        printf("C side print: %d %s %d %.2f %.2f %.2f\n",
               stu[i].num,
               stu[i].name,
               stu[i].total,
               stu[i].scores[0],
               stu[i].scores[1],
               stu[i].scores[2]);
    }
}
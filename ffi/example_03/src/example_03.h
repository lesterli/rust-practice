typedef struct Student
{
    int num;
    int total;
    char name[20];
    float scores[3];
} Student;

Student *student_new();
Student *student_alice();
void student_free(Student *p_stu);

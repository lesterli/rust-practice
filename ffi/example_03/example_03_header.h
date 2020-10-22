typedef enum {
  boy,
  girl,
} capi_gender;

typedef struct {
  int num;
  int total;
  char name[20];
  float scores[3];
  capi_gender gender;
} capi_student;

capi_student *student_new(void);

capi_student *student_alice(void);

void student_free(capi_student *p_stu);

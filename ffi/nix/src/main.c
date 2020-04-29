#include <signal.h>
#include <unistd.h>

int main(void)
{
    pid_t child = fork();
    if (child)
    {
        sleep(5);
        kill(child, SIGKILL);
    }
    else
    {
        for (;;)
        // 循环直到被 kill 掉
        ;
    }

    return 0;
}
#include <pthread.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/_pthread/_pthread_mutex_t.h>
#include <sys/signal.h>
#include <unistd.h>

pthread_mutex_t mutex = PTHREAD_MUTEX_INITIALIZER;
sigset_t set;

void *handler(void *arg) {
    pthread_detach(pthread_self());

    int sig;
    for (;;) {
        if (sigwait(&set, &sig) != 0) {
            perror("sigwait");
            exit(1);
        }

        printf("received signal: %d\n", sig);
        pthread_mutex_lock(&mutex);

        printf("[signal handler] sleep start\n");
        sleep(3);
        printf("[signal handler] sleep end\n");

        pthread_mutex_unlock(&mutex);
    }

    return NULL;
}

void *worker(void * arg) {
    for (int i = 0; i < 10; i++) {
        pthread_mutex_lock(&mutex);

        printf("[worker] sleep start\n");
        sleep(1);
        printf("[worker] sleep end\n");
        pthread_mutex_unlock(&mutex);
        sleep(1);
    }

    return NULL;
}

int main(void) {
    pid_t pid = getpid();
    printf("pid: %d\n", pid);

    sigemptyset(&set);
    sigaddset(&set, SIGUSR1);
    if (pthread_sigmask(SIG_BLOCK, &set, NULL) != 0) {
        perror("pthread_sigmask");
        return 1;
    }

    pthread_t th, wth;
    pthread_create(&th, NULL, handler, NULL);
    pthread_create(&wth, NULL, worker, NULL);

    pthread_join(wth, NULL);

    return 0;
}
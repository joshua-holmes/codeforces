#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main()
{
    char num_of_tests_str[1024] = "";
    fgets(num_of_tests_str, 1024, stdin);
    int num_of_tests = atoi(num_of_tests_str);

    for (int _ = 0; _ < num_of_tests; _++)
    {
        char _unused[1024];
        fgets(_unused, 1024, stdin);

        int likes = 0;
        char reviewers[10240];
        fgets(reviewers, 10240, stdin);
        for (int i = 0; i < strlen(reviewers); i++)
        {
            char c = reviewers[i];
            if (c == '1' || c == '3') {
                likes++;
            }
        }

        printf("%i\n", likes);
    }
}

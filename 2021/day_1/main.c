#include <stdio.h>
int main() {
    int pairs = 0, curr, prev = 10000;
    FILE *file = fopen("input.txt", "r");
    char line[6];
    for (int i = 0; i < 2000; i++) {
        fgets(line, 6, file);
        sscanf(line, "%4d", &curr);
        if (prev < curr) {
            pairs++;
        }
        prev = curr;
    }
    fclose(file);
    printf("%d\n", pairs);
    return 0;
}
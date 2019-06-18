#include <stdio.h>
#include "keyboard.h"

int main() {
    InitKeyboard();

    printf("\nEnter: ");

    for (;;) {
        int r = GetCharacter();
        printf("%c", r);

        if (r == 'q') {
            break;
        }
    }

    CloseKeyboard();

    return 0;
}

void InitKeyboard() {
    printf("InitKeyboard...\n");
}

int GetCharacter() {
    return getchar();
}

void CloseKeyboard() {
    printf("CloseKeyboard...\n");
}
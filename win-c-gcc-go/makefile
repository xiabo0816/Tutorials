build:
	rm -f keyboard.o
	gcc -c keyboard.c -o keyboard.o
	gcc -shared -o libkeyboard.dll keyboard.o
	rm -f keyboard.o
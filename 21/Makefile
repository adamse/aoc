run: d2.exe
	bash -c './$<; echo exit: $$?'

%.exe: %.o
	ld -o $@ $^

%.o: %.s %.txt
	nasm -f elf64 -o $@ $<

clean:
	$(RM) *.o d1

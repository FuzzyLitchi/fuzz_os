default: build

kernel := target/kernel.bin
iso := target/os.iso

linker_script := src/asm/linker.ld
grub_cfg := src/asm/grub.cfg
assembly_source_files := $(wildcard src/asm/*.asm)
assembly_object_files := $(patsubst src/asm/%.asm, target/%.o, $(assembly_source_files))

.PHONY: build clean run iso

build: $(kernel)

clean:
	@rm -r target

run: $(iso)
	@qemu-system-x86_64 -cdrom $(iso)

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p target/isofiles/boot/grub
	@cp $(kernel) target/isofiles/boot/kernel.bin
	@cp $(grub_cfg) target/isofiles/boot/grub
	@grub-mkrescue -o $(iso) target/isofiles 2> /dev/null
	@rm -r target/isofiles

$(kernel): $(assembly_object_files) $(linker_script)
	@ld -n -T $(linker_script) -o $(kernel) $(assembly_object_files)

# compile assembly files
target/%.o: src/asm/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -f elf64 $< -o $@

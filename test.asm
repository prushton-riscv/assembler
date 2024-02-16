; init numbers
	addi $t0 $c0 4
	addi $t1 $c0 4
; do multiplication and check if done
loop:
	add $t3 $t2 $t1
	subi $t1 $t1 1
	beq $t1 $c0 end
; if done sw
end:
	sw $c0[$c1] $t3

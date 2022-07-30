	.global _opendir, _readdir, _closedir	
	
	;; closedir takes file descriptor as argument (appears in A)
_closedir:
	TAX
	LDA #$16
	STA $D640
	NOP
	LDX #$00
	RTS
	
	;; Opendir takes no arguments and returns File descriptor in A
_opendir:
	LDA #$12
	STA $D640
	NOP
	LDX #$00
	RTS

	;; readdir takes the file descriptor returned by opendir as argument
	;; and gets a pointer to a MEGA65 DOS dirent structure.
	;; Again, the annoyance of the MEGA65 Hypervisor requiring a page aligned
	;; transfer area is a nuisance here. We will use $0400-$04FF, and then
	;; copy the result into a regular C dirent structure
	;;
	;; d_ino = first cluster of file
	;; d_off = offset of directory entry in cluster
	;; d_reclen = size of the dirent on disk (32 bytes)
	;; d_type = file/directory type
	;; d_name = name of file
_readdir:

	pha
	
	;; First, clear out the dirent
	ldx #0
	txa
l1:	sta _readdir_dirent,x	
	dex
	bne l1

	;; Third, call the hypervisor trap
	;; File descriptor gets passed in in X.
	;; Result gets written to transfer area we setup at $0400
	plx
	ldy #>$0400 		; write dirent to $0400 
	lda #$14
	STA $D640
	NOP

	bcs readDirSuccess

	;;  Return end of directory
	lda #$00
	ldx #$00
	RTS

readDirSuccess:
	
	;;  Copy file name
	ldx #$3f
l2:	lda $0400,x
	sta _readdir_dirent+4+2+4+2,x
	dex
	bpl l2
	;; make sure it is null terminated
	ldx $0400+64
	lda #$00
	sta _readdir_dirent+4+2+4+2,x

	;; Inode = cluster from offset 64+1+12 = 77
	ldx #$03
l3:	lda $0477,x
	sta _readdir_dirent+0,x
	dex
	bpl l3

	;; d_off stays zero as it is not meaningful here
	
	;; d_reclen we preload with the length of the file (this saves calling stat() on the MEGA65)
	ldx #3
l4:	lda $0400+64+1+12+4,x
	sta _readdir_dirent+4+2,x
	dex
	bpl l4

	;; File type and attributes
	;; XXX - We should translate these to C style meanings
	lda $0400+64+1+12+4+4
	sta _readdir_dirent+4+2+4

	;; Return address of dirent structure
	lda #<_readdir_dirent
	ldx #>_readdir_dirent
	
	RTS

_readdir_dirent:
	.short 0,0   		; d_ino
	.short 0		; d_off
	.short 0,0		; d_reclen
	.short 0		; d_type
	.space 256,0x0

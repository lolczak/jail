//typedef unsigned int          dev_t;
// typedef unsigned long         ino_t;
// typedef unsigned int         mode_t;
// typedef   signed int          pid_t;
// typedef unsigned int          uid_t;
// typedef unsigned int          gid_t;
// typedef unsigned long       nlink_t;
// typedef   signed long         off_t;
// typedef   signed long     blksize_t;
// typedef   signed long      blkcnt_t;
// typedef   signed long        time_t;

//typedef unsigned char       uint8_t;
// typedef   signed char        int8_t;
// typedef unsigned short     uint16_t;
// typedef   signed short      int16_t;
// typedef unsigned int       uint32_t;
// typedef   signed int        int32_t;
// typedef unsigned long long uint64_t;
// typedef   signed long long  int64_t;
// typedef unsigned long        size_t;
// typedef   signed long       ssize_t;
// typedef unsigned long     uintptr_t;
// typedef   signed long      intptr_t;
// typedef   signed long     ptrdiff_t

pub type pid_t = i32;
pub type uid_t = u32;
pub type gid_t = u32;

#[repr(C)]
pub struct IoVec {
    pub base: usize,
    pub len: u64
}
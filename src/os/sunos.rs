#![allow(non_camel_case_types)]

use super::super::ffi;
use libc::{c_int,c_uint,c_char};

pub type cc_t = c_char;
pub type speed_t = c_int;
pub type tcflag_t = c_uint;

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
#[repr(C)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_cc: [cc_t; NCCS],
}

pub const NCCS: usize = 19;

// control characters (c_cc)
pub const VINTR:    usize = 0;
pub const VQUIT:    usize = 1;
pub const VERASE:   usize = 2;
pub const VKILL:    usize = 3;
pub const VEOF:     usize = 4;
pub const VMIN:     usize = 4; // NOTE: Overlaps with VEOF!
pub const VEOL:     usize = 5;
pub const VTIME:    usize = 5; // NOTE: Overlaps with VEOL!
pub const VEOL2:    usize = 6;
pub const VSWTCH:   usize = 7;
pub const VSTART:   usize = 8;
pub const VSTOP:    usize = 9;
pub const VSUSP:    usize = 10;
pub const VDSUSP:   usize = 11;
pub const VREPRINT: usize = 12;
pub const VDISCARD: usize = 13;
pub const VWERASE:  usize = 14;
pub const VLNEXT:   usize = 15;
pub const VSTATUS:  usize = 16;
pub const VERASE2:  usize = 17;

// input modes (c_iflag)
pub const IGNBRK:  tcflag_t = 0o0000001;
pub const BRKINT:  tcflag_t = 0o0000002;
pub const IGNPAR:  tcflag_t = 0o0000004;
pub const PARMRK:  tcflag_t = 0o0000010;
pub const INPCK:   tcflag_t = 0o0000020;
pub const ISTRIP:  tcflag_t = 0o0000040;
pub const INLCR:   tcflag_t = 0o0000100;
pub const IGNCR:   tcflag_t = 0o0000200;
pub const ICRNL:   tcflag_t = 0o0000400;
pub const IUCLC:   tcflag_t = 0o0001000;
pub const IXON:    tcflag_t = 0o0002000;
pub const IXANY:   tcflag_t = 0o0004000;
pub const IXOFF:   tcflag_t = 0o0010000;
pub const IMAXBEL: tcflag_t = 0o0020000;
pub const DOSMODE: tcflag_t = 0o0100000;

// output modes (c_oflag)
pub const OPOST:   tcflag_t = 0o0000001;
pub const OLCUC:   tcflag_t = 0o0000002;
pub const ONLCR:   tcflag_t = 0o0000004;
pub const OCRNL:   tcflag_t = 0o0000010;
pub const ONOCR:   tcflag_t = 0o0000020;
pub const ONLRET:  tcflag_t = 0o0000040;
pub const OFILL:   tcflag_t = 0o0000100;
pub const OFDEL:   tcflag_t = 0o0000200;
pub const NLDLY:   tcflag_t = 0o0000400;
pub const NL0:     tcflag_t = 0o0000000;
pub const NL1:     tcflag_t = 0o0000400;
pub const CRDLY:   tcflag_t = 0o0003000;
pub const CR0:     tcflag_t = 0o0000000;
pub const CR1:     tcflag_t = 0o0001000;
pub const CR2:     tcflag_t = 0o0002000;
pub const CR3:     tcflag_t = 0o0003000;
pub const TABDLY:  tcflag_t = 0o0014000;
pub const TAB0:    tcflag_t = 0o0000000;
pub const TAB1:    tcflag_t = 0o0004000;
pub const TAB2:    tcflag_t = 0o0010000;
pub const TAB3:    tcflag_t = 0o0014000;
pub const XTABS:   tcflag_t = 0o0014000;
pub const BSDLY:   tcflag_t = 0o0020000;
pub const BS0:     tcflag_t = 0o0000000;
pub const BS1:     tcflag_t = 0o0020000;
pub const VTDLY:   tcflag_t = 0o0040000;
pub const VT0:     tcflag_t = 0o0000000;
pub const VT1:     tcflag_t = 0o0040000;
pub const FFDLY:   tcflag_t = 0o0100000;
pub const FF0:     tcflag_t = 0o0000000;
pub const FF1:     tcflag_t = 0o0100000;
pub const PAGEOUT: tcflag_t = 0o0200000;
pub const WRAP:    tcflag_t = 0o0400000;

// control modes (c_cflag)
pub const CBAUD:     tcflag_t = 0o000000000017;
pub const CSIZE:     tcflag_t = 0o000000000060;
pub const CS5:       tcflag_t = 0o000000000000;
pub const CS6:       tcflag_t = 0o000000000020;
pub const CS7:       tcflag_t = 0o000000000040;
pub const CS8:       tcflag_t = 0o000000000060;
pub const CSTOPB:    tcflag_t = 0o000000000100;
pub const CREAD:     tcflag_t = 0o000000000200;
pub const PARENB:    tcflag_t = 0o000000000400;
pub const PARODD:    tcflag_t = 0o000000001000;
pub const HUPCL:     tcflag_t = 0o000000002000;
pub const CLOCAL:    tcflag_t = 0o000000004000;
pub const RCV1EN:    tcflag_t = 0o000000010000;
pub const XMT1EN:    tcflag_t = 0o000000020000;
pub const LOBLK:     tcflag_t = 0o000000040000;
pub const XCLUDE:    tcflag_t = 0o000000100000;
pub const CRTSXOFF:  tcflag_t = 0o010000000000;
pub const CRTSCTS:   tcflag_t = 0o020000000000;
pub const CIBAUD:    tcflag_t = 0o000003600000;
pub const PAREXT:    tcflag_t = 0o000004000000;
pub const CBAUDEXT:  tcflag_t = 0o000010000000;
pub const CIBAUDEXT: tcflag_t = 0o000020000000;

// line discipline 0 modes (c_lflag)
pub const ISIG:    tcflag_t = 0o0000001;
pub const ICANON:  tcflag_t = 0o0000002;
pub const XCASE:   tcflag_t = 0o0000004;
pub const ECHO:    tcflag_t = 0o0000010;
pub const ECHOE:   tcflag_t = 0o0000020;
pub const ECHOK:   tcflag_t = 0o0000040;
pub const ECHONL:  tcflag_t = 0o0000100;
pub const NOFLSH:  tcflag_t = 0o0000200;
pub const TOSTOP:  tcflag_t = 0o0000400;
pub const ECHOCTL: tcflag_t = 0o0001000;
pub const ECHOPRT: tcflag_t = 0o0002000;
pub const ECHOKE:  tcflag_t = 0o0004000;
pub const DEFECHO: tcflag_t = 0o0010000;
pub const FLUSHO:  tcflag_t = 0o0020000;
pub const PENDIN:  tcflag_t = 0o0040000;
pub const IEXTEN:  tcflag_t = 0o0100000;

// baud rates
pub const B0:      speed_t = 0;
pub const B50:     speed_t = 1;
pub const B75:     speed_t = 2;
pub const B110:    speed_t = 3;
pub const B134:    speed_t = 4;
pub const B150:    speed_t = 5;
pub const B200:    speed_t = 6;
pub const B300:    speed_t = 7;
pub const B600:    speed_t = 8;
pub const B1200:   speed_t = 9;
pub const B1800:   speed_t = 10;
pub const B2400:   speed_t = 11;
pub const B4800:   speed_t = 12;
pub const B9600:   speed_t = 13;
pub const B19200:  speed_t = 14;
pub const EXTA:    speed_t = B19200;
pub const B38400:  speed_t = 15;
pub const EXTB:    speed_t = B38400;
pub const B57600:  speed_t = 16;
pub const B76800:  speed_t = 17;
pub const B115200: speed_t = 18;
pub const B153600: speed_t = 19;
pub const B230400: speed_t = 20;
pub const B307200: speed_t = 21;
pub const B460800: speed_t = 22;
pub const B921600: speed_t = 23;


// tcflow()
pub const TCOOFF: c_int = 0;
pub const TCOON:  c_int = 1;
pub const TCIOFF: c_int = 2;
pub const TCION:  c_int = 3;

// tcflush()
pub const TCIFLUSH:  c_int = 0;
pub const TCOFLUSH:  c_int = 1;
pub const TCIOFLUSH: c_int = 2;

// tcsetattr()
pub const TCSANOW:   c_int = 0x540e;
pub const TCSADRAIN: c_int = 0x540f;
pub const TCSAFLUSH: c_int = 0x5410;

pub unsafe fn cfmakeraw(termios: *mut termios) {
    (*termios).c_iflag &= !(IMAXBEL
        | IGNBRK
        | BRKINT
        | PARMRK
        | ISTRIP
        | INLCR
        | IGNCR
        | ICRNL
        | IXON);
    (*termios).c_oflag &= !OPOST;
    (*termios).c_lflag &= !(ECHO | ECHONL | ICANON | ISIG | IEXTEN);
    (*termios).c_cflag &= !(CSIZE | PARENB);
    (*termios).c_cflag |= CS8;

    // By default, most software expects a pending read to block until at
    // least one byte becomes available.  As per termio(7I), this requires
    // setting the MIN and TIME parameters appropriately.
    //
    // As a somewhat unfortunate artefact of history, the MIN and TIME slots
    // in the control character array overlap with the EOF and EOL slots used
    // for canonical mode processing.  Because the EOF character needs to be
    // the ASCII EOT value (aka Control-D), it has the byte value 4.  When
    // switching to raw mode, this is interpreted as a MIN value of 4; i.e.,
    // reads will block until at least four bytes have been input.
    //
    // Other platforms with a distinct MIN slot like Linux and FreeBSD appear
    // to default to a MIN value of 1, so we'll force that value here:
    (*termios).c_cc[VMIN] = 1;
    (*termios).c_cc[VTIME] = 0;
}

pub unsafe fn cfsetspeed(
    termios: *mut termios,
    speed: speed_t,
) -> c_int {
    // Neither of these functions on illumos or Solaris actually ever
    // return an error
    ffi::cfsetispeed(termios, speed);
    ffi::cfsetospeed(termios, speed);
    0
}

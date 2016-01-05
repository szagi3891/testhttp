use std::io::{self, Write};

// https://en.wikipedia.org/wiki/Syslog#Severity_level
/*
enum Level {
    Emerg = 0,
    Alert,
    Crit,
    Error,
    Warn,
    Notice,
    Info,
    Debug
}*/

// http://stackoverflow.com/questions/27588416/how-to-send-output-to-stderr

/*
// Adds prefix and suffix to text to make it red (0;31)
macro_rules! text_red {
    ($fmt:expr) => { concat!("\x1B[0;31m", $fmt, "\x1B[m") };
}

// Adds prefix and suffix to text to make it bold and red (1;31)
macro_rules! text_bold_red {
    ($fmt:expr) => { concat!("\x1B[1;31m", $fmt, "\x1B[m") };
}

macro_rules! log_crit {
    ($fmt:expr) => ({
        use std::io::Write;
        match writeln!(&mut ::std::io::stderr(), text_bold_red!(concat!("CRITICAL: ", $fmt))) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    });

    ($fmt:expr, $($arg:tt)*) => ({
        use std::io::Write;
        match writeln!(&mut ::std::io::stderr(), text_bold_red!(concat!("CRITICAL: ", $fmt)), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    });
}

macro_rules! log_error {
    ($fmt:expr) => ({
        use std::io::Write;
        match writeln!(&mut ::std::io::stderr(), text_red!(concat!("ERROR: ", $fmt))) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    });

    ($fmt:expr, $($arg:tt)*) => ({
        use std::io::Write;
        match writeln!(&mut ::std::io::stderr(), text_red!(concat!("ERROR: ", $fmt)), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    });
}
*/
//macro_rules! warn {
//}

//macro_rules! info {
//}



pub fn error(message: String) {

    let stderr = io::stderr();
    let mut handle = stderr.lock();

    //handle.write(b"hello world").unwrap();
    handle.write(format!("\x1B[1;31m{}\x1B[m\n", message).as_bytes()).unwrap();

    //show(message) - trzeba na czerwono wyświeltić ten komuniakt - ale tylko w przypadku
    //  wyswietlania na ekran, bo do pliku albo do sysloga to be zkoloryzowania
}


pub fn info(message: String) {
    println!("{}", format!("\x1B[32m{}\x1B[39m", message));
}



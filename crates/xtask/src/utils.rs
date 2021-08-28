/// Normalizes the line endings of the given string
///
/// Replaces all instances of CRLF (`\r\n`) with LF (`\n`)
pub fn normalize(string: &mut String) {
    unsafe {
        let vec = string.as_mut_vec();
        let ptr = vec.as_mut_ptr();

        let mut write = vec.as_mut_ptr();
        let mut read = vec.as_ptr();
        let end = read.add(vec.len());

        while read != end {
            if *read == b'\r' {
                let newline = read.add(1);
                if newline == end {
                    break;
                }

                if *newline == b'\n' {
                    read = newline;
                }
            }

            *write = *read;
            write = write.add(1);
            read = read.add(1);
        }

        vec.set_len(write as usize - ptr as usize);
    }
}

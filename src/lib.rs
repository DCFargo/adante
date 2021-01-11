#[cfg(test)]
mod tests;

pub trait ArgumentType {
    fn from_str<E: Error>(key: &str) -> Result<Self, E>
        where Self: std::marker::Sized;
    fn ret_err<E: Error>(&self) -> E;
}

pub trait Error {
    fn handle(&self);
    fn as_str(&self) -> &str;
}

// TODO: Find out where FlagType must be
// PartialEq in par browser,
// implement here or ArgumentType
pub struct Flag<T: ArgumentType> {
    pub key: T,
    // Thought making String generic here
    // may have been overdoing it a bit.
    // TODO: Make final decision on this
    pub value: Option<String>,
}

pub struct Arguments<F: ArgumentType, A: ArgumentType> {
    pub flags: Vec<Flag<F>>,
    pub actions: Vec<A>,
}

impl<F: ArgumentType, A: ArgumentType> Arguments<F, A> {
    fn new() -> Self {
        Arguments {
            flags: Vec::new(),
            actions: Vec::new(),
        }
    }
    fn parse<E: Error>(env_args: Vec<&str>) -> Result<Arguments<F, A>, E> {
        let mut args = Arguments::new();
        let mut eq_pos: usize = 0;
        for arg in env_args.iter() {
            // Detect if argument is option or action:
            if &arg[0..1] == "-" {
                // Assume flag, find seperator:
                for (i, &byte) in arg.as_bytes().iter().enumerate() {
                    if byte == b'=' {
                        eq_pos = i;
                    }
                }
                // Assume no value if no =:
                if eq_pos == 0 {
                    args.flags.push(Flag {
                        key: match F::from_str(arg) {
                            Ok(v) => v,
                            Err(e) => return Err(e),
                        },
                        value: None
                    })
                } else {
                    let key = &arg[0..eq_pos];
                    let val = &arg[(eq_pos + 1)..];
                    args.flags.push(Flag {
                        key: match F::from_str(arg) {
                            Ok(v) => v,
                            Err(e) => return Err(e),
                        },
                        // TODO: make value field a &str by default
                        value: Some(val.to_string())
                    })
                }
            } else {
                args.actions.push(match A::from_str(arg) {
                    Ok(v) => v,
                    Err(e) => return Err(e),
                })
            }
        }

        Ok(args)
    }
}

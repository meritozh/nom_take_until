use nom::{
    bytes::complete::tag,
    character::complete::line_ending,
    error::{context, ErrorKind},
    sequence::{preceded, terminated, tuple},
    Err, IResult,
};

fn front_matter<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    let take_until = move |input: &'a str| loop {
        match input.char_indices().next() {
            Some((nth, '-')) if nth > 1 => {
                // Here raise an error.
                if tuple((tag("---"), line_ending::<&str, _>))(&input[(nth - 1)..]).is_ok() {
                    let (i, o) = input.split_at(nth);
                    return Ok((i, o));
                }
            }
            Some(_) => continue,
            None => return Err(Err::Error((input, ErrorKind::TakeTill1))),
        }
    };
    context(
        "front_matter",
        preceded(terminated(tag("---"), line_ending), take_until),
    )(input)
}

fn main() {
    println!("Hello, world!");
}

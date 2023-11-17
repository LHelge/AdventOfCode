use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, multispace0, newline, u64},
    multi::separated_list1,
    IResult,
};

#[derive(Debug, Clone)]
pub struct Valve<'a> {
    pub name: &'a str,
    pub flow_rate: u64,
    pub tunnels: Vec<&'a str>,
}

// Parse a specific valve using nom
fn valve(s: &str) -> IResult<&str, Valve> {
    let (s, _) = multispace0(s)?;
    let (s, _) = tag("Valve ")(s)?;
    let (s, name) = alpha1(s)?;
    let (s, _) = tag(" has flow rate=")(s)?;
    let (s, flow_rate) = u64(s)?;
    let (s, _) = alt((
        tag("; tunnels lead to valves "),
        tag("; tunnel leads to valve "),
    ))(s)?;
    let (s, tunnels) = separated_list1(tag(", "), alpha1)(s)?;

    Ok((
        s,
        Valve {
            name,
            flow_rate,
            tunnels,
        },
    ))
}

// Parse all valves using nom
pub fn parse_valves(s: &str) -> IResult<&str, Vec<Valve>> {
    separated_list1(newline, valve)(s)
}

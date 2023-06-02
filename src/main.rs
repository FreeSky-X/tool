use port_open::get_port_isopen;
fn main() {
    let isopen = get_port_isopen(&[127, 0, 0, 1], &[80, 5037, 22]);
    for res in isopen {
        println!("{:?}", res);
    }
}

use deltav_calc;

fn main() {
    let map = deltav_calc::DeltavMap::get_stock();
    println!("{}", map.calculate_delta_v("Low Kerbin Orbit (80km)", "Low Mun Orbit (14km)").unwrap().unwrap())
}

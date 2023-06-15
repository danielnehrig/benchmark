fn main() {
    let cpu = benchmark::sys::get_cpu_info();
    let rtsc = benchmark::sys::get_rtsc_time();
    let mem = benchmark::sys::read_memory();
    let cpu_usage = benchmark::sys::read_cpu_usage();
    println!("{:#?}", cpu);
    println!("{:#?}", cpu_usage);
    println!("{:#?}", rtsc);
    println!("{:#?}", mem);
}

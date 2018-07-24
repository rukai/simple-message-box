#![windows_subsystem = "windows"] // disable the terminal in windows, like a real gui app

extern crate simple_message_box;

fn main() {
    let message = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer eget lorem eu lectus feugiat facilisis eget non lectus. Nunc sed felis placerat, pulvinar neque id, tristique justo. Aenean ultrices arcu nibh, ut vehicula metus lobortis ac. In efficitur feugiat interdum. Nulla et lorem sit amet augue tincidunt maximus non at dui. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Integer ut lobortis dolor, lobortis commodo lectus. Mauris vulputate, diam quis rutrum mollis, nulla velit suscipit justo, porttitor bibendum massa nulla eu magna. Vivamus non urna sed urna bibendum laoreet vel sed nunc. Mauris gravida leo maximus, mattis elit a, consequat sem. Praesent eget gravida orci, vel venenatis dui. Nulla bibendum vestibulum dui, eget posuere felis feugiat quis.\n\nMaecenas dolor lectus, gravida ut metus non, placerat lobortis ante. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Quisque vitae dui et lacus pellentesque aliquam sit amet in arcu. Aenean imperdiet lectus at dictum facilisis. Sed ultricies elementum ante, quis dapibus odio tempus ac. Integer hendrerit, risus sed congue volutpat, lacus risus iaculis felis, at fringilla orci mauris eget massa. Sed suscipit ante mauris, sodales cursus est auctor et.";
    simple_message_box::create_message_box(message, "Lorem Ipsum");
}
